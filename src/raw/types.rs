#![allow(non_camel_case_types)]

use std::os::raw::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_handle {
    _unused: [u8; 0],
}

/// List of error codes than can be returned by API functions. 0 and positive
/// return values always mean success, negative values are always errors.
pub type mpv_error = c_int;

/// Data format for options and properties. The API functions to get/set
/// properties and options support multiple formats, and this enum describes
/// them.
pub type mpv_format = c_uint;

/// Generic data storage.
///
/// If mpv writes this struct (e.g. via mpv_get_property()), you must not change
/// the data. In some cases (mpv_get_property()), you have to free it with
/// mpv_free_node_contents(). If you fill this struct yourself, you're also
/// responsible for freeing it, and you must not call mpv_free_node_contents().
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpv_node {
    pub u: mpv_node__bindgen_ty_1,
    /// Type of the data stored in this struct. This value rules what members in
    /// the given union can be accessed. The following formats are currently
    /// defined to be allowed in mpv_node:
    ///
    ///  MPV_FORMAT_STRING       (u.string)
    ///  MPV_FORMAT_FLAG         (u.flag)
    ///  MPV_FORMAT_INT64        (u.int64)
    ///  MPV_FORMAT_DOUBLE       (u.double_)
    ///  MPV_FORMAT_NODE_ARRAY   (u.list)
    ///  MPV_FORMAT_NODE_MAP     (u.list)
    ///  MPV_FORMAT_BYTE_ARRAY   (u.ba)
    ///  MPV_FORMAT_NONE         (no member)
    ///
    /// If you encounter a value you don't know, you must not make any
    /// assumptions about the contents of union u.
    pub format: mpv_format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union mpv_node__bindgen_ty_1 {
    pub string: *mut c_char,
    /// valid if format==MPV_FORMAT_STRING
    pub flag: c_int,
    /// valid if format==MPV_FORMAT_FLAG
    pub int64: i64,
    /// valid if format==MPV_FORMAT_INT64
    pub double_: f64,
    /// valid if format==MPV_FORMAT_DOUBLE */
    ////**
    /// valid if format==MPV_FORMAT_NODE_ARRAY
    ///    or if format==MPV_FORMAT_NODE_MAP
    pub list: *mut mpv_node_list,
    /// valid if format==MPV_FORMAT_BYTE_ARRAY
    pub ba: *mut mpv_byte_array,
}
/// (see mpv_node)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_node_list {
    /// Number of entries. Negative values are not allowed.
    pub num: c_int,
    /// MPV_FORMAT_NODE_ARRAY:
    ///  values[N] refers to value of the Nth item
    ///
    /// MPV_FORMAT_NODE_MAP:
    ///  values[N] refers to value of the Nth key/value pair
    ///
    /// If num > 0, values[0] to values[num-1] (inclusive) are valid.
    /// Otherwise, this can be NULL.
    pub values: *mut mpv_node,
    /// MPV_FORMAT_NODE_ARRAY:
    ///  unused (typically NULL), access is not allowed
    ///
    /// MPV_FORMAT_NODE_MAP:
    ///  keys[N] refers to key of the Nth key/value pair. If num > 0, keys[0] to
    ///  keys[num-1] (inclusive) are valid. Otherwise, this can be NULL.
    ///  The keys are in random order. The only guarantee is that keys[N] belongs
    ///  to the value values[N]. NULL keys are not allowed.
    pub keys: *mut *mut c_char,
}
/// (see mpv_node)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_byte_array {
    /// Pointer to the data. In what format the data is stored is up to whatever
    /// uses MPV_FORMAT_BYTE_ARRAY.
    pub data: *mut c_void,
    /// Size of the data pointed to by ptr.
    pub size: usize,
}

pub type mpv_event_id = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event_property {
    /// Name of the property.
    pub name: *const c_char,
    /// Format of the data field in the same struct. See enum mpv_format.
    /// This is always the same format as the requested format, except when
    /// the property could not be retrieved (unavailable, or an error happened),
    /// in which case the format is MPV_FORMAT_NONE.
    pub format: mpv_format,
    /// Received property value. Depends on the format. This is like the
    /// pointer argument passed to mpv_get_property().
    ///
    /// For example, for MPV_FORMAT_STRING you get the string with:
    ///
    ///    char *value = *(char **)(event_property->data);
    ///
    /// Note that this is set to NULL if retrieving the property failed (the
    /// format will be MPV_FORMAT_NONE).
    pub data: *mut c_void,
}

/// Numeric log levels. The lower the number, the more important the message is.
/// MPV_LOG_LEVEL_NONE is never used when receiving messages. The string in
/// the comment after the value is the name of the log level as used for the
/// mpv_request_log_messages() function.
/// Unused numeric values are unused, but reserved for future use.
pub type mpv_log_level = c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event_log_message {
    /// The module prefix, identifies the sender of the message. As a special
    /// case, if the message buffer overflows, this will be set to the string
    /// \"overflow\" (which doesn't appear as prefix otherwise), and the text
    /// field will contain an informative message.
    pub prefix: *const c_char,
    /// The log level as string. See mpv_request_log_messages() for possible
    /// values. The level \"no\" is never used here.
    pub level: *const c_char,
    /// The log message. It consists of 1 line of text, and is terminated with
    /// a newline character. (Before API version 1.6, it could contain multiple
    /// or partial lines.)
    pub text: *const c_char,
    /// The same contents as the level field, but as a numeric ID.
    /// Since API version 1.6.
    pub log_level: mpv_log_level,
}

/// Since API version 1.9.
pub type mpv_end_file_reason = c_uint;
/// Since API version 1.108.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event_start_file {
    /// Playlist entry ID of the file being loaded now.
    pub playlist_entry_id: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event_end_file {
    /// Corresponds to the values in enum mpv_end_file_reason.
    ///
    /// Unknown values should be treated as unknown.
    pub reason: mpv_end_file_reason,
    /// If reason==MPV_END_FILE_REASON_ERROR, this contains a mpv error code
    /// (one of MPV_ERROR_...) giving an approximate reason why playback
    /// failed. In other cases, this field is 0 (no error).
    /// Since API version 1.9.
    pub error: c_int,
    /// Playlist entry ID of the file that was being played or attempted to be
    /// played. This has the same value as the playlist_entry_id field in the
    /// corresponding mpv_event_start_file event.
    /// Since API version 1.108.
    pub playlist_entry_id: i64,
    /// If loading ended, because the playlist entry to be played was for example
    /// a playlist, and the current playlist entry is replaced with a number of
    /// other entries. This may happen at least with MPV_END_FILE_REASON_REDIRECT
    /// (other event types may use this for similar but different purposes in the
    /// future). In this case, playlist_insert_id will be set to the playlist
    /// entry ID of the first inserted entry, and playlist_insert_num_entries to
    /// the total number of inserted playlist entries. Note this in this specific
    /// case, the ID of the last inserted entry is playlist_insert_id+num-1.
    /// Beware that depending on circumstances, you may observe the new playlist
    /// entries before seeing the event (e.g. reading the \"playlist\" property or
    /// getting a property change notification before receiving the event).
    /// Since API version 1.108.
    pub playlist_insert_id: i64,
    /// See playlist_insert_id. Only non-0 if playlist_insert_id is valid. Never
    /// negative.
    /// Since API version 1.108.
    pub playlist_insert_num_entries: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event_client_message {
    /// Arbitrary arguments chosen by the sender of the message. If num_args > 0,
    /// you can access args[0] through args[num_args - 1] (inclusive). What
    /// these arguments mean is up to the sender and receiver.
    /// None of the valid items are NULL.
    pub num_args: c_int,
    pub args: *mut *const c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event_hook {
    /// The hook name as passed to mpv_hook_add().
    pub name: *const c_char,
    /// Internal ID that must be passed to mpv_hook_continue().
    pub id: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpv_event_command {
    /// Result data of the command. Note that success/failure is signaled
    /// separately via mpv_event.error. This field is only for result data
    /// in case of success. Most commands leave it at MPV_FORMAT_NONE. Set
    /// to MPV_FORMAT_NONE on failure.
    pub result: mpv_node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_event {
    /// One of mpv_event. Keep in mind that later ABI compatible releases might
    /// add new event types. These should be ignored by the API user.
    pub event_id: mpv_event_id,
    /// This is mainly used for events that are replies to (asynchronous)
    /// requests. It contains a status code, which is >= 0 on success, or < 0
    /// on error (a mpv_error value). Usually, this will be set if an
    /// asynchronous request fails.
    /// Used for:
    ///  MPV_EVENT_GET_PROPERTY_REPLY
    ///  MPV_EVENT_SET_PROPERTY_REPLY
    ///  MPV_EVENT_COMMAND_REPLY
    pub error: c_int,
    /// If the event is in reply to a request (made with this API and this
    /// API handle), this is set to the reply_userdata parameter of the request
    /// call. Otherwise, this field is 0.
    /// Used for:
    ///  MPV_EVENT_GET_PROPERTY_REPLY
    ///  MPV_EVENT_SET_PROPERTY_REPLY
    ///  MPV_EVENT_COMMAND_REPLY
    ///  MPV_EVENT_PROPERTY_CHANGE
    ///  MPV_EVENT_HOOK
    pub reply_userdata: u64,
    /// The meaning and contents of the data member depend on the event_id:
    ///  MPV_EVENT_GET_PROPERTY_REPLY:     mpv_event_property*
    ///  MPV_EVENT_PROPERTY_CHANGE:        mpv_event_property*
    ///  MPV_EVENT_LOG_MESSAGE:            mpv_event_log_message*
    ///  MPV_EVENT_CLIENT_MESSAGE:         mpv_event_client_message*
    ///  MPV_EVENT_START_FILE:             mpv_event_start_file* (since v1.108)
    ///  MPV_EVENT_END_FILE:               mpv_event_end_file*
    ///  MPV_EVENT_HOOK:                   mpv_event_hook*
    ///  MPV_EVENT_COMMAND_REPLY*          mpv_event_command*
    ///  other: NULL
    ///
    /// Note: future enhancements might add new event structs for existing or new
    ///       event types.
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_render_context {
    _unused: [u8; 0],
}

/// Parameters for mpv_render_param (which is used in a few places such as
/// mpv_render_context_create().
///
/// Also see mpv_render_param for conventions and how to use it.
pub type mpv_render_param_type = c_uint;
/// Used to pass arbitrary parameters to some mpv_render_* functions. The
/// meaning of the data parameter is determined by the type, and each
/// MPV_RENDER_PARAM_* documents what type the value must point to.
///
/// Each value documents the required data type as the pointer you cast to
/// void* and set on mpv_render_param.data. For example, if MPV_RENDER_PARAM_FOO
/// documents the type as Something* , then the code should look like this:
///
///   Something foo = {...};
///   mpv_render_param param;
///   param.type = MPV_RENDER_PARAM_FOO;
///   param.data = & foo;
///
/// Normally, the data field points to exactly 1 object. If the type is char*,
/// it points to a 0-terminated string.
///
/// In all cases (unless documented otherwise) the pointers need to remain
/// valid during the call only. Unless otherwise documented, the API functions
/// will not write to the params array or any data pointed to it.
///
/// As a convention, parameter arrays are always terminated by type==0. There
/// is no specific order of the parameters required. The order of the 2 fields in
/// this struct is guaranteed (even after ABI changes).
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_render_param {
    pub type_: mpv_render_param_type,
    pub data: *mut c_void,
}

/// Flags used in mpv_render_frame_info.flags. Each value represents a bit in it.
pub type mpv_render_frame_info_flag = c_uint;
/// Information about the next video frame that will be rendered. Can be
/// retrieved with MPV_RENDER_PARAM_NEXT_FRAME_INFO.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_render_frame_info {
    /// A bitset of mpv_render_frame_info_flag values (i.e. multiple flags are
    /// combined with bitwise or).
    pub flags: u64,
    /// Absolute time at which the frame is supposed to be displayed. This is in
    /// the same unit and base as the time returned by mpv_get_time_us(). For
    /// frames that are redrawn, or if vsync locked video timing is used (see
    /// \"video-sync\" option), then this can be 0. The \"video-timing-offset\"
    /// option determines how much \"headroom\" the render thread gets (but a high
    /// enough frame rate can reduce it anyway). mpv_render_context_render() will
    /// normally block until the time is elapsed, unless you pass it
    /// MPV_RENDER_PARAM_BLOCK_FOR_TARGET_TIME = 0.
    pub target_time: i64,
}

pub type mpv_render_update_fn = ::std::option::Option<unsafe extern "C" fn(cb_ctx: *mut c_void)>;

/// Flags returned by mpv_render_context_update(). Each value represents a bit
/// in the function's return value.
pub type mpv_render_update_flag = c_uint;
/// Flags returned by mpv_render_context_update(). Each value represents a bit
/// in the function's return value.
pub use self::mpv_render_update_flag as mpv_render_context_flag;

/// For initializing the mpv OpenGL state via MPV_RENDER_PARAM_OPENGL_INIT_PARAMS.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_opengl_init_params {
    /// This retrieves OpenGL function pointers, and will use them in subsequent
    /// operation.
    /// Usually, you can simply call the GL context APIs from this callback (e.g.
    /// glXGetProcAddressARB or wglGetProcAddress), but some APIs do not always
    /// return pointers for all standard functions (even if present); in this
    /// case you have to compensate by looking up these functions yourself when
    /// libmpv wants to resolve them through this callback.
    /// libmpv will not normally attempt to resolve GL functions on its own, nor
    /// does it link to GL libraries directly.
    pub get_proc_address: ::std::option::Option<
        unsafe extern "C" fn(ctx: *mut c_void, name: *const c_char) -> *mut c_void,
    >,
    /// Value passed as ctx parameter to get_proc_address().
    pub get_proc_address_ctx: *mut c_void,
}
/// For MPV_RENDER_PARAM_OPENGL_FBO.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_opengl_fbo {
    /// Framebuffer object name. This must be either a valid FBO generated by
    /// glGenFramebuffers() that is complete and color-renderable, or 0. If the
    /// value is 0, this refers to the OpenGL default framebuffer.
    pub fbo: c_int,
    /// Valid dimensions. This must refer to the size of the framebuffer. This
    /// must always be set.
    pub w: c_int,
    /// Valid dimensions. This must refer to the size of the framebuffer. This
    /// must always be set.
    pub h: c_int,
    /// Underlying texture internal format (e.g. GL_RGBA8), or 0 if unknown. If
    /// this is the default framebuffer, this can be an equivalent.
    pub internal_format: c_int,
}
/// Deprecated. For MPV_RENDER_PARAM_DRM_DISPLAY.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_opengl_drm_params {
    pub fd: c_int,
    pub crtc_id: c_int,
    pub connector_id: c_int,
    pub atomic_request_ptr: *mut *mut _drmModeAtomicReq,
    pub render_fd: c_int,
}
/// For MPV_RENDER_PARAM_DRM_DRAW_SURFACE_SIZE.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_opengl_drm_draw_surface_size {
    /// size of the draw plane surface in pixels.
    pub width: c_int,
    /// size of the draw plane surface in pixels.
    pub height: c_int,
}
/// For MPV_RENDER_PARAM_DRM_DISPLAY_V2.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_opengl_drm_params_v2 {
    /// DRM fd (int). Set to -1 if invalid.
    pub fd: c_int,
    /// Currently used crtc id
    pub crtc_id: c_int,
    /// Currently used connector id
    pub connector_id: c_int,
    /// Pointer to a drmModeAtomicReq pointer that is being used for the renderloop.
    /// This pointer should hold a pointer to the atomic request pointer
    /// The atomic request pointer is usually changed at every renderloop.
    pub atomic_request_ptr: *mut *mut _drmModeAtomicReq,
    /// DRM render node. Used for VAAPI interop.
    /// Set to -1 if invalid.
    pub render_fd: c_int,
}
/// Read callback used to implement a custom stream. The semantics of the
/// callback match read(2) in blocking mode. Short reads are allowed (you can
/// return less bytes than requested, and libmpv will retry reading the rest
/// with another call). If no data can be immediately read, the callback must
/// block until there is new data. A return of 0 will be interpreted as final
/// EOF, although libmpv might retry the read, or seek to a different position.
///
/// @param cookie opaque cookie identifying the stream,
///               returned from mpv_stream_cb_open_fn
/// @param buf buffer to read data into
/// @param size of the buffer
/// @return number of bytes read into the buffer
/// @return 0 on EOF
/// @return -1 on error
pub type mpv_stream_cb_read_fn = ::std::option::Option<
    unsafe extern "C" fn(cookie: *mut c_void, buf: *mut c_char, nbytes: u64) -> i64,
>;
/// Seek callback used to implement a custom stream.
///
/// Note that mpv will issue a seek to position 0 immediately after opening. This
/// is used to test whether the stream is seekable (since seekability might
/// depend on the URI contents, not just the protocol). Return
/// MPV_ERROR_UNSUPPORTED if seeking is not implemented for this stream. This
/// seek also serves to establish the fact that streams start at position 0.
///
/// This callback can be NULL, in which it behaves as if always returning
/// MPV_ERROR_UNSUPPORTED.
///
/// @param cookie opaque cookie identifying the stream,
///               returned from mpv_stream_cb_open_fn
/// @param offset target absolute stream position
/// @return the resulting offset of the stream
///         MPV_ERROR_UNSUPPORTED or MPV_ERROR_GENERIC if the seek failed
pub type mpv_stream_cb_seek_fn =
    ::std::option::Option<unsafe extern "C" fn(cookie: *mut c_void, offset: i64) -> i64>;
/// Size callback used to implement a custom stream.
///
/// Return MPV_ERROR_UNSUPPORTED if no size is known.
///
/// This callback can be NULL, in which it behaves as if always returning
/// MPV_ERROR_UNSUPPORTED.
///
/// @param cookie opaque cookie identifying the stream,
///               returned from mpv_stream_cb_open_fn
/// @return the total size in bytes of the stream
pub type mpv_stream_cb_size_fn =
    ::std::option::Option<unsafe extern "C" fn(cookie: *mut c_void) -> i64>;
/// Close callback used to implement a custom stream.
///
/// @param cookie opaque cookie identifying the stream,
///               returned from mpv_stream_cb_open_fn
pub type mpv_stream_cb_close_fn = ::std::option::Option<unsafe extern "C" fn(cookie: *mut c_void)>;
/// Cancel callback used to implement a custom stream.
///
/// This callback is used to interrupt any current or future read and seek
/// operations. It will be called from a separate thread than the demux
/// thread, and should not block.
///
/// This callback can be NULL.
///
/// Available since API 1.106.
///
/// @param cookie opaque cookie identifying the stream,
///               returned from mpv_stream_cb_open_fn
pub type mpv_stream_cb_cancel_fn = ::std::option::Option<unsafe extern "C" fn(cookie: *mut c_void)>;
/// See mpv_stream_cb_open_ro_fn callback.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpv_stream_cb_info {
    /// Opaque user-provided value, which will be passed to the other callbacks.
    /// The close callback will be called to release the cookie. It is not
    /// interpreted by mpv. It doesn't even need to be a valid pointer.
    ///
    /// The user sets this in the mpv_stream_cb_open_ro_fn callback.
    pub cookie: *mut c_void,
    /// Callbacks set by the user in the mpv_stream_cb_open_ro_fn callback. Some
    /// of them are optional, and can be left unset.
    ///
    /// The following callbacks are mandatory: read_fn, close_fn
    pub read_fn: mpv_stream_cb_read_fn,
    pub seek_fn: mpv_stream_cb_seek_fn,
    pub size_fn: mpv_stream_cb_size_fn,
    pub close_fn: mpv_stream_cb_close_fn,
    pub cancel_fn: mpv_stream_cb_cancel_fn,
}
/// Open callback used to implement a custom read-only (ro) stream. The user
/// must set the callback fields in the passed info struct. The cookie field
/// also can be set to store state associated to the stream instance.
///
/// Note that the info struct is valid only for the duration of this callback.
/// You can't change the callbacks or the pointer to the cookie at a later point.
///
/// Each stream instance created by the open callback can have different
/// callbacks.
///
/// The close_fn callback will terminate the stream instance. The pointers to
/// your callbacks and cookie will be discarded, and the callbacks will not be
/// called again.
///
/// @param user_data opaque user data provided via mpv_stream_cb_add()
/// @param uri name of the stream to be opened (with protocol prefix)
/// @param info fields which the user should fill
/// @return 0 on success, MPV_ERROR_LOADING_FAILED if the URI cannot be opened.
pub type mpv_stream_cb_open_ro_fn = ::std::option::Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        uri: *mut c_char,
        info: *mut mpv_stream_cb_info,
    ) -> c_int,
>;
pub type __builtin_va_list = *mut c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _drmModeAtomicReq {
    pub _address: u8,
}
