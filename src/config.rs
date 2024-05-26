use std::env;
use std::path::PathBuf;

pub struct Config {
    pub cache_dir: PathBuf,
    pub psql_service: String,
}

impl Default for Config {
    fn default() -> Config {
        let mut key = "MTC_CACHE_DIR";
        let cache_dir = env::var(key).expect(key);

        key = "MTC_PSQL_SERVICE";
        let psql_service = env::var(key).expect(key);

        Config {
            cache_dir: PathBuf::from(cache_dir),
            psql_service: psql_service,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }
}
