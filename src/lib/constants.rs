use std::env;

pub fn build_date() -> String {
    let mut build_date = String::new();
    if let Some(b) = env::var("BUILD_DATE").ok() {
        build_date = b
    };
    build_date
}

pub fn service_name() -> String {
    let mut service_name = String::new();
    if let Some(s) = env::var("SERVICE_NAME").ok() {
        service_name = s
    };
    service_name
}

pub fn full_version() -> String {
    let mut version = String::new();
    if let Some(v) = env::var("VERSION").ok() {
        if let Some(g) = env::var("GIT_COMMIT").ok() {
            version = format!("{}-{}", v, g)
        };
    };
    version
}