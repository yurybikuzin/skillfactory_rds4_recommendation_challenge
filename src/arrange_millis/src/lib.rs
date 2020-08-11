
#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};
#[allow(unused_imports)]
use anyhow::{Result, Error, bail, anyhow};

// ============================================================================
// ============================================================================

/// Оформляет различное количество миллисекунд в виде строки
pub fn get(millis: u128) -> String {
    let secs = millis / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    let mins = mins % 60;
    let secs = secs % 60;
    let millis = millis % 1000;
    if hours != 0 {
        format!("{}:{:0>#2}:{:0>#2}.{:0>#3}", hours, mins, secs, millis)
    } else if mins != 0 {
        format!("{}:{:0>#2}.{:0>#3}", mins, secs, millis)
    } else if secs != 0 {
        format!("{}.{:0>#3}s", secs, millis)
    } else {
        format!("{} ms", millis)
    }
}

// ============================================================================
// ============================================================================
// ============================================================================

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use log::{error, warn, info, debug, trace};
    use super::*;
    use std::sync::Once;
    static INIT: Once = Once::new();
    fn init() {
        INIT.call_once(|| pretty_env_logger::init());
    }

    #[tokio::test]
    async fn it_works() -> Result<()> {
        init();

        assert_eq!(get(0), "0 ms");
        assert_eq!(get(999), "999 ms");
        assert_eq!(get(1000), "1.000s");
        assert_eq!(get(59023), "59.023s");
        assert_eq!(get(60000), "1:00.000");
        assert_eq!(get(60000 * 59), "59:00.000");
        assert_eq!(get(60000 * 60), "1:00:00.000");
        assert_eq!(get(60000 * 60 * 25), "25:00:00.000");

        Ok(())
    }
}

