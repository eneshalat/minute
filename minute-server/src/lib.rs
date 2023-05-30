const MINUTE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn version() -> String {
    format!("Minute-Server version {MINUTE_VERSION}")
}