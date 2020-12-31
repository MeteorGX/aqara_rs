
///
/// 心跳反馈的字节长度: 16
///
pub const AES_KEY_SIZE:usize = 16;

///
/// 这里预定义消息缓存长度, 可以适当调整
///
pub const MESSAGE_BUFF_SIZE:usize = AES_KEY_SIZE;

///
/// 初始化的 AES-CBC-128 Key-IV
///
pub const INITIALIZE_AES_KEY_IV:[u8;AES_KEY_SIZE] = [
    0x17, 0x99, 0x6d, 0x09, 0x3d, 0x28, 0xdd, 0xb3, 0xba, 0x69, 0x5a, 0x2e, 0x6f, 0x58, 0x56, 0x2e
];

///
/// 硬编码 whois 命令
///
pub const COMMAND_WHOIS: &'static str = "{ \"cmd\":\"whois\"}";

///
/// 默认网关广播地址
///
pub const DEFAULT_BROADCAST_ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(224,0,0,50);

///
/// 默认网关广播端口
///
pub const DEFAULT_BROADCAST_PORT:u32 = 4321;

///
/// 默认服务端口
///
pub const DEFAULT_SERVER_PORT:u32 = 9898;

///
/// 设备状态
///
#[derive(Debug)]
pub enum DeviceStatus{
    Open, // 开放
    Close, // 关闭
    Motion, // 被人触发动作
    Click, // 点击
    DoubleClick, // 双击
    BothClick,// 左右键同时按下
    On, // 开启
    Off, // 关闭
}


///
/// 定义简单的错误处理 BOX
///
pub type EBox = Box<dyn std::error::Error + Send + Sync>;

///
/// 定义简单的 Result
///
pub type Res<T> = Result<T,EBox>;

