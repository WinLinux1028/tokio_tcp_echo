use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:20230")
        .await
        .unwrap();
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
