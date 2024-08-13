use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};
use tracing::{info, warn};

pub async fn run(server_addr: impl ToSocketAddrs) {
    let listener = TcpListener::bind(server_addr)
        .await
        .expect("failed to bind");
    info!("Listening on {:?}", listener.local_addr().unwrap());

    loop {
        let (sock, addr) = listener.accept().await.expect("accept failed");
        tokio::spawn(serve_client(sock, addr));
    }
}

async fn serve_client(mut client_socket: TcpStream, client_addr: SocketAddr) {
    info!("Accepted connection from {:?}", client_addr);

    loop {
        let request = read_http_request(&mut client_socket)
            .await
            .expect("read_http_request failed");
        if request.is_empty() {
            warn!("Connection closed by client");
            break;
        }
        info!(
            "Read request from client: {:?}",
            String::from_utf8_lossy(&request)
        );

        let response_first_part = b"HTTP/1.1 200 OK\x0d\x0a\
            Content-Type: text/plain\x0d\x0a\
            Transfer-Encoding: chunked\x0d\x0a\
            Connection: keep-alive\x0d\x0a\
            Content-Encoding: gzip\x0d\x0a\
            \x0d\x0a\
            55\x0d\x0a\
            \x1f\x8b\x08\x00\x00\x00\x00\x00\x00\x03\xabV*\xae\xccM\xca\xcfQ\xb2Rr\x0aq\x0e\x0dv\x09Q\xd2Q\xca/H\xcd\xf3\xcc+I-J-.\x01J\x98\x1b\x18\x98\x9a\xe9\x99\x9a\x18\x03\xa5J2sS\x95\xac\x0c\xcd\x8d\x8cM\x8cLML\x0c---j\x01\xd7Gb;D\x00\x00\x00";
        let response_second_part = b"\x0d\x0a0\x0d\x0a\x0d\x0a";

        client_socket
            .write_all(response_first_part)
            .await
            .expect("response_first_part write_all failed");
        client_socket.flush().await.expect("response_first_part flush failed");
        info!("First part of response has been sent");

        tokio::time::sleep(tokio::time::Duration::from_millis(4000)).await;

        client_socket
            .write_all(response_second_part)
            .await
            .expect("response_second_part write_all failed");
        client_socket.flush().await.expect("response_second_part flush failed");
        info!("Second part of response has been sent");
    }
}

async fn read_http_request(
    client_socket: &mut TcpStream,
) -> core::result::Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();

    // Read until the delimiter "\r\n\r\n" is found
    loop {
        let mut temp_buffer = [0; 1024];
        let n = client_socket.read(&mut temp_buffer).await?;

        if n == 0 {
            break;
        }

        buf.extend_from_slice(&temp_buffer[..n]);

        if let Some(pos) = buf.windows(4).position(|window| window == b"\r\n\r\n") {
            return Ok(buf.drain(..pos + 4).collect());
        }
    }

    Ok(buf)
}
