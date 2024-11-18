use std::ffi::{CStr, CString};

pub fn make_rust_string(data: *mut i8) -> Option<String> {
    match unsafe { CString::from_raw(data) }.into_string() {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn make_rust_string_const(data: *const i8) -> Option<String> {
    match unsafe { CStr::from_ptr(data) }.to_owned().into_string() {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

pub fn make_c_string(s: String) -> Option<*mut i8> {
    let Ok(data) = CString::new(s) else {
        return None;
    };
    Some(data.into_raw())
}
