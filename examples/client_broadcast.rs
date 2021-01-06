use aqara_rs::prelude::Res;
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};

fn main() ->Res<()> {
    // 客户端连接让系统随机选择即可
    let client_socket = UdpSocket::bind(SocketAddr::from(
        (Ipv4Addr::UNSPECIFIED,0)
    ))?;

    // 连接到广播服务器, 注意现在不需要 connect, 而是需要启用广播推送功能
    client_socket.set_broadcast(true)?;

    // 打印本地客户端地址
    println!("Client = {}",client_socket.local_addr()?.ip().to_string());

    // 采用命令行接收数据并发放给服务器
    let mut buffer = String::new();
    println!("Input Send Message:");
    std::io::stdin().read_line(&mut buffer)?;

    // 发送数据, 注意这里使用 send_to 指定发送的地址, 这里的地址需要是内网的广播地址
    // 通过内网的广播地址可以推送内网中指定端口数据, 这里指定 255.255.255.255 让其广播到内网设备的 8082 端口
    let broadcast_address = SocketAddr::from(
        (Ipv4Addr::new(255,255,255,255),8082)
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