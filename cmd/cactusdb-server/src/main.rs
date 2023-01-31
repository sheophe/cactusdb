use cactusdb::config::{errors, CactusDbConfig, ConfigFormat, LogFormat, LogLevel};
use clap::Parser;
use std::process;

#[derive(Parser, Debug)]
#[command(
    about = "Fast highly available distributed transactional key-value database and message exchange system for low-bandwidth radio networks"
)]

struct Args {
    #[arg(short, long, value_name = "FILE", help = "Set the configuration file")]
    config: String,

    #[arg(
        required = false,
        long = "config-format",
        value_enum,
        value_name = "FORMAT",
        default_value = "toml",
        help = "Set the format of the configuration file"
    )]
    config_format: ConfigFormat,

    #[arg(
        required = false,
        long = "config-check",
        help = "Check config file validity and exit"
    )]
    config_check: bool,

    #[arg(
        required = false,
        long = "config-info",
        value_enum,
        value_name = "FORMAT",
        help = "Print configuration information with specified format"
    )]
    config_info: Option<ConfigFormat>,

    #[arg(
        required = false,
        short = 'l',
        long = "log-level",
        value_enum,
        value_name = "LEVEL",
        help = "Set the log level"
    )]
    log_level: Option<LogLevel>,

    #[arg(
        required = false,
        long = "log-format",
        value_enum,
        value_name = "FORMAT",
        help = "Set the log format"
    )]
    log_format: Option<LogFormat>,

    #[arg(
        required = false,
        short = 'f',
        long = "log-file",
        value_name = "FILE",
        help = "Set the log file",
        long_help = "Set the log file path. If not set, logs will output to stderr"
    )]
    log_file: Option<String>,

    #[arg(
        required = false,
        short,
        long,
        value_name = "IP:PORT",
        help = "Set the WebSocket API address"
    )]
    addr: Option<String>,

    #[arg(
        required = false,
        long = "cluster-addr",
        value_name = "IP:PORT",
        help = "Set the cluster API address",
        long_help = "Set the syncronization address for the cluster. If not set, the node will start in standalone mode"
    )]
    cluster_addr: Option<String>,

    #[arg(
        required = false,
        short = 's',
        long = "data-dir",
        value_name = "PATH",
        help = "Set the directory used to store data"
    )]
    data_dir: Option<String>,

    #[arg(
        required = false,
        long,
        value_name = "CAPACITY",
        help = "Set the store capacity",
        long_help = "Set the store capacity to use. If not set, use entire partition"
    )]
    capacity: Option<String>,
}

fn check_config(args: Args) -> Result<String, errors::ConfigError> {
    CactusDbConfig::new(args.config.as_str(), args.config_format)?;
    Ok("config is ok".to_string())
}

fn read_config(args: Args) -> Result<CactusDbConfig, errors::ConfigError> {
    let mut config = CactusDbConfig::new(args.config.as_str(), args.config_format)?;
    if args.log_level.is_some() {
        config.log.level = args.log_level.unwrap();
    }
    if args.log_format.is_some() {
        config.log.format = args.log_format.unwrap();
    }
    if args.log_file.is_some() {
        config.log.file.filename = args.log_file.unwrap();
    }
    if args.addr.is_some() {
        config.server.addr = args.addr.unwrap().parse()?;
    }
    if args.cluster_addr.is_some() {
        config.server.cluster_addr = Some(args.cluster_addr.unwrap().parse()?);
    }
    Ok(config)
}

fn print_config(args: Args) -> Result<String, errors::ConfigError> {
    let format = args.config_info.unwrap();
    let config = read_config(args)?;
    config.to_formatted_string(format)
}

fn main_with_result() -> Result<(), errors::ConfigError> {
    let args = Args::parse();
    if args.config_check {
        println!("{}", check_config(args)?);
        return Ok(());
    }
    if args.config_info.is_some() {
        println!("{}", print_config(args)?);
        return Ok(());
    }
    let _config = read_config(args)?;
    println!("ok, bye");
    Ok(())
}

fn main() {
    match main_with_result() {
        Ok(_) => process::exit(0),
        Err(error) => {
            println!("{}", error.to_string());
            process::exit(1)
        }
    }
}
