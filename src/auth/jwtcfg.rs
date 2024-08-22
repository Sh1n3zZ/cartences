use config::{Config, File};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let settings = Config::builder()
        .add_source(File::with_name("config"))
        .build()
        .unwrap();

    settings.get::<HashMap<String, String>>("settings").unwrap()
});

pub static SECRET: Lazy<String> = Lazy::new(|| {
    CONFIG.get("SECRET").expect("SECRET must be set in config").to_string()
});
