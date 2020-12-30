use crate::constants::*;
use std::net::UdpSocket;
use std::net::Ipv4Addr;

pub struct Broadcast{
    ss: UdpSocket,
}

pub struct Group{
    ss: UdpSocket,
}


impl Broadcast{

    ///
    ///
    ///
    pub fn create()->Res<Self>{
        // 创建广播服务器
        let mut ss = UdpSocket::bind(DEFAULT_BROADCAST_ADDRESS)?;
        ss.set_read_timeout(Some(std::time::Duration::new(5,0)))?;
        ss.set_broadcast(true)?;
        Ok(Self{ ss })
    }

}

impl Group{

    pub fn create()->Res<Self>{
        // 创建组播服务器
        let mut ss = UdpSocket::bind(DEFAULT_BROADCAST_ADDRESS)?;
        ss.set_read_timeout(Some(std::time::Duration::new(5,0)))?;
        ss.set_broadcast(true)?;
        Ok(Self{ ss })
    }
}