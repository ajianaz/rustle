use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncWriteExt};

fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

async fn proxy_client(client: TcpStream, target_addr: SocketAddr) -> io::Result<()> {
    let backend = TcpStream::connect(target_addr).await?;

    let (mut cr, mut cw) = tokio::io::split(client);
    let (mut br, mut bw) = tokio::io::split(backend);

    let client_to_backend = tokio::spawn(async move {
        io::copy(&mut cr, &mut bw).await?;
        bw.shutdown().await.ok();
        Ok::<(), io::Error>(())
    });

    let backend_to_client = tokio::spawn(async move {
        io::copy(&mut br, &mut cw).await?;
        cw.shutdown().await.ok();
        Ok::<(), io::Error>(())
    });

    let (r1, r2) = tokio::join!(client_to_backend, backend_to_client);
    r1??;
    r2??;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listen_port: u16 = get_env("LISTEN_PORT", "8080")
        .parse()
        .expect("LISTEN_PORT must be a valid port number");

    let target_addr = get_env("TARGET_ADDR", "127.0.0.1:8081");
    let target: SocketAddr = target_addr
        .parse()
        .unwrap_or_else(|_| panic!("TARGET_ADDR must be valid host:port, got: {target_addr}"));

    let bind_addr = format!("0.0.0.0:{listen_port}");
    let listener = TcpListener::bind(&bind_addr).await?;

    println!("rustle: listening on {bind_addr} -> {target}");

    loop {
        let (client, client_addr) = listener.accept().await?;
        let target = target;

        tokio::spawn(async move {
            match proxy_client(client, target).await {
                Ok(()) => {}
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {}
                Err(e) => eprintln!("proxy error {client_addr}: {e}"),
            }
        });
    }
}
