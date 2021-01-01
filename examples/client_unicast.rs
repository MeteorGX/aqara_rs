use aqara_rs::prelude::Res;
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};

fn main() ->Res<()> {

    // 设置需要访问的服务器地址 => 127.0.0.1:8081, 这里也可以直接编写局域网地址, 反正必须是服务器地址
    let unicast_address = SocketAddr::from(
        (Ipv4Addr::LOCALHOST,8081)
    );

    // 客户端连接让系统随机选择即可
    let client_socket = UdpSocket::bind(SocketAddr::from(
        (Ipv4Addr::UNSPECIFIED,0)
    ))?;

    // 连接到服务器
    client_socket.connect(unicast_address)?;

    // 打印本地客户端地址
    println!("Client = {}",client_socket.local_addr()?.ip().to_string());

    // 采用命令行接收数据并发放给服务器
    let mut buffer = String::new();
    println!("Input Send Message:");
    std::io::stdin().read_line(&mut buffer)?;

    // 发送数据
    client_socket.send(buffer.as_bytes())?;

    // 获取服务器返回数据
    let mut buf = [0;1024];
    let sz = client_socket.recv(&mut buf)?;
    println!("Response = {:?}",&buf[..sz]);

    Ok(())
}