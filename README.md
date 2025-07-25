# Zygisk Rust Booster

Modul Magisk (Zygisk) untuk menginject library Rust ke proses Android (zygote, system_server, aplikasi) demi mengoptimalkan konsumsi RAM & performa dengan menggantikan sebagian tugas berat Java ke Rust native.

**Fitur rencana**:
- Injeksi Rust .so ke proses sistem
- Hooking fungsi base64, kompresi, crypto, logging
- Logging ke logcat

> Ini adalah project eksperimen. Kontribusi & feedback sangat dihargai!