use clap::{App, Arg};
use std::path::Path;

fn file_exists(v: String) -> Result<(), String> {
    if Path::new(v.as_str()).exists() {
        Ok(())
    } else {
        Err(format!("config-file '{}' not found", v))
    }
}

pub fn get_clap_app<'a, 'b>() -> App<'a, 'b> {
    App::new("devproxy")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("CONFIG")
                .help("path to the config-file (with mappings)")
                .validator(file_exists),
        )
}
