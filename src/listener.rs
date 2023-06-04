use crate::broadcaster::Broadcaster;
use std::net::UdpSocket;
use std::sync::Arc;

pub async fn listener_handler(listener_address: &str, broadcaster: Arc<Broadcaster>) {
    let socket: UdpSocket = UdpSocket::bind(listener_address).expect("Couldn't bind to address");
    let mut buf: [u8; 65527] = [0; 65527];

    println!("Listening UDP on {}", socket.local_addr().unwrap());

    loop {
        let (amt, _src) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let buffer = &buf[..amt];
        broadcaster.broadcast(buffer).await;
    }
}
