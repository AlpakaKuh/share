use clap::Parser;

/// Simple cli file share tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File to share
    #[arg(required_unless_present = "upload")]
    pub file: Option<String>,

    /// Port to bind
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Auto copy share link to clipboard
    #[arg(short, long, default_value_t = true)]
    pub copy: bool,

    /// Upload for other users
    #[arg(short, long)]
    pub upload: bool,
}
