# 多线程? 多进程?

默认架构下面都是需要组播地址和服务地址, 组播地址用于设备发现, 服务地址用于功能处理;

最开始因为是 `Unix/Linux` 平台上编写, 所以使用多进程模式 `fork` 出父子进程, 父进程负责组播地址的监听, 而子进程则是承担服务接收和处理.
但是在移动到 `Window` 平台开发的时候发现目前的多进程并没办法支持 `Window` 平台, 所以为了平台通用性采取了系统的多线程模式分出两个线程.

这里采用伪造代码进行讲解:

```rust
///
/// 网关对象类
///
/// 用于初始化网关服务, 这里面如果在 C/C++ 相对来说还是比较简单;
/// 但是 Rust 之中的涉及线程和不可移动特性导致必须需要审慎才能通过编译, 而编译完成就代表不会出错
/// 
/// 
struct Gateway{
    // 单播监听服务, 之所以启用 Arc, 是为了让 Unicast 具有 Sync+Send 特性, 让其可以发送给线程处理
    unicast:Arc<Unicast>,

    // 组播监听服务
    multicast:Multicast
}

impl Gateway{
    // 初始化句柄
    pub fn new()->Self{
        // 这里不做详细说明
        // 单播监听地址 -> 0.0.0.0:9898
        // 组播监听地址 -> 224.0.0.50:4321
        // ...
    }

    pub fn run(&self){
        // 创建后台线程接收服务挂起
        let thread_unicast = self.unicast.clone();
        std::thread::spawn(move ||{
            let mut buffer_unicast = [0;1024];
            while let Ok((sz,client)) = thread_unicast.recv_from(&mut buffer_unicast) {
                if sz > 0 {
                    thread_unicast.send_to(&buffer_unicast[..sz],client);
                }
            };
        });
    
        // 注册设备发现, 设备发现必须要在主线程, 当设备发现出错的时候直接中断线程, 而不是等待服务线程主导中断
        let mut buffer_multicast = [0;1024];
        while let Ok((sz,client)) = self.multicast.recv_from(&mut buffer_multicast) {
            if sz > 0 {
                self.multicast.send_to(&buffer_multicast[..sz],client);
            }
        };
    }
}

// 进程入口
fn main(){
    let server = Gateway::new();
    server.run();
}
```

上面的伪代码基本上可以挂起基础的网关 `ECHO` 服务, 但是这里还有其他需要考虑的方法.

# 阻塞? 非阻塞? 

阻塞非阻塞本质上为是否要启用 `异步IO` 特性, 这里面涉及平台差异问题:
* `Unix` 的 `kqueue`
* `Linux` 的 `epoll`
* `Window` 的 `iocp`
* `Rust` 提供的 `async-std` 实现异步
* `Rust` 的 `Tokio` 运行时异步
* 还有原生的 `Unix/Linux - AIO` 实现

每个平台的实现不同, 这里比较靠谱的是 `async-std` 实现, 但是代码库很混杂, 有 `std::future/future-rs` 等.

目前服务器还是采用阻塞模式实现, 没有启用该特性, 后期会追加异步功能.

> 注意: 如果没有启用异步支持, 千万不要将 Socket 设置为 nonblock, 否则运行的时候会死循环不断占用 CPU 来运行

# UDP 缓存区延伸问题

`UDP` 服务器实现的过程之中有个问题, 缓冲区如何设置才合理? 如果发送超过缓冲区的数据会发生什么问题?

这里说明下 `Rust` 的语言当中并没有 `UDP` 的 `close` 机制, 也就是说该错误提示会不断轮询直到客户端主动断开.

这里编写测试样例:

```rust
use std::net::{Ipv4Addr, UdpSocket, SocketAddr};


fn main() -> Result<(),Box<dyn std::error::Error>> {

    // 监听访问 => 0.0.0.0:8081
    let unicast_address = SocketAddr::from(
        (Ipv4Addr::UNSPECIFIED,8081)
    );
    let unicast_socket = UdpSocket::bind(unicast_address)?;

    // 确定启动地址
    println!("Server = {}",unicast_address.ip().to_string());

    // 设置缓冲区, 注意该缓冲区只有 3 个字节
    let mut buffer = [0;3];

    // 启动服务器
    loop{
        match unicast_socket.recv_from(&mut buffer) {
            Ok((sz,client)) => {
                // 打印加入的会话
                let ip_address = client.ip();
                println!("Join Session = {:?}",ip_address);

                // 客户端 ECHO
                if let Err(e) = unicast_socket.send_to(&buffer[..sz],client){
                    eprintln!("Failed By Send = {:?}",e);
                }
            }
            
            // 留意错误打印
            Err(e) =>{
                eprintln!("{:?}",e);
                // Os { code: 10040, kind: Other, message: "一个在数据报套接字上发送的消息大于内部消息缓冲区或其他一些网络限制，或该用户用于接收数据报的缓冲区比数据报小。" }
            }
        }
    }
    Ok(())
}
```

如果设置了读写超时的时候:

```plain
// 设置 socket 的读写超时属性
unicast_socket.set_read_timeout(Some(std::time::Duration::new(3,0)))?;
unicast_socket.set_write_timeout(Some(std::time::Duration::new(3,0)))?;
```

则会不断轮询报错数据超时, 这里面因为没有及时进行读写任务导致内部超时错误被触发, 从而不断报错.

而这个如果在 `C/C++` 读写的时候报错应该直接将其 `socket` 进行 `close`, 或者直接 `shutdown(fd,SHUT_RDWR)`; 
目前 `Rust` 并不支持这种功能, 所以只能尽可能不去更改其配置值.


目前的情况来说, 这里 `buffer` 缓冲区后续先更改成调用者可以配置.


