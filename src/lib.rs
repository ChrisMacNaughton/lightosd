use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "lightosd", about = "A lightweight, faster OSD.")]
pub struct Config {
    #[structopt(short = "d", long = "dev", parse(from_os_str))]
    device: PathBuf,
    #[structopt(
        short = "r",
        long = "root-path",
        default_value = "/var/lib/ceph",
        parse(from_os_str)
    )]
    root: PathBuf,
    #[structopt(
        short = "c",
        long = "config-path",
        default_value = "/etc/ceph",
        parse(from_os_str)
    )]
    config_path: PathBuf,
    #[structopt(short = "k", long = "client", default_value = "admin")]
    client: String,
}

impl Config {
    pub fn load() -> Config {
        Config::from_args()
    }
}

pub fn hello(config: &Config) {
    println!("Hello world!");
    println!(
        "Running {} with configuration from {} in {}",
        config.device.display(),
        config.config_path.display(),
        config.root.display()
    );
}
