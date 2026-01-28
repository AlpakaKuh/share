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

    /// Auto copy share link to clipboard
    #[arg(short, long, default_value_t = true)]
    pub copy: bool,

    /// Upload for other users
    #[arg(short, long)]
    pub upload: bool,
}
