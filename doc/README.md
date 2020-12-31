# 架构说明

> Version: 1.0
> Date: 2020-12-30     
> Author: MeteorCat

这里需要解释说明 `aqara` 的整体架构, 官网内部局域网功能提供以下 `API`:
* 设备通知和发现
* 设备状态上报
* 设备读写操作
* 设备心跳包管理

而加密方式基于 `AES-CBC-128` 且无填充的 `16` 字节长度为主的字符串之后解析为 `32` 长度的 `ASCII` 数据, 具体加密流程可以参照 `src/builder.rs` 内部实现.

比较需要说明的是数据交换都是基于 `UDP` 的两种方式:
* UDP 组播
* UDP 单播

如果刚开始接触需要区分好组播, 单播和广播的区分:
* 单播: 内网之中单对单主机的通讯, 基于 UDP 报文协议可以交互两台主机数据.
* 广播: 内网之中单对所有 (注意:`所有`) 可能存在的主机发送报文.
* 组播: 内网之中单对指定组的主机进行报文发送, 而不会直接广播所有内网主机.

内部实现可以参考 `src/session.rs`

### 单播 Rust 实现

```rust
type EBox = Box<dyn std::error::Error + Send + Sync>;
type Res<T> = Result<T,EBox>;
use std::net::{UdpSocket, Ipv4Addr, SocketAddr};

fn main() ->Res<()>{
    // 比较复杂的设置地址 -> 127.0.0.1:8081
    let address_by_server = SocketAddr::from(
        (Ipv4Addr::new(127,0,0,1), 8081)
    );

    // 创建服务器 Socket
    let socket_by_server = UdpSocket::bind(address_by_server.clone())?;

    println!("Server Startup");

    // 将数据丢至线程运行
    std::thread::spawn(move ||{
        // 初始化缓冲区读取数据
        let mut buffer = [0;1024];
        while let Ok(received) =  socket_by_server.recv(&mut buffer){
            println!("Server Received = {} Bytes",received);
            println!("Data = {:?}",&buffer[..received]);
        }
    });

    // 假设以上创建单播的服务器, 现在点对点单播 ============================

    // 本地设置好需要连接的地址, 注意和 TCP 形式不同, UDP 需要手动创建本地连接服务器的地址接口
    // 这里可以只能 (127.0.0.1,8088) 等手动地址, 也可以直接分配 (0.0.0.0,0) 让系统自动分配
    let address_by_client = SocketAddr::from(
        (Ipv4Addr::new(0,0,0,0), 0)
    );

    // 对接本地的数据 socket
    let socket_by_client = UdpSocket::bind(address_by_client)?;

    // 连接到服务器的 地址
    socket_by_client.connect(address_by_server.clone())?;

    // 这里使用命令行输入内容直接发送到线程内部的 UDP 服务器并实现打印
    let mut buffer = String::new();
    println!("Input Message Info:");
    std::io::stdin().read_line(&mut buffer)?;

    // 接收到数据发送
    socket_by_client.send(buffer.as_bytes())?;

    Ok(())
}
```

运行的时候直接就可以转发到线程内部的 `UDP` 服务器之中.

### 广播 Rust 实现

首先需要注意这里一般依赖内网当中某个地址来推送通知, 默认为 `255.255.255.255`

> 注: 局域网下 `255.255.255.255` 可以广播, 但是不会被路由器转发

```rust
type EBox = Box<dyn std::error::Error + Send + Sync>;
type Res<T> = Result<T,EBox>;
use std::net::{UdpSocket, Ipv4Addr, SocketAddr};

fn main() ->Res<()>{
    // 这里同样设置服务器监听, 等待广播地址数据发来 -> 0.0.0.0:8081
    let address_by_server = SocketAddr::from(
        (Ipv4Addr::new(0,0,0,0), 8081)
    );

    // 创建服务器 Socket
    let socket_by_server = UdpSocket::bind(address_by_server.clone())?;

    println!("Server Startup");

    // 将数据丢至线程运行
    std::thread::spawn(move ||{
        // 初始化缓冲区读取数据
        let mut buffer = [0;1024];
        while let Ok(received) =  socket_by_server.recv(&mut buffer){
            println!("Server Received = {} Bytes",received);
            println!("Data = {:?}",&buffer[..received]);
        }
    });

    // 假设以上创建单播的服务器, 现在点对点单播 ============================

    // 先生成随机的本地 UDP 接口用于建立广播数据
    let address_by_client = SocketAddr::from(
        (Ipv4Addr::new(0,0,0,0), 0)
    );

    // 对接本地的数据 socket
    let socket_by_client = UdpSocket::bind(address_by_client)?;

    // 关键点, 设置 socket 发送的属性为广播还有获取结果的超时时间
    socket_by_client.set_read_timeout(Some(std::time::Duration::new(5,0)))?;
    socket_by_client.set_broadcast(true)?;

    // 这里不需要 connect

    // 这里使用命令行输入内容直接发送到线程内部的 UDP 服务器并实现打印
    let mut buffer = String::new();
    println!("Input Message Info:");
    std::io::stdin().read_line(&mut buffer)?;

    // 接收到数据发送, 注意这里应该用 send_to 提供针对性的广播地址 -> 255.255.255.255 -> ALL:8081
    let address_by_broadcast = SocketAddr::from(
        (Ipv4Addr::new(255,255,255,255), 8081)
    );
    // OK, 发送到广播数据, 线程内部的服务器应该接收到广播数据
    socket_by_client.send_to(buffer.as_bytes(),address_by_broadcast)?;

    Ok(())
}
```

### 组播 Rust 实现

建议如果后续需要一对 N 的通信开发应采用该推送模式, 这里组播在内网地址分为两种:
* 永久组播地址: `224.0.0.0~224.0.0.255`, 注意很多服务器都使用该组, 不要设定该组乱推送消息.
* 临时公用地址: `224.0.1.0~238.255.255.255`.
* 本地管理地址: `239.0.0.0~239.255.255.255`.

> 注: 组播也有人称为多播, 绿米网关组播地址为 `224.0.0.50`, 其他 `iot` 网关成员都是监听该组的 `4321` 端口

这里样例会模仿绿米网关来伪装成 `whois` 信息推送到请求:

```rust
type EBox = Box<dyn std::error::Error + Send + Sync>;
type Res<T> = Result<T,EBox>;
use std::net::{UdpSocket, Ipv4Addr, SocketAddr};

// 这里硬编码返回数据作为测试示范
const WHOIS_MSG:&'static str = "{ \"cmd\":\"iam\", \"ip\":\"192.168.0.42\", \"protocal\":\"UDP\", \"port\":\"9898\", \"model\":\"gateway.aq1\" }";
const WHOIS_CMD:&'static str = "{cmd: whois}";

fn main() ->Res<()>{
    // 这里同样设置服务器监听, 等待广播地址数据发来 -> 0.0.0.0:4321
    let address_by_server = SocketAddr::from(
        (Ipv4Addr::new(0,0,0,0), 4321)
    );

    // 创建服务器 Socket
    let socket_by_server = UdpSocket::bind(address_by_server.clone())?;

    // 注意: 这里需要设置加入组网 (244.0.0.50),并且允许所有数据 (0.0.0.0) 通过该组发送数据
    socket_by_server.join_multicast_v4(
        &Ipv4Addr::new(224,0,0,50),
        &Ipv4Addr::new(0,0,0,0)
    )?;

    println!("Server Startup");

    // 将数据丢至线程运行
    std::thread::spawn(move ||{
        // 初始化缓冲区读取数据
        let mut buffer = [0;1024];
        // 这里使用 recv_from/send_to 可以接收/发送指定 socket
        while let Ok((received,src)) =  socket_by_server.recv_from(&mut buffer){
            // 跳过非 whois 命令的数据
            if (&buffer[..received]).ne(WHOIS_CMD.as_bytes()) { continue; }
            println!("Server Received = {} Bytes",received);
            println!("Data = {:?}",&buffer[..received]);

            // 返回 `网关` 信息
            if socket_by_server.send_to(WHOIS_MSG.as_bytes(),&src).is_err() {
                eprintln!("Failed By Send!");
            };
        }
    });

    // 这里没有命令行阻塞, 需要将主线程先阻塞保证执行顺序
    std::thread::sleep(std::time::Duration::new(1,0));

    // 假设以上创建单播的服务器, 现在点对点单播 ============================

    // 先生成随机的本地 UDP 接口用于建立广播数据
    let address_by_client = SocketAddr::from(
        (Ipv4Addr::new(0,0,0,0), 0)
    );

    // 对接本地的数据 socket
    let socket_by_client = UdpSocket::bind(address_by_client)?;

    // 关键点, 设置 socket 超时时间和组播
    socket_by_client.set_read_timeout(Some(std::time::Duration::new(5,0)))?;
    socket_by_client.join_multicast_v4(
        &Ipv4Addr::new(224,0,0,50),
        &Ipv4Addr::new(0,0,0,0)
    )?; // 加入组播设置类型

    // 这里不需要 connect

    // 这里不需要用命令行输入内容, 直接模拟 whois 指令

    // 接收到数据发送, 注意这里应该用 send_to 提供针对性的组播地址 -> 224.0.0.50 -> ALL:4321
    let address_by_multicast = SocketAddr::from(
        (Ipv4Addr::new(224,0,0,50), 4321)
    );
    // OK, 发送到广播数据
    socket_by_client.send_to(WHOIS_CMD.as_bytes(),address_by_multicast)?;

    // 接收获取到返回内容
    let mut buffer = [0;1024];
    let sz = socket_by_client.recv(&mut buffer)?;
    println!("Client Response = {}",std::str::from_utf8(&buffer[..sz])?);

    // 退出该组播
    socket_by_client.leave_multicast_v4(
        &Ipv4Addr::new(224,0,0,50),
        &Ipv4Addr::new(0,0,0,0)
    )?;
    Ok(())
}
```

OK, 这就是简单实现的手法, 相对简单的 `iot` 网关实现, 后续的绿米网关内部采用 `AES-CBC-128` 加密密钥 `Key` 来作为操作标识; 
单纯的裸组播是不可取的, 需要对数据进行加/解密处理进行处理.

 
