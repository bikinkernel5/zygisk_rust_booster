#[no_mangle]
pub extern "C" fn zygisk_main() {
    android_log("Zygisk Rust Booster aktif!");
}

fn android_log(msg: &str) {
    use std::ffi::CString;
    unsafe {
        let c_msg = CString::new(msg).unwrap();
        __android_log_write(4, b"ZygiskRust\0".as_ptr() as *const i8, c_msg.as_ptr());
    }
}

#[link(name = "log")]
extern "C" {
    fn __android_log_write(prio: i32, tag: *const i8, text: *const i8) -> i32;
}
