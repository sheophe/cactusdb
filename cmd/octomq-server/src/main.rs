use clap::Parser;
use octomq::config::LogLevel;

#[derive(Parser, Debug)]
#[command(
    about = "Fast distributed highly available in-memory key/value database with publish-subscribe interface"
)]
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
        value_name = "FORMAT",
        help = "Print configuration information with specified format"
    )]
    config_info: String,

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
        help = "Set the log file",
        long_help = "Set the log file path. If not set, logs will output to stderr"
    )]
    log_file: String,

    #[arg(
        short,
        long,
        value_name = "IP:PORT",
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
    grpc_addr: String,

    #[arg(
        required = false,
        long = "sync-addr",
        value_name = "IP:PORT",
        help = "Set the syncronization API address",
        long_help = "Set the syncronization API address for the cluster. If not set, the node will start in standalone mode"
    )]
    sync_addr: String,

    #[arg(
        short = 's',
        long = "data-dir",
        value_name = "PATH",
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
    capacity: String,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
