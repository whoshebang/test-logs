#[macro_use]
extern crate log;
extern crate log4rs;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    trace!("[topicA] booting up");
    debug!("[topicB] booting up");
    info!("[topicC] booting up");
    warn!("[topicD] booting up");
    error!("[topicE] booting up");

    // ...
}
