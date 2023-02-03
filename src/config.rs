use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Config {
        match envy::prefixed("TIBAH_").from_env::<Config>() {
            Ok(config) => config,
            Err(err) => panic!("{:#?}", err),
        }
    }
}
