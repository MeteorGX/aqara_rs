use aqara_rs::prelude::Res;
use aqara_rs::session::{Unicast, Broadcast, Multicast};
use std::net::{Ipv4Addr};


#[test]
fn unicast() ->Res<()>{
    // 测试单播服务推送消息
    let server_address = Ipv4Addr::LOCALHOST; // 127.0.0.1
    let server_port = 8081;

    // 绑定端口并且创建服务器
    let server = Unicast::create(server_address,server_port)?;

    // 创建线程并移交服务器任务
    std::thread::spawn(move ||{
        println!("Server[Unicast] Startup");

        // 初始化缓冲区读取数据
        let mut buffer = [0;1024];

        // 循环接收数据并回显
        while let Ok((received,client)) =  server.recv_from(&mut buffer){
            if let Ok(client) = server.load_client(client) {
                let addr = client.get_client_addr().unwrap();

                println!("Server[Unicast] Received = {} Bytes | Client = {:?}:{:?}",received,addr.ip(),addr.port());
                let _ = client.send(&buffer[..received]);
            }
        }
    });

    // 先主线程暂停下让服务器线程确保已经运行
    std::thread::sleep(std::time::Duration::new(1,0));

    // 测试单播发送数据

    // 初始化单播客户端
    let client = Unicast::connect(server_address.clone(),server_port)?;

    // 发送数据
    let message = "{ \"cmd\": \"unicast\" }";
    client.send(&message.as_bytes())?;

    // 接收数据
    let mut buffer = [0;1024];
    if let Ok((sz,_)) = client.recv_from(&mut buffer) {
        println!("Client[Unicast] Received = {} Bytes",sz);
    }
    Ok(())
}

#[test]
fn broadcast()->Res<()>{
    // 测试广播服务推送消息
    let server_address = Ipv4Addr::UNSPECIFIED; // 0.0.0.0
    let server_port = 8082;

    // 绑定端口并且创建服务器
    let server = Broadcast::create(
        server_address,
        server_port
    )?;

    // 创建线程并移交服务器任务
    std::thread::spawn(move ||{
        println!("Server[Broadcast] Startup");

        // 初始化缓冲区读取数据
        let mut buffer = [0;1024];

        // 循环接收数据并回显
        while let Ok((received,client)) =  server.recv_from(&mut buffer){
            if let Ok(client) = server.load_client(client) {
                let addr = client.get_client_addr().unwrap();

                println!("Server[Broadcast] Received = {} Bytes | Client = {:?}:{:?}",received,addr.ip(),addr.port());
                let _ = client.send(&buffer[..received]);
            }
        }
    });

    // 先主线程暂停下让服务器线程确保已经运行
    std::thread::sleep(std::time::Duration::new(1,0));

    // 测试广播发送数据

    // 初始化广播客户端: 委托 255.255.255.255 向内网的所有主机端口 8082 发送信息
    let broadcast_address = Ipv4Addr::new(255,255,255,255);// 广播地址
    let client = Broadcast::connect(broadcast_address,server_port)?;

    // 发送数据
    let message = "{ \"cmd\": \"broadcast\" }";
    client.send(&message.as_bytes())?;

    // 接收数据
    let mut buffer = [0;1024];
    if let Ok((sz,_)) = client.recv_from(&mut buffer) {
        println!("Client[Broadcast] Received = {} Bytes",sz);
    }

    Ok(())
}

#[test]
fn multicast()->Res<()>{
    // 测试多播服务推送消息
    let server_address = Ipv4Addr::UNSPECIFIED; // 0.0.0.0
    let server_port = 8083;

    // 绑定端口并且创建服务器
    let server = Multicast::create(
        server_address,
        server_port,
        Ipv4Addr::new(224,0,0,50),
        Ipv4Addr::UNSPECIFIED
    )?;

    // 创建线程并移交服务器任务
    std::thread::spawn(move ||{
        println!("Server[Multicast] Startup");

        // 初始化缓冲区读取数据
        let mut buffer = [0;1024];

        // 循环接收数据并回显
        while let Ok((received,client)) =  server.recv_from(&mut buffer){
            if let Ok(client) = server.load_client(client) {
                let addr = client.get_client_addr().unwrap();

                println!("Server[Multicast] Received = {} Bytes | Client = {:?}:{:?}",received,addr.ip(),addr.port());
                let _ = client.send(&buffer[..received]);
            }
        }
    });

    // 先主线程暂停下让服务器线程确保已经运行
    std::thread::sleep(std::time::Duration::new(1,0));

    // 测试组播发送数据

    // 初始化组播客户端: 委托 224.0.0.50 向内网的所有主机端口 8083 发送信息
    let multicast_address = Ipv4Addr::new(224,0,0,50);// 广播地址
    let client = Multicast::connect(multicast_address,server_port)?;

    // 发送数据
    let message = "{ \"cmd\": \"multicast\" }";
    client.send(&message.as_bytes())?;

    // 接收数据
    let mut buffer = [0;1024];
    if let Ok((sz,_)) = client.recv_from(&mut buffer) {
        println!("Client[Multicast] Received = {} Bytes",sz);
    }

    Ok(())
}