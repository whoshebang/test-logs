#[macro_use]
extern crate log;
extern crate log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    trace!("trace: booting up");
    debug!("debug: booting up");
    info!("info: booting up");
    warn!("warn: booting up");
    error!("error: booting up");

    // ...
}
