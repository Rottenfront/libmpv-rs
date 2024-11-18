use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use libc::free;

use crate::raw::*;

use super::{
    error::MpvError,
    node::{Node, Property},
    util::make_rust_string_const,
};

#[derive(Debug, Clone, Copy)]
pub enum EndFileReason {
    EOF,
    Stop,
    Quit,
    Error(MpvError),
    Redirect,
}

impl EndFileReason {
    pub(crate) fn from_mpv_end_file_reason(
        level: mpv_end_file_reason,
        error: Option<MpvError>,
    ) -> Option<Self> {
        Some(match level {
            mpv_end_file_reason_MPV_END_FILE_REASON_EOF => Self::EOF,
            mpv_end_file_reason_MPV_END_FILE_REASON_STOP => Self::Stop,
            mpv_end_file_reason_MPV_END_FILE_REASON_QUIT => Self::Quit,
            mpv_end_file_reason_MPV_END_FILE_REASON_ERROR => Self::Error(error.unwrap()),
            mpv_end_file_reason_MPV_END_FILE_REASON_REDIRECT => Self::Redirect,
            _ => return None,
        })
    }
}

/// Numeric log levels. The lower the number, the more important the message is.
/// MPV_LOG_LEVEL_NONE is never used when receiving messages. The string in
/// the comment after the value is the name of the log level as used for the
/// mpv_request_log_messages() function.
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    None,
    Fatal,
    Error,
    Warn,
    Info,
    Noise,
    Debug,
    Trace,
}

impl LogLevel {
    pub(crate) fn from_mpv_log_level(level: mpv_log_level) -> Option<Self> {
        Some(match level {
            mpv_log_level_MPV_LOG_LEVEL_NONE => Self::None,
            mpv_log_level_MPV_LOG_LEVEL_FATAL => Self::Fatal,
            mpv_log_level_MPV_LOG_LEVEL_ERROR => Self::Error,
            mpv_log_level_MPV_LOG_LEVEL_WARN => Self::Warn,
            mpv_log_level_MPV_LOG_LEVEL_INFO => Self::Info,
            mpv_log_level_MPV_LOG_LEVEL_V => Self::Noise,
            mpv_log_level_MPV_LOG_LEVEL_DEBUG => Self::Debug,
            mpv_log_level_MPV_LOG_LEVEL_TRACE => Self::Trace,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    /// Happens when the player quits. The player enters a state where it tries
    /// to disconnect all clients. Most requests to the player will fail, and
    /// the client should react to this and quit with mpv_destroy() as soon as
    /// possible.
    Shutdown,
    /// See mpv_request_log_messages().
    LogMessage {
        prefix: String,
        level: String,
        text: String,
        log_level: LogLevel,
    },
    /// Reply to a mpv_get_property_async() request.
    /// See also mpv_event and mpv_event_property.
    GetPropertyReply {
        result: Result<Option<Property>, MpvError>,
        reply_userdata: u64,
    },
    /// Reply to a mpv_set_property_async() request.
    /// (Unlike MPV_EVENT_GET_PROPERTY, mpv_event_property is not used.)
    SetPropertyReply {
        result: Result<Option<Property>, MpvError>,
        reply_userdata: u64,
    },
    /// Reply to a mpv_command_async() or mpv_command_node_async() request.
    /// See also mpv_event and mpv_event_command.
    CommandReply {
        result: Result<Option<Node>, MpvError>,
        reply_userdata: u64,
    },
    /// Notification before playback start of a file (before the file is loaded).
    /// See also mpv_event and mpv_event_start_file.
    StartFile {
        /// Playlist entry ID of the file being loaded now.
        playlist_entry_id: i64,
    },
    /// Notification after playback end (after the file was unloaded).
    /// See also mpv_event and mpv_event_end_file.
    EndFile {
        /// Corresponds to the values in enum mpv_end_file_reason.
        ///
        /// Unknown values should be treated as unknown.
        reason: EndFileReason,
        /// Playlist entry ID of the file that was being played or attempted to be
        /// played. This has the same value as the playlist_entry_id field in the
        /// corresponding mpv_event_start_file event.
        playlist_entry_id: i64,
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
        playlist_insert_id: i64,
        /// See playlist_insert_id. Only non-0 if playlist_insert_id is valid. Never
        /// negative.
        playlist_insert_num_entries: i64,
    },
    /// Notification when the file has been loaded (headers were read etc.), and
    /// decoding starts.
    FileLoaded,
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
    Idle,
    /// Triggered by the script-message input command. The command uses the
    /// first argument of the command as client name (see mpv_client_name()) to
    /// dispatch the message, and passes along all arguments starting from the
    /// second argument as strings.
    /// See also mpv_event and mpv_event_client_message.
    Tick,
    /// Triggered by the script-message input command. The command uses the
    /// first argument of the command as client name (see mpv_client_name()) to
    /// dispatch the message, and passes along all arguments starting from the
    /// second argument as strings.
    /// See also mpv_event and mpv_event_client_message.
    ClientMessage { args: Vec<String> },
    /// Happens after video changed in some way. This can happen on resolution
    /// changes, pixel format changes, or video filter changes. The event is
    /// sent after the video filters and the VO are reconfigured. Applications
    /// embedding a mpv window should listen to this event in order to resize
    /// the window if needed.
    /// Note that this event can happen sporadically, and you should check
    /// yourself whether the video parameters really changed before doing
    /// something expensive.
    VideoReconfig,
    /// Similar to MPV_EVENT_VIDEO_RECONFIG. This is relatively uninteresting,
    /// because there is no such thing as audio output embedding.
    AudioReconfig,
    /// Happens when a seek was initiated. Playback stops. Usually it will
    /// resume with MPV_EVENT_PLAYBACK_RESTART as soon as the seek is finished.
    Seek,
    /// There was a discontinuity of some sort (like a seek), and playback
    /// was reinitialized. Usually happens on start of playback and after
    /// seeking. The main purpose is allowing the client to detect when a seek
    /// request is finished.
    PlaybackRestart,
    /// Event sent due to mpv_observe_property().
    /// See also mpv_event and mpv_event_property.
    PropertyChange {
        result: Result<Option<Property>, MpvError>,
        reply_userdata: u64,
    },
    /// Happens if the internal per-mpv_handle ringbuffer overflows, and at
    /// least 1 event had to be dropped. This can happen if the client doesn't
    /// read the event queue quickly enough with mpv_wait_event(), or if the
    /// client makes a very large number of asynchronous calls at once.
    ///
    /// Event delivery will continue normally once this event was returned
    /// (this forces the client to empty the queue completely).
    QueueOverflow,
    /// Triggered if a hook handler was registered with mpv_hook_add(), and the
    /// hook is invoked. If you receive this, you must handle it, and continue
    /// the hook with mpv_hook_continue().
    /// See also mpv_event and mpv_event_hook.
    Hook {
        name: String,
        id: u64,
        reply_userdata: u64,
    },
}

impl Event {
    pub(crate) fn from_mpv_event(event: mpv_event) -> Option<Self> {
        let mpv_event {
            event_id,
            error,
            reply_userdata,
            data,
        } = event;
        // The meaning and contents of the data member depend on the event_id:
        //  MPV_EVENT_GET_PROPERTY_REPLY:     mpv_event_property*
        //  MPV_EVENT_PROPERTY_CHANGE:        mpv_event_property*
        //  MPV_EVENT_LOG_MESSAGE:            mpv_event_log_message*
        //  MPV_EVENT_CLIENT_MESSAGE:         mpv_event_client_message*
        //  MPV_EVENT_START_FILE:             mpv_event_start_file* (since v1.108)
        //  MPV_EVENT_END_FILE:               mpv_event_end_file*
        //  MPV_EVENT_HOOK:                   mpv_event_hook*
        //  MPV_EVENT_COMMAND_REPLY*          mpv_event_command*
        //  other: NULL
        let res = match event_id {
            mpv_event_id_MPV_EVENT_SHUTDOWN => Some(Self::Shutdown),
            mpv_event_id_MPV_EVENT_LOG_MESSAGE => {
                if data == null_mut() {
                    panic!("No data provided (log message event)");
                }
                let mpv_event_log_message {
                    prefix,
                    level,
                    text,
                    log_level,
                } = unsafe { *(data as *mut mpv_event_log_message) };
                Some(Self::LogMessage {
                    prefix: make_rust_string_const(prefix).unwrap(),
                    level: make_rust_string_const(level).unwrap(),
                    text: make_rust_string_const(text).unwrap(),
                    log_level: LogLevel::from_mpv_log_level(log_level).unwrap(),
                })
            }
            mpv_event_id_MPV_EVENT_GET_PROPERTY_REPLY => {
                let result = match MpvError::from_mpv_error(error) {
                    Some(err) => Err(err),
                    None => Ok(Property::from_mpv_property({
                        if data == null_mut() {
                            panic!("No data provided (get property reply event)");
                        }
                        unsafe { *(data as *mut _) }
                    })),
                };
                Some(Self::GetPropertyReply {
                    result,
                    reply_userdata,
                })
            }
            mpv_event_id_MPV_EVENT_SET_PROPERTY_REPLY => {
                let result = match MpvError::from_mpv_error(error) {
                    Some(err) => Err(err),
                    None => Ok(Property::from_mpv_property({
                        if data == null_mut() {
                            panic!("No data provided (set property reply event)");
                        }
                        unsafe { *(data as *mut _) }
                    })),
                };
                Some(Self::SetPropertyReply {
                    result,
                    reply_userdata,
                })
            }
            mpv_event_id_MPV_EVENT_COMMAND_REPLY => {
                let result = match MpvError::from_mpv_error(error) {
                    Some(err) => Err(err),
                    None => Ok(Node::from_mpv_node({
                        if data == null_mut() {
                            panic!("No data provided (command reply event)");
                        }
                        unsafe { *(data as *mut _) }
                    })),
                };
                Some(Self::CommandReply {
                    result,
                    reply_userdata,
                })
            }
            mpv_event_id_MPV_EVENT_START_FILE => {
                if data == null_mut() {
                    panic!("No data provided (start file event)");
                }
                let playlist_entry_id = unsafe { *(data as *mut _) };
                Some(Self::StartFile { playlist_entry_id })
            }
            mpv_event_id_MPV_EVENT_END_FILE => {
                if data == null_mut() {
                    panic!("No data provided (end file event)");
                }
                let mpv_event_end_file {
                    reason,
                    error,
                    playlist_entry_id,
                    playlist_insert_id,
                    playlist_insert_num_entries,
                } = unsafe { *(data as *mut _) };
                Some(Self::EndFile {
                    reason: EndFileReason::from_mpv_end_file_reason(
                        reason,
                        MpvError::from_mpv_error(error),
                    )
                    .unwrap(),
                    playlist_entry_id,
                    playlist_insert_id,
                    playlist_insert_num_entries: playlist_insert_num_entries as _,
                })
            }
            mpv_event_id_MPV_EVENT_FILE_LOADED => Some(Self::FileLoaded),
            mpv_event_id_MPV_EVENT_IDLE => Some(Self::Idle),
            mpv_event_id_MPV_EVENT_TICK => Some(Self::Tick),
            mpv_event_id_MPV_EVENT_CLIENT_MESSAGE => {
                if data == null_mut() {
                    panic!("No data provided (client message event)");
                }
                let mpv_event_client_message {
                    num_args,
                    args: arr,
                } = unsafe { *(data as *mut _) };
                let mut args = vec![];
                for i in 0..num_args as usize {
                    args.push(make_rust_string_const(unsafe { *(arr.add(i)) }).unwrap());
                }
                unsafe { mpv_free(arr as *mut _) };
                Some(Self::ClientMessage { args })
            }
            mpv_event_id_MPV_EVENT_VIDEO_RECONFIG => Some(Self::VideoReconfig),
            mpv_event_id_MPV_EVENT_AUDIO_RECONFIG => Some(Self::AudioReconfig),
            mpv_event_id_MPV_EVENT_SEEK => Some(Self::Seek),
            mpv_event_id_MPV_EVENT_PLAYBACK_RESTART => Some(Self::PlaybackRestart),
            mpv_event_id_MPV_EVENT_PROPERTY_CHANGE => {
                let result = match MpvError::from_mpv_error(error) {
                    Some(err) => Err(err),
                    None => Ok(Property::from_mpv_property({
                        if data == null_mut() {
                            panic!("No data provided (property change event)");
                        }
                        unsafe { *(data as *mut _) }
                    })),
                };
                Some(Self::PropertyChange {
                    result,
                    reply_userdata,
                })
            }
            mpv_event_id_MPV_EVENT_QUEUE_OVERFLOW => Some(Self::QueueOverflow),
            mpv_event_id_MPV_EVENT_HOOK => {
                if data == null_mut() {
                    panic!("No data provided (client message event)");
                }
                let mpv_event_hook { name, id } = unsafe { *(data as *mut _) };

                Some(Self::Hook {
                    name: make_rust_string_const(name).unwrap(),
                    id,
                    reply_userdata,
                })
            }
            _ => None,
        };
        if data != null_mut() {
            unsafe {
                mpv_free(data);
            }
        }
        res
    }

    pub fn get_event_string(&self) -> String {
        let event = match self {
            Event::Shutdown => mpv_event_id_MPV_EVENT_SHUTDOWN,
            Event::LogMessage { .. } => mpv_event_id_MPV_EVENT_LOG_MESSAGE,
            Event::GetPropertyReply { .. } => mpv_event_id_MPV_EVENT_GET_PROPERTY_REPLY,
            Event::SetPropertyReply { .. } => mpv_event_id_MPV_EVENT_SET_PROPERTY_REPLY,
            Event::CommandReply { .. } => mpv_event_id_MPV_EVENT_COMMAND_REPLY,
            Event::StartFile { .. } => mpv_event_id_MPV_EVENT_START_FILE,
            Event::EndFile { .. } => mpv_event_id_MPV_EVENT_END_FILE,
            Event::FileLoaded => mpv_event_id_MPV_EVENT_FILE_LOADED,
            Event::Idle => mpv_event_id_MPV_EVENT_IDLE,
            Event::Tick => mpv_event_id_MPV_EVENT_TICK,
            Event::ClientMessage { .. } => mpv_event_id_MPV_EVENT_CLIENT_MESSAGE,
            Event::VideoReconfig => mpv_event_id_MPV_EVENT_VIDEO_RECONFIG,
            Event::AudioReconfig => mpv_event_id_MPV_EVENT_AUDIO_RECONFIG,
            Event::Seek => mpv_event_id_MPV_EVENT_SEEK,
            Event::PlaybackRestart => mpv_event_id_MPV_EVENT_PLAYBACK_RESTART,
            Event::PropertyChange { .. } => mpv_event_id_MPV_EVENT_PROPERTY_CHANGE,
            Event::QueueOverflow => mpv_event_id_MPV_EVENT_QUEUE_OVERFLOW,
            Event::Hook { .. } => mpv_event_id_MPV_EVENT_HOOK,
        };
        let s = unsafe { mpv_event_name(event) };
        CString::from(unsafe { CStr::from_ptr(s) })
            .to_str()
            .unwrap()
            .to_owned()
    }
}
