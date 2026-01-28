use clap::ArgAction;
use clap::Parser;
/// Simple cli file share tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to share
    #[arg(required_unless_present = "upload")]
    pub file: Option<String>,

    /// Port to bind, 0 to use next free
    #[arg(short, long, default_value_t = 0)]
    pub port: u16,

    /// Auto copy share link to clipboard [default: true]
    #[arg(short, long, action=ArgAction::SetFalse)]
    pub copy: bool,

    /// Display's uuid string instead of filename
    #[arg(short, long, default_value_t = false)]
    pub randomized: bool,

    /// Upload for other users
    #[arg(short, long, default_value_t = false)]
    pub upload: bool,

    /// Use https instead of http
    #[arg(short, long, default_value_t = false)]
    pub tls: bool,
}
