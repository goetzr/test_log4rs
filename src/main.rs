use log::{trace, debug, info, warn, error};

fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
