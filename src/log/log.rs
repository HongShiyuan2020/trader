use std::env;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_logger() {

    let mode = env::var("MODE").expect("Please check MODE in .env!");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| 
                if mode.eq("dev") {
                    "debug".into()
                } else {
                    "info".into()
                }
            ),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
