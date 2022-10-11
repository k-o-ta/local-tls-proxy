use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long = "cert")]
    cert: String,

    #[clap(long = "cert-key")]
    cert_key: String,

    #[clap(long = "src-ip", default_value = "localhost")]
    src_ip: String,

    #[clap(long = "dest-ip")]
    dest_ip: String,

    #[clap(long = "src-port", default_value_t = 443)]
    src_port: i32,

    #[clap(long = "dest-port")]
    dest_port: i32,
}

fn main() {
    let args = Args::parse();
}
