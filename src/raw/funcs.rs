use super::types::*;
use std::os::raw::*;

extern "C" {
    /// Return the MPV_CLIENT_API_VERSION the mpv source has been compiled with.
    pub fn mpv_client_api_version() -> c_ulong;

    /// Return a string describing the error. For unknown errors, the string
    /// \"unknown error\" is returned.
    ///
    /// @param error error number, see enum mpv_error
    /// @return A static string describing the error. The string is completely
    ///         static, i.e. doesn't need to be deallocated, and is valid forever.
    pub fn mpv_error_string(error: c_int) -> *const c_char;

    /// General function to deallocate memory returned by some of the API functions.
    /// Call this only if it's explicitly documented as allowed. Calling this on
    /// mpv memory not owned by the caller will lead to undefined behavior.
    ///
    /// @param data A valid pointer returned by the API, or NULL.
    pub fn mpv_free(data: *mut c_void);

    /// Return the name of this client handle. Every client has its own unique
    /// name, which is mostly used for user interface purposes.
    ///
    /// @return The client name. The string is read-only and is valid until the
    ///         mpv_handle is destroyed.
    pub fn mpv_client_name(ctx: *mut mpv_handle) -> *const c_char;

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
    pub fn mpv_client_id(ctx: *mut mpv_handle) -> i64;

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
    pub fn mpv_create() -> *mut mpv_handle;

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
    ///
    /// @return error code
    pub fn mpv_initialize(ctx: *mut mpv_handle) -> c_int;

    /// Disconnect and destroy the mpv_handle. ctx will be deallocated with this
    /// API call.
    ///
    /// If the last mpv_handle is detached, the core player is destroyed. In
    /// addition, if there are only weak mpv_handles (such as created by
    /// mpv_create_weak_client() or internal scripts), these mpv_handles will
    /// be sent MPV_EVENT_SHUTDOWN. This function may block until these clients
    /// have responded to the shutdown event, and the core is finally destroyed.
    pub fn mpv_destroy(ctx: *mut mpv_handle);

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
    pub fn mpv_terminate_destroy(ctx: *mut mpv_handle);

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
    /// @param ctx Used to get the reference to the mpv core; handle-specific
    ///            settings and parameters are not used.
    ///            If NULL, this function behaves like mpv_create() (ignores name).
    /// @param name The client name. This will be returned by mpv_client_name(). If
    ///             the name is already in use, or contains non-alphanumeric
    ///             characters (other than '_'), the name is modified to fit.
    ///             If NULL, an arbitrary name is automatically chosen.
    /// @return a new handle, or NULL on error
    pub fn mpv_create_client(ctx: *mut mpv_handle, name: *const c_char) -> *mut mpv_handle;

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
    pub fn mpv_create_weak_client(ctx: *mut mpv_handle, name: *const c_char) -> *mut mpv_handle;

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
    pub fn mpv_load_config_file(ctx: *mut mpv_handle, filename: *const c_char) -> c_int;

    /// Return the internal time in nanoseconds. This has an arbitrary start offset,
    /// but will never wrap or go backwards.
    ///
    /// Note that this is always the real time, and doesn't necessarily have to do
    /// with playback time. For example, playback could go faster or slower due to
    /// playback speed, or due to playback being paused. Use the \"time-pos\" property
    /// instead to get the playback status.
    ///
    /// Unlike other libmpv APIs, this can be called at absolutely any time (even
    /// within wakeup callbacks), as long as the context is valid.
    ///
    /// Safe to be called from mpv render API threads.
    pub fn mpv_get_time_ns(ctx: *mut mpv_handle) -> i64;

    /// Same as mpv_get_time_ns but in microseconds.
    pub fn mpv_get_time_us(ctx: *mut mpv_handle) -> i64;

    /// Frees any data referenced by the node. It doesn't free the node itself.
    /// Call this only if the mpv client API set the node. If you constructed the
    /// node yourself (manually), you have to free it yourself.
    ///
    /// If node->format is MPV_FORMAT_NONE, this call does nothing. Likewise, if
    /// the client API sets a node with this format, this function doesn't need to
    /// be called. (This is just a clarification that there's no danger of anything
    /// strange happening in these cases.)
    pub fn mpv_free_node_contents(node: *mut mpv_node);

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
    pub fn mpv_set_option(
        ctx: *mut mpv_handle,
        name: *const c_char,
        format: mpv_format,
        data: *mut c_void,
    ) -> c_int;

    /// Convenience function to set an option to a string value. This is like
    /// calling mpv_set_option() with MPV_FORMAT_STRING.
    ///
    /// @return error code
    pub fn mpv_set_option_string(
        ctx: *mut mpv_handle,
        name: *const c_char,
        data: *const c_char,
    ) -> c_int;

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
    pub fn mpv_command(ctx: *mut mpv_handle, args: *mut *const c_char) -> c_int;

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
    pub fn mpv_command_node(
        ctx: *mut mpv_handle,
        args: *mut mpv_node,
        result: *mut mpv_node,
    ) -> c_int;

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
    pub fn mpv_command_ret(
        ctx: *mut mpv_handle,
        args: *mut *const c_char,
        result: *mut mpv_node,
    ) -> c_int;

    /// Same as mpv_command, but use input.conf parsing for splitting arguments.
    /// This is slightly simpler, but also more error prone, since arguments may
    /// need quoting/escaping.
    ///
    /// This also has OSD and string expansion enabled by default.
    pub fn mpv_command_string(ctx: *mut mpv_handle, args: *const c_char) -> c_int;

    /// Same as mpv_command, but run the command asynchronously.
    ///
    /// Commands are executed asynchronously. You will receive a
    /// MPV_EVENT_COMMAND_REPLY event. This event will also have an
    /// error code set if running the command failed. For commands that
    /// return data, the data is put into mpv_event_command.result.
    ///
    /// The only case when you do not receive an event is when the function call
    /// itself fails. This happens only if parsing the command itself (or otherwise
    /// validating it) fails, i.e. the return code of the API call is not 0 or
    /// positive.
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param reply_userdata the value mpv_event.reply_userdata of the reply will
    ///                       be set to (see section about asynchronous calls)
    /// @param args NULL-terminated list of strings (see mpv_command())
    /// @return error code (if parsing or queuing the command fails)
    pub fn mpv_command_async(
        ctx: *mut mpv_handle,
        reply_userdata: u64,
        args: *mut *const c_char,
    ) -> c_int;

    /// Same as mpv_command_node(), but run it asynchronously. Basically, this
    /// function is to mpv_command_node() what mpv_command_async() is to
    /// mpv_command().
    ///
    /// See mpv_command_async() for details.
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param reply_userdata the value mpv_event.reply_userdata of the reply will
    ///                       be set to (see section about asynchronous calls)
    /// @param args as in mpv_command_node()
    /// @return error code (if parsing or queuing the command fails)
    pub fn mpv_command_node_async(
        ctx: *mut mpv_handle,
        reply_userdata: u64,
        args: *mut mpv_node,
    ) -> c_int;

    /// Signal to all async requests with the matching ID to abort. This affects
    /// the following API calls:
    ///
    ///      mpv_command_async
    ///      mpv_command_node_async
    ///
    /// All of these functions take a reply_userdata parameter. This API function
    /// tells all requests with the matching reply_userdata value to try to return
    /// as soon as possible. If there are multiple requests with matching ID, it
    /// aborts all of them.
    ///
    /// This API function is mostly asynchronous itself. It will not wait until the
    /// command is aborted. Instead, the command will terminate as usual, but with
    /// some work not done. How this is signaled depends on the specific command (for
    /// example, the \"subprocess\" command will indicate it by \"killed_by_us\" set to
    /// true in the result). How long it takes also depends on the situation. The
    /// aborting process is completely asynchronous.
    ///
    /// Not all commands may support this functionality. In this case, this function
    /// will have no effect. The same is true if the request using the passed
    /// reply_userdata has already terminated, has not been started yet, or was
    /// never in use at all.
    ///
    /// You have to be careful of race conditions: the time during which the abort
    /// request will be effective is _after_ e.g. mpv_command_async() has returned,
    /// and before the command has signaled completion with MPV_EVENT_COMMAND_REPLY.
    ///
    /// @param reply_userdata ID of the request to be aborted (see above)
    pub fn mpv_abort_async_command(ctx: *mut mpv_handle, reply_userdata: u64);

    /// Set a property to a given value. Properties are essentially variables which
    /// can be queried or set at runtime. For example, writing to the pause property
    /// will actually pause or unpause playback.
    ///
    /// If the format doesn't match with the internal format of the property, access
    /// usually will fail with MPV_ERROR_PROPERTY_FORMAT. In some cases, the data
    /// is automatically converted and access succeeds. For example, MPV_FORMAT_INT64
    /// is always converted to MPV_FORMAT_DOUBLE, and access using MPV_FORMAT_STRING
    /// usually invokes a string parser. The same happens when calling this function
    /// with MPV_FORMAT_NODE: the underlying format may be converted to another
    /// type if possible.
    ///
    /// Using a format other than MPV_FORMAT_NODE is equivalent to constructing a
    /// mpv_node with the given format and data, and passing the mpv_node to this
    /// function. (Before API version 1.21, this was different.)
    ///
    /// Note: starting with mpv 0.21.0 (client API version 1.23), this can be used to
    ///       set options in general. It even can be used before mpv_initialize()
    ///       has been called. If called before mpv_initialize(), setting properties
    ///       not backed by options will result in MPV_ERROR_PROPERTY_UNAVAILABLE.
    ///       In some cases, properties and options still conflict. In these cases,
    ///       mpv_set_property() accesses the options before mpv_initialize(), and
    ///       the properties after mpv_initialize(). These conflicts will be removed
    ///       in mpv 0.23.0. See mpv_set_option() for further remarks.
    ///
    /// @param name The property name. See input.rst for a list of properties.
    /// @param format see enum mpv_format.
    /// @param[in] data Option value.
    /// @return error code
    pub fn mpv_set_property(
        ctx: *mut mpv_handle,
        name: *const c_char,
        format: mpv_format,
        data: *mut c_void,
    ) -> c_int;

    /// Convenience function to set a property to a string value.
    ///
    /// This is like calling mpv_set_property() with MPV_FORMAT_STRING.
    pub fn mpv_set_property_string(
        ctx: *mut mpv_handle,
        name: *const c_char,
        data: *const c_char,
    ) -> c_int;

    /// Convenience function to delete a property.
    ///
    /// This is equivalent to running the command \"del [name]\".
    ///
    /// @param name The property name. See input.rst for a list of properties.
    /// @return error code
    pub fn mpv_del_property(ctx: *mut mpv_handle, name: *const c_char) -> c_int;

    /// Set a property asynchronously. You will receive the result of the operation
    /// as MPV_EVENT_SET_PROPERTY_REPLY event. The mpv_event.error field will contain
    /// the result status of the operation. Otherwise, this function is similar to
    /// mpv_set_property().
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param reply_userdata see section about asynchronous calls
    /// @param name The property name.
    /// @param format see enum mpv_format.
    /// @param[in] data Option value. The value will be copied by the function. It
    ///                 will never be modified by the client API.
    /// @return error code if sending the request failed
    pub fn mpv_set_property_async(
        ctx: *mut mpv_handle,
        reply_userdata: u64,
        name: *const c_char,
        format: mpv_format,
        data: *mut c_void,
    ) -> c_int;

    /// Read the value of the given property.
    ///
    /// If the format doesn't match with the internal format of the property, access
    /// usually will fail with MPV_ERROR_PROPERTY_FORMAT. In some cases, the data
    /// is automatically converted and access succeeds. For example, MPV_FORMAT_INT64
    /// is always converted to MPV_FORMAT_DOUBLE, and access using MPV_FORMAT_STRING
    /// usually invokes a string formatter.
    ///
    /// @param name The property name.
    /// @param format see enum mpv_format.
    /// @param[out] data Pointer to the variable holding the option value. On
    ///                  success, the variable will be set to a copy of the option
    ///                  value. For formats that require dynamic memory allocation,
    ///                  you can free the value with mpv_free() (strings) or
    ///                  mpv_free_node_contents() (MPV_FORMAT_NODE).
    /// @return error code
    pub fn mpv_get_property(
        ctx: *mut mpv_handle,
        name: *const c_char,
        format: mpv_format,
        data: *mut c_void,
    ) -> c_int;

    /// Return the value of the property with the given name as string. This is
    /// equivalent to mpv_get_property() with MPV_FORMAT_STRING.
    ///
    /// See MPV_FORMAT_STRING for character encoding issues.
    ///
    /// On error, NULL is returned. Use mpv_get_property() if you want fine-grained
    /// error reporting.
    ///
    /// @param name The property name.
    /// @return Property value, or NULL if the property can't be retrieved. Free
    ///         the string with mpv_free().
    pub fn mpv_get_property_string(ctx: *mut mpv_handle, name: *const c_char) -> *mut c_char;

    /// Return the property as \"OSD\" formatted string. This is the same as
    /// mpv_get_property_string, but using MPV_FORMAT_OSD_STRING.
    ///
    /// @return Property value, or NULL if the property can't be retrieved. Free
    ///         the string with mpv_free().
    pub fn mpv_get_property_osd_string(ctx: *mut mpv_handle, name: *const c_char) -> *mut c_char;

    /// Get a property asynchronously. You will receive the result of the operation
    /// as well as the property data with the MPV_EVENT_GET_PROPERTY_REPLY event.
    /// You should check the mpv_event.error field on the reply event.
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param reply_userdata see section about asynchronous calls
    /// @param name The property name.
    /// @param format see enum mpv_format.
    /// @return error code if sending the request failed
    pub fn mpv_get_property_async(
        ctx: *mut mpv_handle,
        reply_userdata: u64,
        name: *const c_char,
        format: mpv_format,
    ) -> c_int;

    /// Get a notification whenever the given property changes. You will receive
    /// updates as MPV_EVENT_PROPERTY_CHANGE. Note that this is not very precise:
    /// for some properties, it may not send updates even if the property changed.
    /// This depends on the property, and it's a valid feature request to ask for
    /// better update handling of a specific property. (For some properties, like
    /// ``clock``, which shows the wall clock, this mechanism doesn't make too
    /// much sense anyway.)
    ///
    /// Property changes are coalesced: the change events are returned only once the
    /// event queue becomes empty (e.g. mpv_wait_event() would block or return
    /// MPV_EVENT_NONE), and then only one event per changed property is returned.
    ///
    /// You always get an initial change notification. This is meant to initialize
    /// the user's state to the current value of the property.
    ///
    /// Normally, change events are sent only if the property value changes according
    /// to the requested format. mpv_event_property will contain the property value
    /// as data member.
    ///
    /// Warning: if a property is unavailable or retrieving it caused an error,
    ///          MPV_FORMAT_NONE will be set in mpv_event_property, even if the
    ///          format parameter was set to a different value. In this case, the
    ///          mpv_event_property.data field is invalid.
    ///
    /// If the property is observed with the format parameter set to MPV_FORMAT_NONE,
    /// you get low-level notifications whether the property _may_ have changed, and
    /// the data member in mpv_event_property will be unset. With this mode, you
    /// will have to determine yourself whether the property really changed. On the
    /// other hand, this mechanism can be faster and uses less resources.
    ///
    /// Observing a property that doesn't exist is allowed. (Although it may still
    /// cause some sporadic change events.)
    ///
    /// Keep in mind that you will get change notifications even if you change a
    /// property yourself. Try to avoid endless feedback loops, which could happen
    /// if you react to the change notifications triggered by your own change.
    ///
    /// Only the mpv_handle on which this was called will receive the property
    /// change events, or can unobserve them.
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param reply_userdata This will be used for the mpv_event.reply_userdata
    ///                       field for the received MPV_EVENT_PROPERTY_CHANGE
    ///                       events. (Also see section about asynchronous calls,
    ///                       although this function is somewhat different from
    ///                       actual asynchronous calls.)
    ///                       If you have no use for this, pass 0.
    ///                       Also see mpv_unobserve_property().
    /// @param name The property name.
    /// @param format see enum mpv_format. Can be MPV_FORMAT_NONE to omit values
    ///               from the change events.
    /// @return error code (usually fails only on OOM or unsupported format)
    pub fn mpv_observe_property(
        mpv: *mut mpv_handle,
        reply_userdata: u64,
        name: *const c_char,
        format: mpv_format,
    ) -> c_int;

    /// Undo mpv_observe_property(). This will remove all observed properties for
    /// which the given number was passed as reply_userdata to mpv_observe_property.
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param registered_reply_userdata ID that was passed to mpv_observe_property
    /// @return negative value is an error code, >=0 is number of removed properties
    ///         on success (includes the case when 0 were removed)
    pub fn mpv_unobserve_property(mpv: *mut mpv_handle, registered_reply_userdata: u64) -> c_int;

    /// Return a string describing the event. For unknown events, NULL is returned.
    ///
    /// Note that all events actually returned by the API will also yield a non-NULL
    /// string with this function.
    ///
    /// @param event event ID, see see enum mpv_event_id
    /// @return A static string giving a short symbolic name of the event. It
    ///         consists of lower-case alphanumeric characters and can include \"-\"
    ///         characters. This string is suitable for use in e.g. scripting
    ///         interfaces.
    ///         The string is completely static, i.e. doesn't need to be deallocated,
    ///         and is valid forever.
    pub fn mpv_event_name(event: mpv_event_id) -> *const c_char;

    /// Convert the given src event to a mpv_node, and set *dst to the result. *dst
    /// is set to a MPV_FORMAT_NODE_MAP, with fields for corresponding mpv_event and
    /// mpv_event.data/mpv_event_* fields.
    ///
    /// The exact details are not completely documented out of laziness. A start
    /// is located in the \"Events\" section of the manpage.
    ///
    /// *dst may point to newly allocated memory, or pointers in mpv_event. You must
    /// copy the entire mpv_node if you want to reference it after mpv_event becomes
    /// invalid (such as making a new mpv_wait_event() call, or destroying the
    /// mpv_handle from which it was returned). Call mpv_free_node_contents() to free
    /// any memory allocations made by this API function.
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param dst Target. This is not read and fully overwritten. Must be released
    ///            with mpv_free_node_contents(). Do not write to pointers returned
    ///            by it. (On error, this may be left as an empty node.)
    /// @param src The source event. Not modified (it's not const due to the author's
    ///            prejudice of the C version of const).
    /// @return error code (MPV_ERROR_NOMEM only, if at all)
    pub fn mpv_event_to_node(dst: *mut mpv_node, src: *mut mpv_event) -> c_int;

    /// Enable or disable the given event.
    ///
    /// Some events are enabled by default. Some events can't be disabled.
    ///
    /// (Informational note: currently, all events are enabled by default, except
    ///  MPV_EVENT_TICK.)
    ///
    /// Safe to be called from mpv render API threads.
    ///
    /// @param event See enum mpv_event_id.
    /// @param enable 1 to enable receiving this event, 0 to disable it.
    /// @return error code
    pub fn mpv_request_event(ctx: *mut mpv_handle, event: mpv_event_id, enable: c_int) -> c_int;

    /// Enable or disable receiving of log messages. These are the messages the
    /// command line player prints to the terminal. This call sets the minimum
    /// required log level for a message to be received with MPV_EVENT_LOG_MESSAGE.
    ///
    /// @param min_level Minimal log level as string. Valid log levels:
    ///                      no fatal error warn info v debug trace
    ///                  The value \"no\" disables all messages. This is the default.
    ///                  An exception is the value \"terminal-default\", which uses the
    ///                  log level as set by the \"--msg-level\" option. This works
    ///                  even if the terminal is disabled. (Since API version 1.19.)
    ///                  Also see mpv_log_level.
    /// @return error code
    pub fn mpv_request_log_messages(ctx: *mut mpv_handle, min_level: *const c_char) -> c_int;

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
    pub fn mpv_wait_event(ctx: *mut mpv_handle, timeout: f64) -> *mut mpv_event;

    /// Interrupt the current mpv_wait_event() call. This will wake up the thread
    /// currently waiting in mpv_wait_event(). If no thread is waiting, the next
    /// mpv_wait_event() call will return immediately (this is to avoid lost
    /// wakeups).
    ///
    /// mpv_wait_event() will receive a MPV_EVENT_NONE if it's woken up due to
    /// this call. But note that this dummy event might be skipped if there are
    /// already other events queued. All what counts is that the waiting thread
    /// is woken up at all.
    ///
    /// Safe to be called from mpv render API threads.
    pub fn mpv_wakeup(ctx: *mut mpv_handle);

    /// Set a custom function that should be called when there are new events. Use
    /// this if blocking in mpv_wait_event() to wait for new events is not feasible.
    ///
    /// Keep in mind that the callback will be called from foreign threads. You
    /// must not make any assumptions of the environment, and you must return as
    /// soon as possible (i.e. no long blocking waits). Exiting the callback through
    /// any other means than a normal return is forbidden (no throwing exceptions,
    /// no longjmp() calls). You must not change any local thread state (such as
    /// the C floating point environment).
    ///
    /// You are not allowed to call any client API functions inside of the callback.
    /// In particular, you should not do any processing in the callback, but wake up
    /// another thread that does all the work. The callback is meant strictly for
    /// notification only, and is called from arbitrary core parts of the player,
    /// that make no considerations for reentrant API use or allowing the callee to
    /// spend a lot of time doing other things. Keep in mind that it's also possible
    /// that the callback is called from a thread while a mpv API function is called
    /// (i.e. it can be reentrant).
    ///
    /// In general, the client API expects you to call mpv_wait_event() to receive
    /// notifications, and the wakeup callback is merely a helper utility to make
    /// this easier in certain situations. Note that it's possible that there's
    /// only one wakeup callback invocation for multiple events. You should call
    /// mpv_wait_event() with no timeout until MPV_EVENT_NONE is reached, at which
    /// point the event queue is empty.
    ///
    /// If you actually want to do processing in a callback, spawn a thread that
    /// does nothing but call mpv_wait_event() in a loop and dispatches the result
    /// to a callback.
    ///
    /// Only one wakeup callback can be set.
    ///
    /// @param cb function that should be called if a wakeup is required
    /// @param d arbitrary userdata passed to cb
    pub fn mpv_set_wakeup_callback(
        ctx: *mut mpv_handle,
        cb: ::std::option::Option<unsafe extern "C" fn(d: *mut c_void)>,
        d: *mut c_void,
    );

    /// Block until all asynchronous requests are done. This affects functions like
    /// mpv_command_async(), which return immediately and return their result as
    /// events.
    ///
    /// This is a helper, and somewhat equivalent to calling mpv_wait_event() in a
    /// loop until all known asynchronous requests have sent their reply as event,
    /// except that the event queue is not emptied.
    ///
    /// In case you called mpv_suspend() before, this will also forcibly reset the
    /// suspend counter of the given handle.
    pub fn mpv_wait_async_requests(ctx: *mut mpv_handle);

    /// A hook is like a synchronous event that blocks the player. You register
    /// a hook handler with this function. You will get an event, which you need
    /// to handle, and once things are ready, you can let the player continue with
    /// mpv_hook_continue().
    ///
    /// Currently, hooks can't be removed explicitly. But they will be implicitly
    /// removed if the mpv_handle it was registered with is destroyed. This also
    /// continues the hook if it was being handled by the destroyed mpv_handle (but
    /// this should be avoided, as it might mess up order of hook execution).
    ///
    /// Hook handlers are ordered globally by priority and order of registration.
    /// Handlers for the same hook with same priority are invoked in order of
    /// registration (the handler registered first is run first). Handlers with
    /// lower priority are run first (which seems backward).
    ///
    /// See the \"Hooks\" section in the manpage to see which hooks are currently
    /// defined.
    ///
    /// Some hooks might be reentrant (so you get multiple MPV_EVENT_HOOK for the
    /// same hook). If this can happen for a specific hook type, it will be
    /// explicitly documented in the manpage.
    ///
    /// Only the mpv_handle on which this was called will receive the hook events,
    /// or can \"continue\" them.
    ///
    /// @param reply_userdata This will be used for the mpv_event.reply_userdata
    ///                       field for the received MPV_EVENT_HOOK events.
    ///                       If you have no use for this, pass 0.
    /// @param name The hook name. This should be one of the documented names. But
    ///             if the name is unknown, the hook event will simply be never
    ///             raised.
    /// @param priority See remarks above. Use 0 as a neutral default.
    /// @return error code (usually fails only on OOM)
    pub fn mpv_hook_add(
        ctx: *mut mpv_handle,
        reply_userdata: u64,
        name: *const c_char,
        priority: c_int,
    ) -> c_int;

    /// Respond to a MPV_EVENT_HOOK event. You must call this after you have handled
    /// the event. There is no way to \"cancel\" or \"stop\" the hook.
    ///
    /// Calling this will will typically unblock the player for whatever the hook
    /// is responsible for (e.g. for the \"on_load\" hook it lets it continue
    /// playback).
    ///
    /// It is explicitly undefined behavior to call this more than once for each
    /// MPV_EVENT_HOOK, to pass an incorrect ID, or to call this on a mpv_handle
    /// different from the one that registered the handler and received the event.
    ///
    /// @param id This must be the value of the mpv_event_hook.id field for the
    ///           corresponding MPV_EVENT_HOOK.
    /// @return error code
    pub fn mpv_hook_continue(ctx: *mut mpv_handle, id: u64) -> c_int;

    /// Return a UNIX file descriptor referring to the read end of a pipe. This
    /// pipe can be used to wake up a poll() based processing loop. The purpose of
    /// this function is very similar to mpv_set_wakeup_callback(), and provides
    /// a primitive mechanism to handle coordinating a foreign event loop and the
    /// libmpv event loop. The pipe is non-blocking. It's closed when the mpv_handle
    /// is destroyed. This function always returns the same value (on success).
    ///
    /// This is in fact implemented using the same underlying code as for
    /// mpv_set_wakeup_callback() (though they don't conflict), and it is as if each
    /// callback invocation writes a single 0 byte to the pipe. When the pipe
    /// becomes readable, the code calling poll() (or select()) on the pipe should
    /// read all contents of the pipe and then call mpv_wait_event(c, 0) until
    /// no new events are returned. The pipe contents do not matter and can just
    /// be discarded. There is not necessarily one byte per readable event in the
    /// pipe. For example, the pipes are non-blocking, and mpv won't block if the
    /// pipe is full. Pipes are normally limited to 4096 bytes, so if there are
    /// more than 4096 events, the number of readable bytes can not equal the number
    /// of events queued. Also, it's possible that mpv does not write to the pipe
    /// once it's guaranteed that the client was already signaled. See the example
    /// below how to do it correctly.
    ///
    /// Example:
    ///
    ///  int pipefd = mpv_get_wakeup_pipe(mpv);
    ///  if (pipefd < 0)
    ///      error();
    ///  while (1) {
    ///      struct pollfd pfds[1] = {
    ///          { .fd = pipefd, .events = POLLIN },
    ///      };
    ///      // Wait until there are possibly new mpv events.
    ///      poll(pfds, 1, -1);
    ///      if (pfds[0].revents & POLLIN) {
    ///          // Empty the pipe. Doing this before calling mpv_wait_event()
    ///          // ensures that no wakeups are missed. It's not so important to
    ///          // make sure the pipe is really empty (it will just cause some
    ///          // additional wakeups in unlikely corner cases).
    ///          char unused[256];
    ///          read(pipefd, unused, sizeof(unused));
    ///          while (1) {
    ///              mpv_event *ev = mpv_wait_event(mpv, 0);
    ///              // If MPV_EVENT_NONE is received, the event queue is empty.
    ///              if (ev->event_id == MPV_EVENT_NONE)
    ///                  break;
    ///              // Process the event.
    ///              ...
    ///          }
    ///      }
    ///  }
    ///
    /// @deprecated this function will be removed in the future. If you need this
    ///             functionality, use mpv_set_wakeup_callback(), create a pipe
    ///             manually, and call write() on your pipe in the callback.
    ///
    /// @return A UNIX FD of the read end of the wakeup pipe, or -1 on error.
    ///         On MS Windows/MinGW, this will always return -1.
    pub fn mpv_get_wakeup_pipe(ctx: *mut mpv_handle) -> c_int;

    /// Initialize the renderer state. Depending on the backend used, this will
    /// access the underlying GPU API and initialize its own objects.
    ///
    /// You must free the context with mpv_render_context_free(). Not doing so before
    /// the mpv core is destroyed may result in memory leaks or crashes.
    ///
    /// Currently, only at most 1 context can exists per mpv core (it represents the
    /// main video output).
    ///
    /// You should pass the following parameters:
    ///  - MPV_RENDER_PARAM_API_TYPE to select the underlying backend/GPU API.
    ///  - Backend-specific init parameter, like MPV_RENDER_PARAM_OPENGL_INIT_PARAMS.
    ///  - Setting MPV_RENDER_PARAM_ADVANCED_CONTROL and following its rules is
    ///    strongly recommended.
    ///  - If you want to use hwdec, possibly hwdec interop resources.
    ///
    /// @param res set to the context (on success) or NULL (on failure). The value
    ///            is never read and always overwritten.
    /// @param mpv handle used to get the core (the mpv_render_context won't depend
    ///            on this specific handle, only the core referenced by it)
    /// @param params an array of parameters, terminated by type==0. It's left
    ///               unspecified what happens with unknown parameters. At least
    ///               MPV_RENDER_PARAM_API_TYPE is required, and most backends will
    ///               require another backend-specific parameter.
    /// @return error code, including but not limited to:
    ///      MPV_ERROR_UNSUPPORTED: the OpenGL version is not supported
    ///                             (or required extensions are missing)
    ///      MPV_ERROR_NOT_IMPLEMENTED: an unknown API type was provided, or
    ///                                 support for the requested API was not
    ///                                 built in the used libmpv binary.
    ///      MPV_ERROR_INVALID_PARAMETER: at least one of the provided parameters was
    ///                                   not valid.
    pub fn mpv_render_context_create(
        res: *mut *mut mpv_render_context,
        mpv: *mut mpv_handle,
        params: *mut mpv_render_param,
    ) -> c_int;

    /// Attempt to change a single parameter. Not all backends and parameter types
    /// support all kinds of changes.
    ///
    /// @param ctx a valid render context
    /// @param param the parameter type and data that should be set
    /// @return error code. If a parameter could actually be changed, this returns
    ///         success, otherwise an error code depending on the parameter type
    ///         and situation.
    pub fn mpv_render_context_set_parameter(
        ctx: *mut mpv_render_context,
        param: mpv_render_param,
    ) -> c_int;

    /// Retrieve information from the render context. This is NOT a counterpart to
    /// mpv_render_context_set_parameter(), because you generally can't read
    /// parameters set with it, and this function is not meant for this purpose.
    /// Instead, this is for communicating information from the renderer back to the
    /// user. See mpv_render_param_type; entries which support this function
    /// explicitly mention it, and for other entries you can assume it will fail.
    ///
    /// You pass param with param.type set and param.data pointing to a variable
    /// of the required data type. The function will then overwrite that variable
    /// with the returned value (at least on success).
    ///
    /// @param ctx a valid render context
    /// @param param the parameter type and data that should be retrieved
    /// @return error code. If a parameter could actually be retrieved, this returns
    ///         success, otherwise an error code depending on the parameter type
    ///         and situation. MPV_ERROR_NOT_IMPLEMENTED is used for unknown
    ///         param.type, or if retrieving it is not supported.
    pub fn mpv_render_context_get_info(
        ctx: *mut mpv_render_context,
        param: mpv_render_param,
    ) -> c_int;

    /// Set the callback that notifies you when a new video frame is available, or
    /// if the video display configuration somehow changed and requires a redraw.
    /// Similar to mpv_set_wakeup_callback(), you must not call any mpv API from
    /// the callback, and all the other listed restrictions apply (such as not
    /// exiting the callback by throwing exceptions).
    ///
    /// This can be called from any thread, except from an update callback. In case
    /// of the OpenGL backend, no OpenGL state or API is accessed.
    ///
    /// Calling this will raise an update callback immediately.
    ///
    /// @param callback callback(callback_ctx) is called if the frame should be
    ///                 redrawn
    /// @param callback_ctx opaque argument to the callback
    pub fn mpv_render_context_set_update_callback(
        ctx: *mut mpv_render_context,
        callback: mpv_render_update_fn,
        callback_ctx: *mut c_void,
    );

    /// The API user is supposed to call this when the update callback was invoked
    /// (like all mpv_render_* functions, this has to happen on the render thread,
    /// and _not_ from the update callback itself).
    ///
    /// This is optional if MPV_RENDER_PARAM_ADVANCED_CONTROL was not set (default).
    /// Otherwise, it's a hard requirement that this is called after each update
    /// callback. If multiple update callback happened, and the function could not
    /// be called sooner, it's OK to call it once after the last callback.
    ///
    /// If an update callback happens during or after this function, the function
    /// must be called again at the soonest possible time.
    ///
    /// If MPV_RENDER_PARAM_ADVANCED_CONTROL was set, this will do additional work
    /// such as allocating textures for the video decoder.
    ///
    /// @return a bitset of mpv_render_update_flag values (i.e. multiple flags are
    ///         combined with bitwise or). Typically, this will tell the API user
    ///         what should happen next. E.g. if the MPV_RENDER_UPDATE_FRAME flag is
    ///         set, mpv_render_context_render() should be called. If flags unknown
    ///         to the API user are set, or if the return value is 0, nothing needs
    ///         to be done.
    pub fn mpv_render_context_update(ctx: *mut mpv_render_context) -> u64;

    /// Render video.
    ///
    /// Typically renders the video to a target surface provided via mpv_render_param
    /// (the details depend on the backend in use). Options like \"panscan\" are
    /// applied to determine which part of the video should be visible and how the
    /// video should be scaled. You can change these options at runtime by using the
    /// mpv property API.
    ///
    /// The renderer will reconfigure itself every time the target surface
    /// configuration (such as size) is changed.
    ///
    /// This function implicitly pulls a video frame from the internal queue and
    /// renders it. If no new frame is available, the previous frame is redrawn.
    /// The update callback set with mpv_render_context_set_update_callback()
    /// notifies you when a new frame was added. The details potentially depend on
    /// the backends and the provided parameters.
    ///
    /// Generally, libmpv will invoke your update callback some time before the video
    /// frame should be shown, and then lets this function block until the supposed
    /// display time. This will limit your rendering to video FPS. You can prevent
    /// this by setting the \"video-timing-offset\" global option to 0. (This applies
    /// only to \"audio\" video sync mode.)
    ///
    /// You should pass the following parameters:
    ///  - Backend-specific target object, such as MPV_RENDER_PARAM_OPENGL_FBO.
    ///  - Possibly transformations, such as MPV_RENDER_PARAM_FLIP_Y.
    ///
    /// @param ctx a valid render context
    /// @param params an array of parameters, terminated by type==0. Which parameters
    ///               are required depends on the backend. It's left unspecified what
    ///               happens with unknown parameters.
    /// @return error code
    pub fn mpv_render_context_render(
        ctx: *mut mpv_render_context,
        params: *mut mpv_render_param,
    ) -> c_int;

    /// Tell the renderer that a frame was flipped at the given time. This is
    /// optional, but can help the player to achieve better timing.
    ///
    /// Note that calling this at least once informs libmpv that you will use this
    /// function. If you use it inconsistently, expect bad video playback.
    ///
    /// If this is called while no video is initialized, it is ignored.
    ///
    /// @param ctx a valid render context
    pub fn mpv_render_context_report_swap(ctx: *mut mpv_render_context);

    /// Destroy the mpv renderer state.
    ///
    /// If video is still active (e.g. a file playing), video will be disabled
    /// forcefully.
    ///
    /// @param ctx a valid render context. After this function returns, this is not
    ///            a valid pointer anymore. NULL is also allowed and does nothing.
    pub fn mpv_render_context_free(ctx: *mut mpv_render_context);

    /// Add a custom stream protocol. This will register a protocol handler under
    /// the given protocol prefix, and invoke the given callbacks if an URI with the
    /// matching protocol prefix is opened.
    ///
    /// The \"ro\" is for read-only - only read-only streams can be registered with
    /// this function.
    ///
    /// The callback remains registered until the mpv core is registered.
    ///
    /// If a custom stream with the same name is already registered, then the
    /// MPV_ERROR_INVALID_PARAMETER error is returned.
    ///
    /// @param protocol protocol prefix, for example \"foo\" for \"foo://\" URIs
    /// @param user_data opaque pointer passed into the mpv_stream_cb_open_fn
    ///                  callback.
    /// @return error code
    pub fn mpv_stream_cb_add_ro(
        ctx: *mut mpv_handle,
        protocol: *const c_char,
        user_data: *mut c_void,
        open_fn: mpv_stream_cb_open_ro_fn,
    ) -> c_int;
}
