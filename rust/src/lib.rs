use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
fn hello() -> *mut c_char {
    let cstring = CString::new("hello from rust").unwrap();
    cstring.into_raw()
}

#[no_mangle]
fn freemem(c: *mut c_char) {
    unsafe {
        CString::from_raw(c);
    }
}
