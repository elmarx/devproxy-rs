use std::default::Default;
use std::env;
use std::env::current_dir;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug)]
pub struct DevproxyConfig {
    pub addr: SocketAddr,
}

impl Default for DevproxyConfig {
    fn default() -> Self {
        DevproxyConfig {
            addr: "127.0.0.1:8080".parse().unwrap(),
        }
    }
}

impl DevproxyConfig {
    pub fn new(path: Option<PathBuf>) -> Self {
        let path = match path {
            Some(p) => Some(p),
            None => get_default_config_path(),
        };

        if let Some(config_file) = path {
            dbg!(config_file);
            unimplemented!("configuration is WIP");
        }

        Self::default()
    }
}

pub fn get_default_config_path() -> Option<PathBuf> {
    // check if there's a config in $CWD/devproxy.toml
    let cwd_config = current_dir()
        .map(|p| p.join("devproxy.toml"))
        .ok()
        .filter(|p| p.exists());

    // take this file by default
    if cwd_config.is_some() {
        return cwd_config;
    }

    // otherwise, take ~/.devproxy.toml
    env::var("HOME")
        .map(|home| PathBuf::from(home).join(".devproxy.toml"))
        .ok()
        .filter(|p| p.exists())
}
