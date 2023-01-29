use clap::Parser;
use octomq::config::{ConfigFormat, LogLevel, OctomqConfig};

#[derive(Parser, Debug)]
#[command(about = "Fast distributed highly available transactional key-value database")]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Set the configuration file")]
    config: String,

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
        default_value = "json",
        help = "Print configuration information with specified format"
    )]
    config_info: ConfigFormat,

    #[arg(
        required = false,
        short = 'l',
        long = "log-level",
        value_enum,
        value_name = "LEVEL",
        default_value = "info",
        help = "Set the log level"
    )]
    log_level: LogLevel,

    #[arg(
        required = false,
        short = 'f',
        long = "log-file",
        value_name = "FILE",
        default_value = "/dev/stderr",
        help = "Set the log file",
        long_help = "Set the log file path. If not set, logs will output to stderr"
    )]
    log_file: String,

    #[arg(
        required = false,
        short,
        long,
        value_name = "IP:PORT",
        default_value = "127.0.0.1:1996",
        help = "Set the WebSocket API address"
    )]
    addr: String,

    #[arg(
        required = false,
        long = "grpc-addr",
        value_name = "IP:PORT",
        help = "Set the gRPC API address",
        long_help = "Set the gRPC API address. If not set, the node will not provide this interface"
    )]
    grpc_addr: Option<String>,

    #[arg(
        required = false,
        long = "sync-addr",
        value_name = "IP:PORT",
        help = "Set the syncronization API address",
        long_help = "Set the syncronization API address for the cluster. If not set, the node will start in standalone mode"
    )]
    sync_addr: Option<String>,

    #[arg(
        required = false,
        short = 's',
        long = "data-dir",
        value_name = "PATH",
        default_value = "/var/lib/octomq/data",
        help = "Set the directory used to store data"
    )]
    data_dir: String,

    #[arg(
        required = false,
        long,
        value_name = "CAPACITY",
        help = "Set the store capacity",
        long_help = "Set the store capacity to use. If not set, use entire partition"
    )]
    capacity: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
    let config = OctomqConfig::new(args.config.as_str()).expect("config error");
    println!("{:#?}", config);
}
