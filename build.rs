use std::env;
use std::fs;
use std::path::Path;

fn main() {
    const LOG_CFG_FILE: &'static str = "log4rs.yml";
    println!("cargo:rerun-if-changed={}", LOG_CFG_FILE);
    let tgt_dir = Path::new(env::var("CARGO_TARGET_DIR").unwrap().as_ref());
    fs::copy(LOG_CFG_FILE, tgt_dir.join(LOG_CFG_FILE)).expect("copoy log config file");
}