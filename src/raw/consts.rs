#![allow(non_upper_case_globals)]

use super::types::*;

pub const MPV_ENABLE_DEPRECATED: u32 = 1;
pub const MPV_RENDER_API_TYPE_OPENGL: &[u8; 7] = b"opengl\0";
pub const MPV_RENDER_API_TYPE_SW: &[u8; 3] = b"sw\0";

/// No error happened (used to signal successful operation).
/// Keep in mind that many API functions returning error codes can also
/// return positive values, which also indicate success. API users can
/// hardcode the fact that \">= 0\" means success.
pub const mpv_error_MPV_ERROR_SUCCESS: mpv_error = 0;
/// The event ringbuffer is full. This means the client is choked, and can't
/// receive any events. This can happen when too many asynchronous requests
/// have been made, but not answered. Probably never happens in practice,
/// unless the mpv core is frozen for some reason, and the client keeps
/// making asynchronous requests. (Bugs in the client API implementation
/// could also trigger this, e.g. if events become \"lost\".)
pub const mpv_error_MPV_ERROR_EVENT_QUEUE_FULL: mpv_error = -1;
/// Memory allocation failed.
pub const mpv_error_MPV_ERROR_NOMEM: mpv_error = -2;
/// The mpv core wasn't configured and initialized yet. See the notes in
/// mpv_create().
pub const mpv_error_MPV_ERROR_UNINITIALIZED: mpv_error = -3;
/// Generic catch-all error if a parameter is set to an invalid or
/// unsupported value. This is used if there is no better error code.
pub const mpv_error_MPV_ERROR_INVALID_PARAMETER: mpv_error = -4;
/// Trying to set an option that doesn't exist.
pub const mpv_error_MPV_ERROR_OPTION_NOT_FOUND: mpv_error = -5;
/// Trying to set an option using an unsupported MPV_FORMAT.
pub const mpv_error_MPV_ERROR_OPTION_FORMAT: mpv_error = -6;
/// Setting the option failed. Typically this happens if the provided option
/// value could not be parsed.
pub const mpv_error_MPV_ERROR_OPTION_ERROR: mpv_error = -7;
/// The accessed property doesn't exist.
pub const mpv_error_MPV_ERROR_PROPERTY_NOT_FOUND: mpv_error = -8;
/// Trying to set or get a property using an unsupported MPV_FORMAT.
pub const mpv_error_MPV_ERROR_PROPERTY_FORMAT: mpv_error = -9;
/// The property exists, but is not available. This usually happens when the
/// associated subsystem is not active, e.g. querying audio parameters while
/// audio is disabled.
pub const mpv_error_MPV_ERROR_PROPERTY_UNAVAILABLE: mpv_error = -10;
/// Error setting or getting a property.
pub const mpv_error_MPV_ERROR_PROPERTY_ERROR: mpv_error = -11;
/// General error when running a command with mpv_command and similar.
pub const mpv_error_MPV_ERROR_COMMAND: mpv_error = -12;
/// Generic error on loading (usually used with mpv_event_end_file.error).
pub const mpv_error_MPV_ERROR_LOADING_FAILED: mpv_error = -13;
/// Initializing the audio output failed.
pub const mpv_error_MPV_ERROR_AO_INIT_FAILED: mpv_error = -14;
/// Initializing the video output failed.
pub const mpv_error_MPV_ERROR_VO_INIT_FAILED: mpv_error = -15;
/// There was no audio or video data to play. This also happens if the
/// file was recognized, but did not contain any audio or video streams,
/// or no streams were selected.
pub const mpv_error_MPV_ERROR_NOTHING_TO_PLAY: mpv_error = -16;
/// When trying to load the file, the file format could not be determined,
/// or the file was too broken to open it.
pub const mpv_error_MPV_ERROR_UNKNOWN_FORMAT: mpv_error = -17;
/// Generic error for signaling that certain system requirements are not
/// fulfilled.
pub const mpv_error_MPV_ERROR_UNSUPPORTED: mpv_error = -18;
/// The API function which was called is a stub only.
pub const mpv_error_MPV_ERROR_NOT_IMPLEMENTED: mpv_error = -19;
/// Unspecified error.
pub const mpv_error_MPV_ERROR_GENERIC: mpv_error = -20;

/// Invalid. Sometimes used for empty values. This is always defined to 0,
/// so a normal 0-init of mpv_format (or e.g. mpv_node) is guaranteed to set
/// this it to MPV_FORMAT_NONE (which makes some things saner as consequence).
pub const mpv_format_MPV_FORMAT_NONE: mpv_format = 0;
/// The basic type is char*. It returns the raw property string, like
/// using ${=property} in input.conf (see input.rst).
///
/// NULL isn't an allowed value.
///
/// Warning: although the encoding is usually UTF-8, this is not always the
///          case. File tags often store strings in some legacy codepage,
///          and even filenames don't necessarily have to be in UTF-8 (at
///          least on Linux). If you pass the strings to code that requires
///          valid UTF-8, you have to sanitize it in some way.
///          On Windows, filenames are always UTF-8, and libmpv converts
///          between UTF-8 and UTF-16 when using win32 API functions. See
///          the "Encoding of filenames" section for details.
///
/// Example for reading:
///
///     char *result = NULL;
///     if (mpv_get_property(ctx, \"property\", MPV_FORMAT_STRING, &result) < 0)
///         goto error;
///     printf(\"%s\\n\", result);
///     mpv_free(result);
///
/// Or just use mpv_get_property_string().
///
/// Example for writing:
///
///     char *value = \"the new value\";
///     // yep, you pass the address to the variable
///     // (needed for symmetry with other types and mpv_get_property)
///     mpv_set_property(ctx, \"property\", MPV_FORMAT_STRING, &value);
///
/// Or just use mpv_set_property_string().
pub const mpv_format_MPV_FORMAT_STRING: mpv_format = 1;
/// The basic type is char*. It returns the OSD property string, like
/// using ${property} in input.conf (see input.rst). In many cases, this
/// is the same as the raw string, but in other cases it's formatted for
/// display on OSD. It's intended to be human readable. Do not attempt to
/// parse these strings.
///
/// Only valid when doing read access. The rest works like MPV_FORMAT_STRING.
pub const mpv_format_MPV_FORMAT_OSD_STRING: mpv_format = 2;
/// The basic type is int. The only allowed values are 0 (\"no\")
/// and 1 (\"yes\").
///
/// Example for reading:
///
///     int result;
///     if (mpv_get_property(ctx, \"property\", MPV_FORMAT_FLAG, &result) < 0)
///         goto error;
///     printf(\"%s\\n\", result ? \"true\" : \"false\");
///
/// Example for writing:
///
///     int flag = 1;
///     mpv_set_property(ctx, \"property\", MPV_FORMAT_FLAG, &flag);
pub const mpv_format_MPV_FORMAT_FLAG: mpv_format = 3;
/// The basic type is int64_t.
pub const mpv_format_MPV_FORMAT_INT64: mpv_format = 4;
/// The basic type is double.
pub const mpv_format_MPV_FORMAT_DOUBLE: mpv_format = 5;
/// The type is mpv_node.
///
/// For reading, you usually would pass a pointer to a stack-allocated
/// mpv_node value to mpv, and when you're done you call
/// mpv_free_node_contents(&node).
/// You're expected not to write to the data - if you have to, copy it
/// first (which you have to do manually).
///
/// For writing, you construct your own mpv_node, and pass a pointer to the
/// API. The API will never write to your data (and copy it if needed), so
/// you're free to use any form of allocation or memory management you like.
///
/// Warning: when reading, always check the mpv_node.format member. For
///          example, properties might change their type in future versions
///          of mpv, or sometimes even during runtime.
///
/// Example for reading:
///
///     mpv_node result;
///     if (mpv_get_property(ctx, \"property\", MPV_FORMAT_NODE, &result) < 0)
///         goto error;
///     printf(\"format=%d\\n\", (int)result.format);
///     mpv_free_node_contents(&result).
///
/// Example for writing:
///
///     mpv_node value;
///     value.format = MPV_FORMAT_STRING;
///     value.u.string = \"hello\";
///     mpv_set_property(ctx, \"property\", MPV_FORMAT_NODE, &value);
pub const mpv_format_MPV_FORMAT_NODE: mpv_format = 6;
/// Used with mpv_node only. Can usually not be used directly.
pub const mpv_format_MPV_FORMAT_NODE_ARRAY: mpv_format = 7;
/// See MPV_FORMAT_NODE_ARRAY.
pub const mpv_format_MPV_FORMAT_NODE_MAP: mpv_format = 8;
/// A raw, untyped byte array. Only used only with mpv_node, and only in
/// some very specific situations. (Some commands use it.)
pub const mpv_format_MPV_FORMAT_BYTE_ARRAY: mpv_format = 9;

/// Nothing happened. Happens on timeouts or sporadic wakeups.
pub const mpv_event_id_MPV_EVENT_NONE: mpv_event_id = 0;
/// Happens when the player quits. The player enters a state where it tries
/// to disconnect all clients. Most requests to the player will fail, and
/// the client should react to this and quit with mpv_destroy() as soon as
/// possible.
pub const mpv_event_id_MPV_EVENT_SHUTDOWN: mpv_event_id = 1;
/// See mpv_request_log_messages().
pub const mpv_event_id_MPV_EVENT_LOG_MESSAGE: mpv_event_id = 2;
/// Reply to a mpv_get_property_async() request.
/// See also mpv_event and mpv_event_property.
pub const mpv_event_id_MPV_EVENT_GET_PROPERTY_REPLY: mpv_event_id = 3;
/// Reply to a mpv_set_property_async() request.
/// (Unlike MPV_EVENT_GET_PROPERTY, mpv_event_property is not used.)
pub const mpv_event_id_MPV_EVENT_SET_PROPERTY_REPLY: mpv_event_id = 4;
/// Reply to a mpv_command_async() or mpv_command_node_async() request.
/// See also mpv_event and mpv_event_command.
pub const mpv_event_id_MPV_EVENT_COMMAND_REPLY: mpv_event_id = 5;
/// Notification before playback start of a file (before the file is loaded).
/// See also mpv_event and mpv_event_start_file.
pub const mpv_event_id_MPV_EVENT_START_FILE: mpv_event_id = 6;
/// Notification after playback end (after the file was unloaded).
/// See also mpv_event and mpv_event_end_file.
pub const mpv_event_id_MPV_EVENT_END_FILE: mpv_event_id = 7;
/// Notification when the file has been loaded (headers were read etc.), and
/// decoding starts.
pub const mpv_event_id_MPV_EVENT_FILE_LOADED: mpv_event_id = 8;
/// Idle mode was entered. In this mode, no file is played, and the playback
/// core waits for new commands. (The command line player normally quits
/// instead of entering idle mode, unless --idle was specified. If mpv
/// was started with mpv_create(), idle mode is enabled by default.)
///
/// @deprecated This is equivalent to using mpv_observe_property() on the
///             \"idle-active\" property. The event is redundant, and might be
///             removed in the far future. As a further warning, this event
///             is not necessarily sent at the right point anymore (at the
///             start of the program), while the property behaves correctly.
pub const mpv_event_id_MPV_EVENT_IDLE: mpv_event_id = 11;
/// Sent every time after a video frame is displayed. Note that currently,
/// this will be sent in lower frequency if there is no video, or playback
/// is paused - but that will be removed in the future, and it will be
/// restricted to video frames only.
///
/// @deprecated Use mpv_observe_property() with relevant properties instead
///             (such as \"playback-time\").
pub const mpv_event_id_MPV_EVENT_TICK: mpv_event_id = 14;
/// Triggered by the script-message input command. The command uses the
/// first argument of the command as client name (see mpv_client_name()) to
/// dispatch the message, and passes along all arguments starting from the
/// second argument as strings.
/// See also mpv_event and mpv_event_client_message.
pub const mpv_event_id_MPV_EVENT_CLIENT_MESSAGE: mpv_event_id = 16;
/// Happens after video changed in some way. This can happen on resolution
/// changes, pixel format changes, or video filter changes. The event is
/// sent after the video filters and the VO are reconfigured. Applications
/// embedding a mpv window should listen to this event in order to resize
/// the window if needed.
/// Note that this event can happen sporadically, and you should check
/// yourself whether the video parameters really changed before doing
/// something expensive.
pub const mpv_event_id_MPV_EVENT_VIDEO_RECONFIG: mpv_event_id = 17;
/// Similar to MPV_EVENT_VIDEO_RECONFIG. This is relatively uninteresting,
/// because there is no such thing as audio output embedding.
pub const mpv_event_id_MPV_EVENT_AUDIO_RECONFIG: mpv_event_id = 18;
/// Happens when a seek was initiated. Playback stops. Usually it will
/// resume with MPV_EVENT_PLAYBACK_RESTART as soon as the seek is finished.
pub const mpv_event_id_MPV_EVENT_SEEK: mpv_event_id = 20;
/// There was a discontinuity of some sort (like a seek), and playback
/// was reinitialized. Usually happens on start of playback and after
/// seeking. The main purpose is allowing the client to detect when a seek
/// request is finished.
pub const mpv_event_id_MPV_EVENT_PLAYBACK_RESTART: mpv_event_id = 21;
/// Event sent due to mpv_observe_property().
/// See also mpv_event and mpv_event_property.
pub const mpv_event_id_MPV_EVENT_PROPERTY_CHANGE: mpv_event_id = 22;
/// Happens if the internal per-mpv_handle ringbuffer overflows, and at
/// least 1 event had to be dropped. This can happen if the client doesn't
/// read the event queue quickly enough with mpv_wait_event(), or if the
/// client makes a very large number of asynchronous calls at once.
///
/// Event delivery will continue normally once this event was returned
/// (this forces the client to empty the queue completely).
pub const mpv_event_id_MPV_EVENT_QUEUE_OVERFLOW: mpv_event_id = 24;
/// Triggered if a hook handler was registered with mpv_hook_add(), and the
/// hook is invoked. If you receive this, you must handle it, and continue
/// the hook with mpv_hook_continue().
/// See also mpv_event and mpv_event_hook.
pub const mpv_event_id_MPV_EVENT_HOOK: mpv_event_id = 25;

pub const mpv_log_level_MPV_LOG_LEVEL_NONE: mpv_log_level = 0;
/// \"no\"    - disable absolutely all messages
pub const mpv_log_level_MPV_LOG_LEVEL_FATAL: mpv_log_level = 10;
/// \"fatal\" - critical/aborting errors
pub const mpv_log_level_MPV_LOG_LEVEL_ERROR: mpv_log_level = 20;
/// \"error\" - simple errors
pub const mpv_log_level_MPV_LOG_LEVEL_WARN: mpv_log_level = 30;
/// \"warn\"  - possible problems
pub const mpv_log_level_MPV_LOG_LEVEL_INFO: mpv_log_level = 40;
/// \"info\"  - informational message
pub const mpv_log_level_MPV_LOG_LEVEL_V: mpv_log_level = 50;
/// \"v\"     - noisy informational message
pub const mpv_log_level_MPV_LOG_LEVEL_DEBUG: mpv_log_level = 60;
/// \"debug\" - very noisy technical information
pub const mpv_log_level_MPV_LOG_LEVEL_TRACE: mpv_log_level = 70;

/// The end of file was reached. Sometimes this may also happen on
/// incomplete or corrupted files, or if the network connection was
/// interrupted when playing a remote file. It also happens if the
/// playback range was restricted with --end or --frames or similar.
pub const mpv_end_file_reason_MPV_END_FILE_REASON_EOF: mpv_end_file_reason = 0;
/// Playback was stopped by an external action (e.g. playlist controls).
pub const mpv_end_file_reason_MPV_END_FILE_REASON_STOP: mpv_end_file_reason = 2;
/// Playback was stopped by the quit command or player shutdown.
pub const mpv_end_file_reason_MPV_END_FILE_REASON_QUIT: mpv_end_file_reason = 3;
/// Some kind of error happened that lead to playback abort. Does not
/// necessarily happen on incomplete or broken files (in these cases, both
/// MPV_END_FILE_REASON_ERROR or MPV_END_FILE_REASON_EOF are possible).
///
/// mpv_event_end_file.error will be set.
pub const mpv_end_file_reason_MPV_END_FILE_REASON_ERROR: mpv_end_file_reason = 4;
/// The file was a playlist or similar. When the playlist is read, its
/// entries will be appended to the playlist after the entry of the current
/// file, the entry of the current file is removed, and a MPV_EVENT_END_FILE
/// event is sent with reason set to MPV_END_FILE_REASON_REDIRECT. Then
/// playback continues with the playlist contents.
/// Since API version 1.18.
pub const mpv_end_file_reason_MPV_END_FILE_REASON_REDIRECT: mpv_end_file_reason = 5;

/// Not a valid value, but also used to terminate a params array. Its value
/// is always guaranteed to be 0 (even if the ABI changes in the future).
pub const mpv_render_param_type_MPV_RENDER_PARAM_INVALID: mpv_render_param_type = 0;
/// The render API to use. Valid for mpv_render_context_create().
///
/// Type: char*
///
/// Defined APIs:
///
///   MPV_RENDER_API_TYPE_OPENGL:
///      OpenGL desktop 2.1 or later (preferably core profile compatible to
///      OpenGL 3.2), or OpenGLES 2.0 or later.
///      Providing MPV_RENDER_PARAM_OPENGL_INIT_PARAMS is required.
///      It is expected that an OpenGL context is valid and \"current\" when
///      calling mpv_render_* functions (unless specified otherwise). It
///      must be the same context for the same mpv_render_context.
pub const mpv_render_param_type_MPV_RENDER_PARAM_API_TYPE: mpv_render_param_type = 1;
/// Required parameters for initializing the OpenGL renderer. Valid for
/// mpv_render_context_create().
/// Type: mpv_opengl_init_params*
pub const mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_INIT_PARAMS: mpv_render_param_type = 2;
/// Describes a GL render target. Valid for mpv_render_context_render().
/// Type: mpv_opengl_fbo*
pub const mpv_render_param_type_MPV_RENDER_PARAM_OPENGL_FBO: mpv_render_param_type = 3;
/// Control flipped rendering. Valid for mpv_render_context_render().
/// Type: int*
/// If the value is set to 0, render normally. Otherwise, render it flipped,
/// which is needed e.g. when rendering to an OpenGL default framebuffer
/// (which has a flipped coordinate system).
pub const mpv_render_param_type_MPV_RENDER_PARAM_FLIP_Y: mpv_render_param_type = 4;
/// Control surface depth. Valid for mpv_render_context_render().
/// Type: int*
/// This implies the depth of the surface passed to the render function in
/// bits per channel. If omitted or set to 0, the renderer will assume 8.
/// Typically used to control dithering.
pub const mpv_render_param_type_MPV_RENDER_PARAM_DEPTH: mpv_render_param_type = 5;
/// ICC profile blob. Valid for mpv_render_context_set_parameter().
/// Type: mpv_byte_array*
/// Set an ICC profile for use with the \"icc-profile-auto\" option. (If the
/// option is not enabled, the ICC data will not be used.)
pub const mpv_render_param_type_MPV_RENDER_PARAM_ICC_PROFILE: mpv_render_param_type = 6;
/// Ambient light in lux. Valid for mpv_render_context_set_parameter().
/// Type: int*
/// This can be used for automatic gamma correction.
pub const mpv_render_param_type_MPV_RENDER_PARAM_AMBIENT_LIGHT: mpv_render_param_type = 7;
/// X11 Display, sometimes used for hwdec. Valid for
/// mpv_render_context_create(). The Display must stay valid for the lifetime
/// of the mpv_render_context.
/// Type: Display*
pub const mpv_render_param_type_MPV_RENDER_PARAM_X11_DISPLAY: mpv_render_param_type = 8;
/// Wayland display, sometimes used for hwdec. Valid for
/// mpv_render_context_create(). The wl_display must stay valid for the
/// lifetime of the mpv_render_context.
/// Type: struct wl_display*
pub const mpv_render_param_type_MPV_RENDER_PARAM_WL_DISPLAY: mpv_render_param_type = 9;
/// Better control about rendering and enabling some advanced features. Valid
/// for mpv_render_context_create().
///
/// This conflates multiple requirements the API user promises to abide if
/// this option is enabled:
///
///  - The API user's render thread, which is calling the mpv_render_*()
///    functions, never waits for the core. Otherwise deadlocks can happen.
///    See \"Threading\" section.
///  - The callback set with mpv_render_context_set_update_callback() can now
///    be called even if there is no new frame. The API user should call the
///    mpv_render_context_update() function, and interpret the return value
///    for whether a new frame should be rendered.
///  - Correct functionality is impossible if the update callback is not set,
///    or not set soon enough after mpv_render_context_create() (the core can
///    block while waiting for you to call mpv_render_context_update(), and
///    if the update callback is not correctly set, it will deadlock, or
///    block for too long).
///
/// In general, setting this option will enable the following features (and
/// possibly more):
///
///  - \"Direct rendering\", which means the player decodes directly to a
///    texture, which saves a copy per video frame (\"vd-lavc-dr\" option
///    needs to be enabled, and the rendering backend as well as the
///    underlying GPU API/driver needs to have support for it).
///  - Rendering screenshots with the GPU API if supported by the backend
///    (instead of using a suboptimal software fallback via libswscale).
///
/// Warning: do not just add this without reading the \"Threading\" section
///          above, and then wondering that deadlocks happen. The
///          requirements are tricky. But also note that even if advanced
///          control is disabled, not adhering to the rules will lead to
///          playback problems. Enabling advanced controls simply makes
///          violating these rules fatal.
///
/// Type: int*: 0 for disable (default), 1 for enable
pub const mpv_render_param_type_MPV_RENDER_PARAM_ADVANCED_CONTROL: mpv_render_param_type = 10;
/// Return information about the next frame to render. Valid for
/// mpv_render_context_get_info().
///
/// Type: mpv_render_frame_info*
///
/// It strictly returns information about the _next_ frame. The implication
/// is that e.g. mpv_render_context_update()'s return value will have
/// MPV_RENDER_UPDATE_FRAME set, and the user is supposed to call
/// mpv_render_context_render(). If there is no next frame, then the
/// return value will have is_valid set to 0.
pub const mpv_render_param_type_MPV_RENDER_PARAM_NEXT_FRAME_INFO: mpv_render_param_type = 11;
/// Enable or disable video timing. Valid for mpv_render_context_render().
///
/// Type: int*: 0 for disable, 1 for enable (default)
///
/// When video is timed to audio, the player attempts to render video a bit
/// ahead, and then do a blocking wait until the target display time is
/// reached. This blocks mpv_render_context_render() for up to the amount
/// specified with the \"video-timing-offset\" global option. You can set
/// this parameter to 0 to disable this kind of waiting. If you do, it's
/// recommended to use the target time value in mpv_render_frame_info to
/// wait yourself, or to set the \"video-timing-offset\" to 0 instead.
///
/// Disabling this without doing anything in addition will result in A/V sync
/// being slightly off.
pub const mpv_render_param_type_MPV_RENDER_PARAM_BLOCK_FOR_TARGET_TIME: mpv_render_param_type = 12;
/// Use to skip rendering in mpv_render_context_render().
///
/// Type: int*: 0 for rendering (default), 1 for skipping
///
/// If this is set, you don't need to pass a target surface to the render
/// function (and if you do, it's completely ignored). This can still call
/// into the lower level APIs (i.e. if you use OpenGL, the OpenGL context
/// must be set).
///
/// Be aware that the render API will consider this frame as having been
/// rendered. All other normal rules also apply, for example about whether
/// you have to call mpv_render_context_report_swap(). It also does timing
/// in the same way.
pub const mpv_render_param_type_MPV_RENDER_PARAM_SKIP_RENDERING: mpv_render_param_type = 13;
/// Deprecated. Not supported. Use MPV_RENDER_PARAM_DRM_DISPLAY_V2 instead.
/// Type : struct mpv_opengl_drm_params*
pub const mpv_render_param_type_MPV_RENDER_PARAM_DRM_DISPLAY: mpv_render_param_type = 14;
/// DRM draw surface size, contains draw surface dimensions.
/// Valid for mpv_render_context_create().
/// Type : struct mpv_opengl_drm_draw_surface_size*
pub const mpv_render_param_type_MPV_RENDER_PARAM_DRM_DRAW_SURFACE_SIZE: mpv_render_param_type = 15;
/// DRM display, contains drm display handles.
/// Valid for mpv_render_context_create().
/// Type : struct mpv_opengl_drm_params_v2*
pub const mpv_render_param_type_MPV_RENDER_PARAM_DRM_DISPLAY_V2: mpv_render_param_type = 16;
/// MPV_RENDER_API_TYPE_SW only: rendering target surface size, mandatory.
/// Valid for MPV_RENDER_API_TYPE_SW & mpv_render_context_render().
/// Type: int[2] (e.g.: int s[2] = {w, h}; param.data = &s[0];)
///
/// The video frame is transformed as with other VOs. Typically, this means
/// the video gets scaled and black bars are added if the video size or
/// aspect ratio mismatches with the target size.
pub const mpv_render_param_type_MPV_RENDER_PARAM_SW_SIZE: mpv_render_param_type = 17;
/// MPV_RENDER_API_TYPE_SW only: rendering target surface pixel format,
/// mandatory.
/// Valid for MPV_RENDER_API_TYPE_SW & mpv_render_context_render().
/// Type: char* (e.g.: char *f = \"rgb0\"; param.data = f;)
///
/// Valid values are:
///  \"rgb0\", \"bgr0\", \"0bgr\", \"0rgb\"
///      4 bytes per pixel RGB, 1 byte (8 bit) per component, component bytes
///      with increasing address from left to right (e.g. \"rgb0\" has r at
///      address 0), the \"0\" component contains uninitialized garbage (often
///      the value 0, but not necessarily; the bad naming is inherited from
///      FFmpeg)
///      Pixel alignment size: 4 bytes
///  \"rgb24\"
///      3 bytes per pixel RGB. This is strongly discouraged because it is
///      very slow.
///      Pixel alignment size: 1 bytes
///  other
///      The API may accept other pixel formats, using mpv internal format
///      names, as long as it's internally marked as RGB, has exactly 1
///      plane, and is supported as conversion output. It is not a good idea
///      to rely on any of these. Their semantics and handling could change.
pub const mpv_render_param_type_MPV_RENDER_PARAM_SW_FORMAT: mpv_render_param_type = 18;
/// MPV_RENDER_API_TYPE_SW only: rendering target surface bytes per line,
/// mandatory.
/// Valid for MPV_RENDER_API_TYPE_SW & mpv_render_context_render().
/// Type: size_t*
///
/// This is the number of bytes between a pixel (x, y) and (x, y + 1) on the
/// target surface. It must be a multiple of the pixel size, and have space
/// for the surface width as specified by MPV_RENDER_PARAM_SW_SIZE.
///
/// Both stride and pointer value should be a multiple of 64 to facilitate
/// fast SIMD operation. Lower alignment might trigger slower code paths,
/// and in the worst case, will copy the entire target frame. If mpv is built
/// with zimg (and zimg is not disabled), the performance impact might be
/// less.
/// In either cases, the pointer and stride must be aligned at least to the
/// pixel alignment size. Otherwise, crashes and undefined behavior is
/// possible on platforms which do not support unaligned accesses (either
/// through normal memory access or aligned SIMD memory access instructions).
pub const mpv_render_param_type_MPV_RENDER_PARAM_SW_STRIDE: mpv_render_param_type = 19;
/// MPV_RENDER_API_TYPE_SW only: rendering target surface bytes per line,
/// mandatory.
/// Valid for MPV_RENDER_API_TYPE_SW & mpv_render_context_render().
/// Type: size_t*
///
/// This is the number of bytes between a pixel (x, y) and (x, y + 1) on the
/// target surface. It must be a multiple of the pixel size, and have space
/// for the surface width as specified by MPV_RENDER_PARAM_SW_SIZE.
///
/// Both stride and pointer value should be a multiple of 64 to facilitate
/// fast SIMD operation. Lower alignment might trigger slower code paths,
/// and in the worst case, will copy the entire target frame. If mpv is built
/// with zimg (and zimg is not disabled), the performance impact might be
/// less.
/// In either cases, the pointer and stride must be aligned at least to the
/// pixel alignment size. Otherwise, crashes and undefined behavior is
/// possible on platforms which do not support unaligned accesses (either
/// through normal memory access or aligned SIMD memory access instructions).
pub const mpv_render_param_type_MPV_RENDER_PARAM_SW_POINTER: mpv_render_param_type = 20;

/// Set if there is actually a next frame. If unset, there is no next frame
/// yet, and other flags and fields that require a frame to be queued will
/// be unset.
///
/// This is set for _any_ kind of frame, even for redraw requests.
///
/// Note that when this is unset, it simply means no new frame was
/// decoded/queued yet, not necessarily that the end of the video was
/// reached. A new frame can be queued after some time.
///
/// If the return value of mpv_render_context_render() had the
/// MPV_RENDER_UPDATE_FRAME flag set, this flag will usually be set as well,
/// unless the frame is rendered, or discarded by other asynchronous events.
pub const mpv_render_frame_info_flag_MPV_RENDER_FRAME_INFO_PRESENT: mpv_render_frame_info_flag = 1;
/// If set, the frame is not an actual new video frame, but a redraw request.
/// For example if the video is paused, and an option that affects video
/// rendering was changed (or any other reason), an update request can be
/// issued and this flag will be set.
///
/// Typically, redraw frames will not be subject to video timing.
///
/// Implies MPV_RENDER_FRAME_INFO_PRESENT.
pub const mpv_render_frame_info_flag_MPV_RENDER_FRAME_INFO_REDRAW: mpv_render_frame_info_flag = 2;
/// If set, this is supposed to reproduce the previous frame perfectly. This
/// is usually used for certain \"video-sync\" options (\"display-...\" modes).
/// Typically the renderer will blit the video from a FBO. Unset otherwise.
///
/// Implies MPV_RENDER_FRAME_INFO_PRESENT.
pub const mpv_render_frame_info_flag_MPV_RENDER_FRAME_INFO_REPEAT: mpv_render_frame_info_flag = 4;
/// If set, the player timing code expects that the user thread blocks on
/// vsync (by either delaying the render call, or by making a call to
/// mpv_render_context_report_swap() at vsync time).
///
/// Implies MPV_RENDER_FRAME_INFO_PRESENT.
pub const mpv_render_frame_info_flag_MPV_RENDER_FRAME_INFO_BLOCK_VSYNC: mpv_render_frame_info_flag =
    8;

/// A new video frame must be rendered. mpv_render_context_render() must be
/// called.
pub const mpv_render_update_flag_MPV_RENDER_UPDATE_FRAME: mpv_render_update_flag = 1;
