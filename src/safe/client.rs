use std::{
    ffi::{CStr, CString},
    path::Path,
    ptr::null_mut,
};

use libc::free;

use crate::raw::*;

use super::{error::MpvError, event::Event, node::Node, util::make_rust_string_const};

/// Return the MPV_CLIENT_API_VERSION the mpv source has been compiled with.
pub fn client_api_version() -> u64 {
    unsafe { mpv_client_api_version() }
}

pub struct MpvHandle(*mut mpv_handle);

impl MpvHandle {
    /// Create a new mpv instance and an associated client API handle to control
    /// the mpv instance. This instance is in a pre-initialized state,
    /// and needs to be initialized to be actually used with most other API
    /// functions.
    ///
    /// Some API functions will return MPV_ERROR_UNINITIALIZED in the uninitialized
    /// state. You can call mpv_set_property() (or mpv_set_property_string() and
    /// other variants, and before mpv 0.21.0 mpv_set_option() etc.) to set initial
    /// options. After this, call mpv_initialize() to start the player, and then use
    /// e.g. mpv_command() to start playback of a file.
    ///
    /// The point of separating handle creation and actual initialization is that
    /// you can configure things which can't be changed during runtime.
    ///
    /// Unlike the command line player, this will have initial settings suitable
    /// for embedding in applications. The following settings are different:
    /// - stdin/stdout/stderr and the terminal will never be accessed. This is
    ///   equivalent to setting the --no-terminal option.
    ///   (Technically, this also suppresses C signal handling.)
    /// - No config files will be loaded. This is roughly equivalent to using
    ///   --config=no. Since libmpv 1.15, you can actually re-enable this option,
    ///   which will make libmpv load config files during mpv_initialize(). If you
    ///   do this, you are strongly encouraged to set the \"config-dir\" option too.
    ///   (Otherwise it will load the mpv command line player's config.)
    ///   For example:
    ///      mpv_set_option_string(mpv, \"config-dir\", \"/my/path\"); // set config root
    ///      mpv_set_option_string(mpv, \"config\", \"yes\"); // enable config loading
    ///      (call mpv_initialize() _after_ this)
    /// - Idle mode is enabled, which means the playback core will enter idle mode
    ///   if there are no more files to play on the internal playlist, instead of
    ///   exiting. This is equivalent to the --idle option.
    /// - Disable parts of input handling.
    /// - Most of the different settings can be viewed with the command line player
    ///   by running \"mpv --show-profile=libmpv\".
    ///
    /// All this assumes that API users want a mpv instance that is strictly
    /// isolated from the command line player's configuration, user settings, and
    /// so on. You can re-enable disabled features by setting the appropriate
    /// options.
    ///
    /// The mpv command line parser is not available through this API, but you can
    /// set individual options with mpv_set_property(). Files for playback must be
    /// loaded with mpv_command() or others.
    ///
    /// Note that you should avoid doing concurrent accesses on the uninitialized
    /// client handle. (Whether concurrent access is definitely allowed or not has
    /// yet to be decided.)
    ///
    /// @return a new mpv client API handle. Returns NULL on error. Currently, this
    ///         can happen in the following situations:
    ///         - out of memory
    ///         - LC_NUMERIC is not set to \"C\" (see general remarks)
    pub fn new() -> Option<Self> {
        let ctx = unsafe { mpv_create() };
        if ctx == null_mut() {
            None
        } else {
            Some(Self(ctx))
        }
    }

    /// Return the name of this client handle. Every client has its own unique
    /// name, which is mostly used for user interface purposes.
    ///
    /// @return The client name. The string is read-only and is valid until the
    ///         mpv_handle is destroyed.
    pub fn name(&self) -> String {
        make_rust_string_const(unsafe { mpv_client_name(self.0) }).unwrap()
    }

    /// Return the ID of this client handle. Every client has its own unique ID. This
    /// ID is never reused by the core, even if the mpv_handle at hand gets destroyed
    /// and new handles get allocated.
    ///
    /// IDs are never 0 or negative.
    ///
    /// Some mpv APIs (not necessarily all) accept a name in the form \"@<id>\" in
    /// addition of the proper mpv_client_name(), where \"<id>\" is the ID in decimal
    /// form (e.g. \"@123\"). For example, the \"script-message-to\" command takes the
    /// client name as first argument, but also accepts the client ID formatted in
    /// this manner.
    ///
    /// @return The client ID.
    pub fn id(&self) -> i64 {
        unsafe { mpv_client_id(self.0) }
    }

    /// Initialize an uninitialized mpv instance. If the mpv instance is already
    /// running, an error is returned.
    ///
    /// This function needs to be called to make full use of the client API if the
    /// client API handle was created with mpv_create().
    ///
    /// Only the following options are required to be set _before_ mpv_initialize():
    ///      - options which are only read at initialization time:
    ///        - config
    ///        - config-dir
    ///        - input-conf
    ///        - load-scripts
    ///        - script
    ///        - player-operation-mode
    ///        - input-app-events (macOS)
    ///      - all encoding mode options
    pub fn initialize(&mut self) -> Option<MpvError> {
        let status = unsafe { mpv_initialize(self.0) };
        MpvError::from_mpv_error(status)
    }

    /// Similar to mpv_destroy(), but brings the player and all clients down
    /// as well, and waits until all of them are destroyed. This function blocks. The
    /// advantage over mpv_destroy() is that while mpv_destroy() merely
    /// detaches the client handle from the player, this function quits the player,
    /// waits until all other clients are destroyed (i.e. all mpv_handles are
    /// detached), and also waits for the final termination of the player.
    ///
    /// Since mpv_destroy() is called somewhere on the way, it's not safe to
    /// call other functions concurrently on the same context.
    ///
    /// Since mpv client API version 1.29:
    ///  The first call on any mpv_handle will block until the core is destroyed.
    ///  This means it will wait until other mpv_handle have been destroyed. If you
    ///  want asynchronous destruction, just run the \"quit\" command, and then react
    ///  to the MPV_EVENT_SHUTDOWN event.
    ///  If another mpv_handle already called mpv_terminate_destroy(), this call will
    ///  not actually block. It will destroy the mpv_handle, and exit immediately,
    ///  while other mpv_handles might still be uninitializing.
    ///
    /// Before mpv client API version 1.29:
    ///  If this is called on a mpv_handle that was not created with mpv_create(),
    ///  this function will merely send a quit command and then call
    ///  mpv_destroy(), without waiting for the actual shutdown.
    pub fn terminate(self) {
        unsafe { mpv_terminate_destroy(self.0) };
        std::mem::forget(self);
    }

    /// Create a new client handle connected to the same player core as ctx. This
    /// context has its own event queue, its own mpv_request_event() state, its own
    /// mpv_request_log_messages() state, its own set of observed properties, and
    /// its own state for asynchronous operations. Otherwise, everything is shared.
    ///
    /// This handle should be destroyed with mpv_destroy() if no longer
    /// needed. The core will live as long as there is at least 1 handle referencing
    /// it. Any handle can make the core quit, which will result in every handle
    /// receiving MPV_EVENT_SHUTDOWN.
    ///
    /// This function can not be called before the main handle was initialized with
    /// mpv_initialize(). The new handle is always initialized, unless ctx=NULL was
    /// passed.
    ///
    /// Because API requires name to be const pointer, it has to be static CStr to avoid leaks
    pub fn create_client(&mut self, name: &'static CStr) -> Option<MpvHandle> {
        let ctx = unsafe { mpv_create_client(self.0, name.as_ptr()) };
        if ctx == null_mut() {
            None
        } else {
            Some(Self(ctx))
        }
    }

    /// This is the same as mpv_create_client(), but the created mpv_handle is
    /// treated as a weak reference. If all mpv_handles referencing a core are
    /// weak references, the core is automatically destroyed. (This still goes
    /// through normal uninit of course. Effectively, if the last non-weak mpv_handle
    /// is destroyed, then the weak mpv_handles receive MPV_EVENT_SHUTDOWN and are
    /// asked to terminate as well.)
    ///
    /// Note if you want to use this like refcounting: you have to be aware that
    /// mpv_terminate_destroy() _and_ mpv_destroy() for the last non-weak
    /// mpv_handle will block until all weak mpv_handles are destroyed.
    pub fn create_weak_client(&mut self, name: &'static CStr) -> Option<MpvHandle> {
        let ctx = unsafe { mpv_create_weak_client(self.0, name.as_ptr()) };
        if ctx == null_mut() {
            None
        } else {
            Some(Self(ctx))
        }
    }

    /// Load a config file. This loads and parses the file, and sets every entry in
    /// the config file's default section as if mpv_set_option_string() is called.
    ///
    /// The filename should be an absolute path. If it isn't, the actual path used
    /// is unspecified. (Note: an absolute path starts with '/' on UNIX.) If the
    /// file wasn't found, MPV_ERROR_INVALID_PARAMETER is returned.
    ///
    /// If a fatal error happens when parsing a config file, MPV_ERROR_OPTION_ERROR
    /// is returned. Errors when setting options as well as other types or errors
    /// are ignored (even if options do not exist). You can still try to capture
    /// the resulting error messages with mpv_request_log_messages(). Note that it's
    /// possible that some options were successfully set even if any of these errors
    /// happen.
    ///
    /// @param filename absolute path to the config file on the local filesystem
    /// @return error code
    pub fn load_config_file(&mut self, filename: &Path) -> Option<MpvError> {
        let path = CString::into_raw(CString::new(filename.to_path_buf().to_str()?).unwrap());
        let status = unsafe { mpv_load_config_file(self.0, path as _) };
        let res = MpvError::from_mpv_error(status);
        unsafe { free(path as _) };
        res
    }

    /// Set an option. Note that you can't normally set options during runtime. It
    /// works in uninitialized state (see mpv_create()), and in some cases in at
    /// runtime.
    ///
    /// Using a format other than MPV_FORMAT_NODE is equivalent to constructing a
    /// mpv_node with the given format and data, and passing the mpv_node to this
    /// function.
    ///
    /// Note: this is semi-deprecated. For most purposes, this is not needed anymore.
    ///       Starting with mpv version 0.21.0 (version 1.23) most options can be set
    ///       with mpv_set_property() (and related functions), and even before
    ///       mpv_initialize(). In some obscure corner cases, using this function
    ///       to set options might still be required (see
    ///       \"Inconsistencies between options and properties\" in the manpage). Once
    ///       these are resolved, the option setting functions might be fully
    ///       deprecated.
    ///
    /// @param name Option name. This is the same as on the mpv command line, but
    ///             without the leading \"--\".
    /// @param format see enum mpv_format.
    /// @param[in] data Option value (according to the format).
    /// @return error code
    pub fn set_option(&mut self, name: String, node: Node) -> Option<MpvError> {
        let name = CString::into_raw(CString::new(name.clone()).unwrap());
        let Some(raw) = node.to_mpv_node() else {
            return Some(MpvError::OptionError);
        };

        let data = Box::into_raw(Box::new(raw.u.clone()));
        let status = unsafe { mpv_set_option(self.0, name as _, raw.format, data as *mut _) };
        unsafe {
            free(name as _);
            free(data as _);
        }
        MpvError::from_mpv_error(status)
    }

    /// Send a command to the player. Commands are the same as those used in
    /// input.conf, except that this function takes parameters in a pre-split
    /// form.
    ///
    /// The commands and their parameters are documented in input.rst.
    ///
    /// Does not use OSD and string expansion by default (unlike mpv_command_string()
    /// and input.conf).
    ///
    /// @param[in] args NULL-terminated list of strings. Usually, the first item
    ///                 is the command, and the following items are arguments.
    /// @return error code
    /// This is essentially identical to mpv_command() but it also returns a result.
    ///
    /// Does not use OSD and string expansion by default.
    ///
    /// @param[in] args NULL-terminated list of strings. Usually, the first item
    ///                 is the command, and the following items are arguments.
    /// @param[out] result Optional, pass NULL if unused. If not NULL, and if the
    ///                    function succeeds, this is set to command-specific return
    ///                    data. You must call mpv_free_node_contents() to free it
    ///                    (again, only if the command actually succeeds).
    ///                    Not many commands actually use this at all.
    /// @return error code (the result parameter is not set on error)
    pub fn command(
        &mut self,
        args: Vec<String>,
        require_result: bool,
    ) -> Result<Option<Node>, MpvError> {
        let mut args = args
            .iter()
            .map(|s| CString::into_raw(CString::new(s.clone()).unwrap()))
            .collect::<Vec<*mut i8>>();
        args.push(null_mut());
        let args = args.leak().as_mut_ptr();
        let res = if require_result {
            let result = Box::into_raw(Box::new(mpv_node {
                format: 0,
                u: mpv_node__bindgen_ty_1 { flag: 0 },
            }));

            let status = unsafe { mpv_command_ret(self.0, args as *mut _, result) };

            let res = match MpvError::from_mpv_error(status) {
                Some(err) => Err(err),
                None => Ok(Node::from_mpv_node(unsafe { *result })),
            };

            unsafe { mpv_free_node_contents(result) };
            res
        } else {
            let status = unsafe { mpv_command(self.0, args as *mut _) };

            match MpvError::from_mpv_error(status) {
                Some(err) => Err(err),
                None => Ok(None),
            }
        };
        free_array(args);
        res
    }

    /// Same as mpv_command(), but allows passing structured data in any format.
    /// In particular, calling mpv_command() is exactly like calling
    /// mpv_command_node() with the format set to MPV_FORMAT_NODE_ARRAY, and
    /// every arg passed in order as MPV_FORMAT_STRING.
    ///
    /// Does not use OSD and string expansion by default.
    ///
    /// The args argument can have one of the following formats:
    ///
    /// MPV_FORMAT_NODE_ARRAY:
    ///      Positional arguments. Each entry is an argument using an arbitrary
    ///      format (the format must be compatible to the used command). Usually,
    ///      the first item is the command name (as MPV_FORMAT_STRING). The order
    ///      of arguments is as documented in each command description.
    ///
    /// MPV_FORMAT_NODE_MAP:
    ///      Named arguments. This requires at least an entry with the key \"name\"
    ///      to be present, which must be a string, and contains the command name.
    ///      The special entry \"_flags\" is optional, and if present, must be an
    ///      array of strings, each being a command prefix to apply. All other
    ///      entries are interpreted as arguments. They must use the argument names
    ///      as documented in each command description. Some commands do not
    ///      support named arguments at all, and must use MPV_FORMAT_NODE_ARRAY.
    ///
    /// @param[in] args mpv_node with format set to one of the values documented
    ///                 above (see there for details)
    /// @param[out] result Optional, pass NULL if unused. If not NULL, and if the
    ///                    function succeeds, this is set to command-specific return
    ///                    data. You must call mpv_free_node_contents() to free it
    ///                    (again, only if the command actually succeeds).
    ///                    Not many commands actually use this at all.
    /// @return error code (the result parameter is not set on error)
    pub fn command_node(
        &mut self,
        arg: Node,
        require_result: bool,
    ) -> Result<Option<Node>, MpvError> {
        let Some(args) = arg.to_mpv_node() else {
            return Err(MpvError::CommandError);
        };
        let args = Box::into_raw(Box::new(args));
        let result = if require_result {
            Box::into_raw(Box::new(mpv_node {
                format: 0,
                u: mpv_node__bindgen_ty_1 { flag: 0 },
            }))
        } else {
            null_mut()
        };

        let status = unsafe { mpv_command_node(self.0, args, result) };

        let res = match MpvError::from_mpv_error(status) {
            Some(err) => Err(err),
            None => {
                if require_result {
                    Ok(Node::from_mpv_node(unsafe { *result }))
                } else {
                    Ok(None)
                }
            }
        };
        unsafe { mpv_free_node_contents(args) };
        if require_result {
            unsafe { mpv_free_node_contents(result) };
        }
        res
    }

    /// Wait for the next event, or until the timeout expires, or if another thread
    /// makes a call to mpv_wakeup(). Passing 0 as timeout will never wait, and
    /// is suitable for polling.
    ///
    /// The internal event queue has a limited size (per client handle). If you
    /// don't empty the event queue quickly enough with mpv_wait_event(), it will
    /// overflow and silently discard further events. If this happens, making
    /// asynchronous requests will fail as well (with MPV_ERROR_EVENT_QUEUE_FULL).
    ///
    /// Only one thread is allowed to call this on the same mpv_handle at a time.
    /// The API won't complain if more than one thread calls this, but it will cause
    /// race conditions in the client when accessing the shared mpv_event struct.
    /// Note that most other API functions are not restricted by this, and no API
    /// function internally calls mpv_wait_event(). Additionally, concurrent calls
    /// to different mpv_handles are always safe.
    ///
    /// As long as the timeout is 0, this is safe to be called from mpv render API
    /// threads.
    ///
    /// @param timeout Timeout in seconds, after which the function returns even if
    ///                no event was received. A MPV_EVENT_NONE is returned on
    ///                timeout. A value of 0 will disable waiting. Negative values
    ///                will wait with an infinite timeout.
    /// @return A struct containing the event ID and other data. The pointer (and
    ///         fields in the struct) stay valid until the next mpv_wait_event()
    ///         call, or until the mpv_handle is destroyed. You must not write to
    ///         the struct, and all memory referenced by it will be automatically
    ///         released by the API on the next mpv_wait_event() call, or when the
    ///         context is destroyed. The return value is never NULL.
    pub fn wait_event(&mut self, timeout: f64) -> Option<Event> {
        let event = unsafe { mpv_wait_event(self.0, timeout) };
        let res = if event == null_mut() {
            None
        } else {
            Event::from_mpv_event(unsafe { *event })
        };
        res
    }
}

fn free_array<T>(ptr: *mut *mut T) {
    let mut i = 0;
    while unsafe { *(ptr.add(i)) != null_mut() } {
        unsafe { free(*(ptr.add(i)) as _) };
        i += 8;
    }
    unsafe { free(ptr as _) };
}

impl Drop for MpvHandle {
    fn drop(&mut self) {
        unsafe {
            mpv_destroy(self.0);
        }
    }
}
