pub fn crate_author() -> &'static str {
    option_env!("CARGO_PKG_AUTHORS")
        .and_then(|authors| authors.split(',').next())
        .unwrap_or("<unknown>")
}

pub fn crate_description() -> &'static str {
    option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("")
}

pub fn crate_name() -> &'static str {
    option_env!("CARGO_PKG_NAME").unwrap_or("<program name not set>")
}

pub fn crate_version() -> &'static str {
    option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
}
