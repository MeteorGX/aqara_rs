use aqara_rs::prelude::Res;
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};

fn main() ->Res<()> {
    // 客户端连接让系统随机选择即可
    let client_socket = UdpSocket::bind(SocketAddr::from(
        (Ipv4Addr::UNSPECIFIED,0)
    ))?;

    // 连接到广播服务器, 注意现在不需要 connect, 也不需要 broadcast, 而是加入组网: 224.0.0.50
    client_socket.join_multicast_v4(
        &Ipv4Addr::new(224,0,0,50),
        &Ipv4Addr::UNSPECIFIED
    )?;


    // 打印本地客户端地址
    println!("Client = {}",client_socket.local_addr()?.ip().to_string());

    // 采用命令行接收数据并发放给服务器
    let mut buffer = String::new();
    println!("Input Send Message:");
    std::io::stdin().read_line(&mut buffer)?;

    // 发送数据, 注意这里使用 send_to 指定发送的地址, 这里的地址需要是内网的组播地址
    // 通过内网的组播地址可以推送内网中指定端口数据, 这里指定 224.0.0.50 让其组播到内网设备的 8083 端口
    let broadcast_address = SocketAddr::from(
        (Ipv4Addr::new(224,0,0,50),8083)
    );
    client_socket.send_to(
        buffer.as_bytes(),
        broadcast_address
    )?;

    // 获取服务器返回数据
    let mut buf = [0;1024];
    let sz = client_socket.recv(&mut buf)?;
    println!("Response = {:?}",&buf[..sz]);

    Ok(())
}