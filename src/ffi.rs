// extern C api
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn c_analyze(
    key: *const c_char,
    scale: *const c_char,
    extended: bool,
) -> *const c_char {
    let c_key = unsafe { CStr::from_ptr(key) };

    let c_scale = unsafe { CStr::from_ptr(scale) };

    CString::new(super::analyze_json(
        c_key.to_str().unwrap(),
        c_scale.to_str().unwrap(),
        extended,
    ))
    .unwrap()
    .into_raw()
}

#[no_mangle]
pub extern "C" fn c_scales() -> *const c_char {
    CString::new(super::supported_scales_json())
        .unwrap()
        .into_raw()
}
