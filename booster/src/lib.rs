#[no_mangle]
pub extern "C" fn zygisk_main() {
    android_log("Zygisk Rust Booster aktif (prio 6 error)!");
}

fn android_log(msg: &str) {
    use std::ffi::CString;
    let tag = CString::new("ZygiskRust").unwrap();
    let c_msg = CString::new(msg).unwrap();
    unsafe {
        __android_log_write(
            6, // <- ubah dari 4 (INFO) ke 6 (ERROR)
            tag.as_ptr() as *const i8,
            c_msg.as_ptr() as *const i8,
        );
    }
}
