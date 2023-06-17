use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // We retrieve the port assigned to us by the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    println!("Running at http://127.0.0.1:{}", port);
    run(listener)?.await
}

// up to page 76
