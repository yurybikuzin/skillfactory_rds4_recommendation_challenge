
use std::sync::Once;
static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| pretty_env_logger::init_timed());
}

