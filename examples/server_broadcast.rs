use aqara_rs::prelude::Res;
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};


fn main() -> Res<()> {

    // 监听访问 => 0.0.0.0:8082
    let broadcast_address = SocketAddr::from(
        (Ipv4Addr::UNSPECIFIED,8082)
    );
    let broadcast_socket = UdpSocket::bind(broadcast_address)?;

    // 确定启动地址
    println!("Server = {}",broadcast_address.ip().to_string());

    // 设置缓冲区
    let mut buffer = [0;1024];

    // 启动服务器
    while let Ok((sz,client)) = broadcast_socket.recv_from(&mut buffer) {
        // 打印加入的会话
        let ip_address = client.ip();
        println!("Join Session = {:?}",ip_address);

        // 客户端 ECHO
        if let Err(e) = broadcast_socket.send_to(&buffer[..sz],client){
            eprintln!("Failed By Send = {:?}",e);
        }
    }
    Ok(())
}
