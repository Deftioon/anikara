pub mod backend;

use config::{Config, File};
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
pub struct KaraConfig {
    pub moe_server: String,
    pub us_server: String,
    pub song_dir: String,
}

fn load_config() -> Result<KaraConfig, config::ConfigError> {
    let settings = Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?;

    settings.try_deserialize()
}

#[tokio::main]
async fn main() {
    let config = load_config().unwrap();
    println!("{:?}", config);
    backend::networking::moe::main(config).await;
}
