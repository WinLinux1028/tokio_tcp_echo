#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    args.next();

    let mut listen = None;
    while let Some(s) = args.next() {
        match s.as_str() {
            "--listen" => listen = args.next(),
            _ => continue,
        }
    }
    let listen = listen.unwrap_or("[::]:4545".to_string());

    let listener = tokio::net::TcpListener::bind(listen).await.unwrap();
    loop {
        let (stream, addr) = match listener.accept().await {
            Ok(o) => o,
            Err(_) => continue,
        };

        tokio::spawn(async move {
            println!("{}: connected", addr);
            let _ = exec(stream, addr).await;
            println!("{}: disconnected", addr);
        });
    }
}

async fn exec(mut stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);

    loop {
        let buf = reader.fill_buf().await?;
        let buf_len = buf.len();

        if buf_len == 0 {
            break;
        }

        println!("{}: {} bytes received", addr, buf_len);
        writer.write_all(buf).await?;
        writer.flush().await?;
        println!("{}: {} bytes sent", addr, buf_len);

        reader.consume(buf_len);
    }

    Ok(())
}
