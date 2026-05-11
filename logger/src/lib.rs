/// Initialize logging.
///
/// This is best-effort for library consumers: if a logger is already installed,
/// initialization is skipped.
pub fn init() {
    let _ = env_logger::init();
}
