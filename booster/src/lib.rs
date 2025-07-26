use android_logger::Config;
use log::Level;

#[no_mangle]
pub extern "C" fn zygisk_main() {
    android_logger::init_once(
        Config::default()
            .with_min_level(Level::Info)
            .with_tag("ZygiskRust"),
    );

    log::info!("Zygisk Rust Booster aktif!");
}
