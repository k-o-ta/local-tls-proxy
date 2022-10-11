use clap::Parser;
use warp::{http::Response, hyper::body::Bytes, Filter, Rejection, Reply};
use warp_reverse_proxy::reverse_proxy_filter;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long = "cert")]
    cert: String,

    #[clap(long = "cert-key")]
    cert_key: String,

    #[clap(long = "dest-ip", default_value = "localhost")]
    dest_ip: String,

    #[clap(long = "src-port", default_value_t = 3030)]
    src_port: u16,

    #[clap(long = "dest-port", default_value_t = 8080)]
    dest_port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cert = args.cert;
    let cert_key = args.cert_key;
    let src_port = args.src_port;
    let dest_ip = args.dest_ip;
    let dest_port = args.dest_port;

    let app = warp::path!().and(reverse_proxy_filter(
        "".to_string(),
        format!("http://{}:{}/", dest_ip, dest_port),
    ));

    warp::serve(app)
        .tls()
        .cert_path(cert)
        .key_path(cert_key)
        .run(([127, 0, 0, 1], src_port))
        .await;
}
