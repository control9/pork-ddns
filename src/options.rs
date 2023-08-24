use clap::Parser;
#[derive(Parser)]
pub struct Options {
    /// DDNS refresh periodicity in seconds
    #[arg(short, long, default_value_t = 60)]
    pub periodicity: u64,
    /// TTL of DNS record (defaults to 600 seconds)
    #[arg(long)]
    pub ttl: Option<u64>,
    /// Create record if missing on service start
    #[arg(long)]
    pub create: bool,
    /// Porkbun API key
    #[arg(long)]
    pub api_key: String,
    /// Porkbun secret API key
    #[arg(long)]
    pub secret_api_key: String,
    /// subdomain to change (leave blank to change root record instead)
    pub subdomain: Option<String>,
    /// domain to change (without subdomain part)
    pub domain: String,
}