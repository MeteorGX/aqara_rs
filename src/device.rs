//!
//! # 设备架构说明
//!
//! 这里分为网关和其他 iOT, 组网传播的架构如下：
//!
//! <pre>
//!  iOT_1,iOT_2,iOT_3                        网关                           我们准备接入的服务器
//!      |                                     |                                   |
//! 假设入网 IP( 192.168.0.43~45... )      假设目前网关 IP(192.168.0.42)        已经接入网关路由(192.168.0.100)
//!      |                                     |                                   |
//!      |                                     |                         本地需要启用端口监听接收广播信息( BIND -> socket = UDP:192.168.0.100:20022 )
//!      |                                     |                                   |
//!      |                                     |       将服务挂起并监听( UDP 报文协议, 不存在 Listen 所以 BIND 之后就轮询报文查询recv/send )
//!      |                                     |                                   |
//!      |                                     |                               加入网关广播, 参数如下:
//!      |                                     |                       广播地址 -> gateway = net::Ipv4Addr::new(224,0,0,50)
//!      |                                     |                       接口地址 -> any_address = net::Ipv4Addr::UNSPECIFIED
//!      |                                     |                       加入地址 -> socket.join_multicast_v4(gateway,any_address)
//!      |                                     |  Rust 设置 (0.0.0.0) 即为 INADDR_ANY, 如果指定 INADDR_ANY 系统会自动分配合适的接口使用, 如果要指定网关发送就需要指定该值
//!      |                                     |                                   |
//!      |                                     |              首先需要获取全部网关, 默认获取网关是没有加密的, 所以可以直接进行命令请求( {cmd:'whois'} JSON 数据 )
//!      |                                     |              首先测试发送数据包 -> socket.send("{cmd:'whois'}".as_bytes()) -> 等待 recv 获取所有消息
//!      |                                     |                                   |
//! 汇报网关自身( 广播通知 `whois` )  ->  汇报网关自身( 广播通知 `whois` )       ->   接收到所有网关的广播数据
//! </pre>
//!
//! 以此架构可以反推如何构建出网关服务
//!

use crate::prelude::{DEFAULT_MULTICAST_ADDRESS, DEFAULT_MULTICAST_PORT, Res, DEFAULT_UNICAST_ADDRESS, DEFAULT_UNICAST_PORT, ResponseEvent};
use crate::session::{Multicast, Unicast};
use std::net::Ipv4Addr;
use std::sync::Arc;

///
/// 网关构建器
/// 初始化网关的配置信息, 一般来说网关内部有组播和单播句柄, 组播用于服务发现和通知, 单播用于点对点通讯
///
/// 参数说明:
/// * multicast_address: 网关的组播地址, 一般默认为 `224.0.0.50`
/// * multicast_port: 网关的组播端口, 一般默认为 `4321`
/// * unicast_address: 本机单播接收网关服务的地址, 一般可以留空, 只有在设备支持多网络环境的时候才需要
/// * unicast_port: 本机单播接收网关服务的端口, 一般默认为 `9898`
///
pub struct Gateway{
    multicast:Multicast,
    unicast:Arc<Unicast>,
    capacity:usize
}

impl Gateway {
    pub fn with_capacity(capacity:usize)->Res<Self>{
        let multicast = Multicast::create(
            DEFAULT_UNICAST_ADDRESS,
            DEFAULT_MULTICAST_PORT,
            DEFAULT_MULTICAST_ADDRESS,
            Ipv4Addr::UNSPECIFIED
        )?;

        let unicast = Unicast::create(
            DEFAULT_UNICAST_ADDRESS,
            DEFAULT_UNICAST_PORT)?;
        Ok(Self{multicast,unicast:Arc::new(unicast),capacity})
    }

    pub fn run(&self, callback:Box<dyn ResponseEvent+Sync+Send>) ->Res<()>{

        let cb = Arc::new(callback);

        let thread_unicast = self.unicast.clone();
        let thread_capacity = self.capacity;
        let thread_cb = cb.clone();

        std::thread::spawn(move ||{
            let mut buffer_unicast = vec![0;thread_capacity];
            while let Ok((sz,client)) = thread_unicast.recv_from(buffer_unicast.as_mut_slice()) {
                if sz > 0 {
                    let client = thread_unicast.load_client(client).unwrap();
                    thread_cb.join_unicast(
                        buffer_unicast[..sz].to_vec(),
                        client
                    );
                }
            };
        });

        // 注册设备发现, 设备发现必须要在主线程, 当设备发现出错的时候直接中断线程, 而不是等待服务线程主导中断
        let mut buffer_multicast = vec![0;self.capacity];
        let main_cb = cb.clone();
        loop{
            match self.multicast.recv_from(buffer_multicast.as_mut_slice()) {
                Ok((sz,client)) => {
                    if sz > 0 {
                        let client = self.multicast.load_client(client).unwrap();
                        main_cb.join_multicast(buffer_multicast[..sz].to_vec(),client);
                    }
                }
                Err(e) => {
                    eprintln!("{:?}",e);
                }
            }
        }
        Ok(())
    }


}
