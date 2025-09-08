use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn reply_greeting(name: *const c_char) -> *mut c_char {
    // Funktion, die von außen (z.B. Dart) aufgerufen werden kann
    let c_str = unsafe {
        assert!(!name.is_null());
        CStr::from_ptr(name)
    };
    let name_str = c_str.to_str().unwrap_or("Guest");
    let greeting = format!("Hello, {}! How are you?", name_str);
    CString::new(greeting).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut c_char) {
    // Funktion zum Freigeben von Strings, die von Rust zurückgegeben wurden
    if s.is_null() { return; }
    unsafe {
       drop(CString::from_raw(s))
    }
}