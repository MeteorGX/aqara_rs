use crate::prelude::*;
use std::net::{UdpSocket, Ipv4Addr, SocketAddr, IpAddr};
use std::borrow::BorrowMut;


///
/// 单播
///
pub struct Unicast{
    ss: UdpSocket,
    target:SocketAddr,
}

///
/// 广播
///
pub struct Broadcast{
    ss: UdpSocket,
    broadcast:SocketAddr,
}

///
/// 组播|多播
///
pub struct Multicast{
    ss: UdpSocket,
    multicast:SocketAddr,
}


impl Unicast{

    ///
    /// 单播连接指定地址
    ///
    pub fn connect(address:Ipv4Addr,port:u16)->Res<Self>{
        // 本地随机生成端口进行通讯
        let ss = UdpSocket::bind(SocketAddr::from(
            (Ipv4Addr::UNSPECIFIED, 0)
        ))?;

        // 生成接入对象 Socket 地址
        let target = SocketAddr::from((address,port));
        ss.connect(target)?;
        Ok(Self{ss,target})
    }


    ///
    /// 单播指定地址推送数据报文
    ///
    pub fn send_to(&mut self,buf:&[u8])->Res<usize>{
        Ok(self.ss.send_to(buf,self.target)?)
    }


    ///
    /// 单播获取推送过来的数据报文
    ///
    pub fn recv_from(&mut self,buf:&mut [u8])->Res<(usize,SocketAddr)>{
        Ok(self.ss.recv_from(buf)?)
    }

    ///
    /// 单播指定缓存长度的数据, 这里会让数据一直保存在队列之中等待 recv 去消耗, 而不会去消耗数据
    ///
    pub fn peek_from(&mut self,buf:&mut [u8])->Res<(usize,SocketAddr)>{
        Ok(self.ss.peek_from(buf)?)
    }


    ///
    /// 获取原始的 socket 对象, 主要用于设置属性(借用)
    ///
    pub fn get_socket(&mut self)->&mut UdpSocket{
        self.ss.borrow_mut()
    }
}

impl Broadcast{

    ///
    /// 广播的连接相对来说, 需要传递指定的广播地址即可, 且内部不会进行 connect
    /// 这里的 connect 命名只是作为方法名一致和语义类似的作用
    ///
    pub fn connect(address:Ipv4Addr,port:u16)->Res<Self>{
        // 本地随机生成端口进行通讯
        let ss = UdpSocket::bind(SocketAddr::from(
            (Ipv4Addr::UNSPECIFIED, 0)
        ))?;

        // 生成接入对象 Socket 地址
        let broadcast = SocketAddr::from((address,port));

        ss.set_broadcast(true)?;// 开启广播设置
        Ok(Self{ss,broadcast})
    }

    ///
    /// 推送数据到广播地址传递给内网信号
    ///
    pub fn send_to(&mut self,buf:&[u8])->Res<usize> {
        Ok(self.ss.send_to(buf,self.broadcast)?)
    }

    ///
    /// 获取广播数据返回的数据报文
    ///
    pub fn recv_from(&mut self,buf:&mut [u8])->Res<(usize,SocketAddr)>{
        Ok(self.ss.recv_from(buf)?)
    }


    ///
    /// 获取广播推送过来指定缓存长度的数据, 这里会让数据一直保存在队列之中等待 recv 去消耗, 而不会去消耗数据
    ///
    pub fn peek_from(&mut self,buf:&mut [u8])->Res<(usize,SocketAddr)>{
        Ok(self.ss.peek_from(buf)?)
    }


    ///
    /// 获取原始的 socket 对象, 主要用于设置属性(借用)
    ///
    pub fn get_socket(&mut self)->&mut UdpSocket{
        self.ss.borrow_mut()
    }

}


impl Multicast{

    ///
    /// 组播的连接相对来说, 需要多了 join/leave 组的动作, 所以需要单独处理较多数据
    /// 这里的 connect 命名只是作为方法名一致和语义类似的作用
    ///
    pub fn connect(address:Ipv4Addr,port:u16)->Res<Self>{
        // 本地随机生成端口进行通讯
        let ss = UdpSocket::bind(SocketAddr::from(
            (Ipv4Addr::UNSPECIFIED, 0)
        ))?;

        // 生成接入对象 Socket 地址
        let multicast = SocketAddr::from((address,port));

        // 加入分组
        let _ = ss.join_multicast_v4(
            &address,
            &Ipv4Addr::UNSPECIFIED
        );

        Ok(Self{ss,multicast})
    }

    ///
    /// 推送数据到组播地址传递给内网信号
    ///
    pub fn send_to(&mut self,buf:&[u8])->Res<usize> {
        Ok(self.ss.send_to(buf,self.multicast)?)
    }

    ///
    /// 获取组播数据返回的数据报文
    ///
    pub fn recv_from(&mut self,buf:&mut [u8])->Res<(usize,SocketAddr)>{
        Ok(self.ss.recv_from(buf)?)
    }

    ///
    /// 获取组播推送过来指定缓存长度的数据, 这里会让数据一直保存在队列之中等待 recv 去消耗, 而不会去消耗数据
    ///
    pub fn peek_from(&mut self,buf:&mut [u8])->Res<(usize,SocketAddr)>{
        Ok(self.ss.peek_from(buf)?)
    }


    ///
    /// 获取原始的 socket 对象, 主要用于设置属性(借用)
    ///
    pub fn get_socket(&mut self)->&mut UdpSocket{
        self.ss.borrow_mut()
    }

}

impl Drop for Multicast{
    ///
    /// 析构方法, 推出的时候需要离开分组
    ///
    fn drop(&mut self) {

        if let IpAddr::V4(address) = self.multicast.ip(){
            let _ = self.ss.leave_multicast_v4(
                &address,
                &Ipv4Addr::UNSPECIFIED
            );
        }
    }
}