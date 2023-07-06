fn main() {
    if std::env::var("CROSS_SYSROOT").is_ok() {
        std::env::remove_var("CROSS_SYSROOT");
    }
}
