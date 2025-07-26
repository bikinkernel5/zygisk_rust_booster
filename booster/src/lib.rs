use android_logger::Config;
use log::Level;

#[no_mangle]
pub extern "C" fn zygisk_main() {
    // Inisialisasi logger Android sekali saja
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Info)
            .with_tag("ZygiskRust"),
    );

    // Log info ke logcat
    log::info!("Zygisk Rust Booster aktif!");
}
