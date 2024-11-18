use crate::raw::*;

use super::util::make_rust_string_const;

#[derive(Debug, Clone, Copy)]
pub enum MpvError {
    /// The event ringbuffer is full. This means the client is choked, and can't
    /// receive any events. This can happen when too many asynchronous requests
    /// have been made, but not answered. Probably never happens in practice,
    /// unless the mpv core is frozen for some reason, and the client keeps
    /// making asynchronous requests. (Bugs in the client API implementation
    /// could also trigger this, e.g. if events become \"lost\".)
    EventQueueFull,
    /// Memory allocation failed.
    NoMemory,
    /// The mpv core wasn't configured and initialized yet. See the notes in
    /// mpv_create().
    Uninitialized,
    /// Generic catch-all error if a parameter is set to an invalid or
    /// unsupported value. This is used if there is no better error code.
    InvalidParameter,
    /// Trying to set an option that doesn't exist.
    OptionNotFound,
    /// Trying to set an option using an unsupported MPV_FORMAT.
    OptionFormatUnsupported,
    /// Setting the option failed. Typically this happens if the provided option
    /// value could not be parsed.
    OptionError,
    /// The accessed property doesn't exist.
    PropertyNotFound,
    /// Trying to set or get a property using an unsupported MPV_FORMAT.
    PropertyNotSupported,
    /// The property exists, but is not available. This usually happens when the
    /// associated subsystem is not active, e.g. querying audio parameters while
    /// audio is disabled.
    PropertyUnavailable,
    /// Error setting or getting a property.
    PropertyError,
    /// General error when running a command with mpv_command and similar.
    CommandError,
    /// Generic error on loading (usually used with mpv_event_end_file.error).
    LoadingFailed,
    /// Initializing the audio output failed.
    AudioOutputInitFailed,
    /// Initializing the video output failed.
    VideoOutputInitFailed,
    /// There was no audio or video data to play. This also happens if the
    /// file was recognized, but did not contain any audio or video streams,
    /// or no streams were selected.
    NothingToPlay,
    /// When trying to load the file, the file format could not be determined,
    /// or the file was too broken to open it.
    UnknownFormat,
    /// Generic error for signaling that certain system requirements are not
    /// fulfilled.
    Unsupported,
    /// The API function which was called is a stub only.
    NotImplemented,
    /// Unspecified error.
    Unspecified,
}

impl MpvError {
    pub(crate) fn from_mpv_error(status: mpv_error) -> Option<Self> {
        match status {
            mpv_error_MPV_ERROR_SUCCESS => None,
            mpv_error_MPV_ERROR_EVENT_QUEUE_FULL => Some(Self::EventQueueFull),
            mpv_error_MPV_ERROR_NOMEM => Some(Self::NoMemory),
            mpv_error_MPV_ERROR_UNINITIALIZED => Some(Self::Uninitialized),
            mpv_error_MPV_ERROR_INVALID_PARAMETER => Some(Self::InvalidParameter),
            mpv_error_MPV_ERROR_OPTION_NOT_FOUND => Some(Self::OptionNotFound),
            mpv_error_MPV_ERROR_OPTION_FORMAT => Some(Self::OptionFormatUnsupported),
            mpv_error_MPV_ERROR_OPTION_ERROR => Some(Self::OptionError),
            mpv_error_MPV_ERROR_PROPERTY_NOT_FOUND => Some(Self::PropertyNotFound),
            mpv_error_MPV_ERROR_PROPERTY_FORMAT => Some(Self::PropertyNotSupported),
            mpv_error_MPV_ERROR_PROPERTY_UNAVAILABLE => Some(Self::PropertyUnavailable),
            mpv_error_MPV_ERROR_PROPERTY_ERROR => Some(Self::PropertyError),
            mpv_error_MPV_ERROR_COMMAND => Some(Self::CommandError),
            mpv_error_MPV_ERROR_LOADING_FAILED => Some(Self::LoadingFailed),
            mpv_error_MPV_ERROR_AO_INIT_FAILED => Some(Self::AudioOutputInitFailed),
            mpv_error_MPV_ERROR_VO_INIT_FAILED => Some(Self::VideoOutputInitFailed),
            mpv_error_MPV_ERROR_NOTHING_TO_PLAY => Some(Self::NothingToPlay),
            mpv_error_MPV_ERROR_UNKNOWN_FORMAT => Some(Self::UnknownFormat),
            mpv_error_MPV_ERROR_UNSUPPORTED => Some(Self::Unsupported),
            mpv_error_MPV_ERROR_NOT_IMPLEMENTED => Some(Self::NotImplemented),
            _ => Some(Self::Unspecified),
        }
    }

    /// Return a string describing the error. For unknown errors, the string
    /// "unknown error" is returned.
    pub fn get_error_string(&self) -> String {
        let error = match self {
            Self::EventQueueFull => mpv_error_MPV_ERROR_EVENT_QUEUE_FULL,
            Self::NoMemory => mpv_error_MPV_ERROR_NOMEM,
            Self::Uninitialized => mpv_error_MPV_ERROR_UNINITIALIZED,
            Self::InvalidParameter => mpv_error_MPV_ERROR_INVALID_PARAMETER,
            Self::OptionNotFound => mpv_error_MPV_ERROR_OPTION_NOT_FOUND,
            Self::OptionFormatUnsupported => mpv_error_MPV_ERROR_OPTION_FORMAT,
            Self::OptionError => mpv_error_MPV_ERROR_OPTION_ERROR,
            Self::PropertyNotFound => mpv_error_MPV_ERROR_PROPERTY_NOT_FOUND,
            Self::PropertyNotSupported => mpv_error_MPV_ERROR_PROPERTY_FORMAT,
            Self::PropertyUnavailable => mpv_error_MPV_ERROR_PROPERTY_UNAVAILABLE,
            Self::PropertyError => mpv_error_MPV_ERROR_PROPERTY_ERROR,
            Self::CommandError => mpv_error_MPV_ERROR_COMMAND,
            Self::LoadingFailed => mpv_error_MPV_ERROR_LOADING_FAILED,
            Self::AudioOutputInitFailed => mpv_error_MPV_ERROR_AO_INIT_FAILED,
            Self::VideoOutputInitFailed => mpv_error_MPV_ERROR_VO_INIT_FAILED,
            Self::NothingToPlay => mpv_error_MPV_ERROR_NOTHING_TO_PLAY,
            Self::UnknownFormat => mpv_error_MPV_ERROR_UNKNOWN_FORMAT,
            Self::Unsupported => mpv_error_MPV_ERROR_UNSUPPORTED,
            Self::NotImplemented => mpv_error_MPV_ERROR_NOT_IMPLEMENTED,
            Self::Unspecified => mpv_error_MPV_ERROR_GENERIC,
        };
        let raw = unsafe { mpv_error_string(error) };
        make_rust_string_const(raw).unwrap()
    }
}
