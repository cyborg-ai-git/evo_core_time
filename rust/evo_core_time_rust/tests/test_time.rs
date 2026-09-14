use env_logger::Builder;
use std::env;
use std::sync::{Arc, Once};

static INIT_LOGGER: Once = Once::new();
const TEST_COUNT: u64 = 1_000_000;
pub fn do_init_logger() {
    INIT_LOGGER.call_once(|| {
        if env::var("RUST_LOG").is_err() {
            unsafe { env::set_var("RUST_LOG", "debug") };
        }
        Builder::from_default_env().init();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use evo_core_time_rust::UTime;
    use futures::FutureExt;
    use futures::future::BoxFuture;
    use log::{debug, error, info};
    use std::collections::HashMap;
    use std::fmt::format;

    #[tokio::test]
    async fn test_id() -> Result<(), Box<dyn std::error::Error>> {
        do_init_logger();

        let start = std::time::Instant::now();

        for i in 0..TEST_COUNT {
            let _ = UTime::time_ns();
            let _ = UTime::time_ms();
            let _ = UTime::time_s();
        }

        let duration = start.elapsed();
        info!("Time elapsed random: {:?}", duration);

        Ok(())
    }
}
