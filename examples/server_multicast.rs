use aqara_rs::prelude::Res;
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};


fn main() -> Res<()> {

    // 监听访问 => 0.0.0.0:8083
    let multicast_address = SocketAddr::from(
        (Ipv4Addr::UNSPECIFIED,8083)
    );
    let multicast_socket = UdpSocket::bind(multicast_address)?;

    // 这里需要加入组网, 只允许该组网内部的发送过来的消息, 使用组网: 224.0.0.50
    multicast_socket.join_multicast_v4(
        &Ipv4Addr::new(224,0,0,50),
        &Ipv4Addr::UNSPECIFIED
    )?;

    // 确定启动地址
    println!("Server = {}",multicast_address.ip().to_string());

    // 设置缓冲区
    let mut buffer = [0;1024];

    // 启动服务器
    while let Ok((sz,client)) = multicast_socket.recv_from(&mut buffer) {
        // 打印加入的会话
        let ip_address = client.ip();
        println!("Join Session = {:?}",ip_address);

        // 客户端 ECHO
        if let Err(e) = multicast_socket.send_to(&buffer[..sz],client){
            eprintln!("Failed By Send = {:?}",e);
        }
    }

    // 如果想设置动态退出组播, 不同于广播强制所有主机推送, 组播是可以动态退出组和加入新的组, 利用以下方式离开组:
    // multicast_socket.leave_multicast_v4(
    //    &Ipv4Addr::new(224,0,0,50),
    //    &Ipv4Addr::UNSPECIFIED
    // )?;

    Ok(())
}
