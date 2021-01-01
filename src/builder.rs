extern crate crypto;

use self::crypto::buffer::{BufferResult, WriteBuffer, ReadBuffer};
use crate::prelude::{INITIALIZE_AES_KEY_IV, MESSAGE_BUFF_SIZE};


///
/// Key 构建器
///
pub struct KeyBuilder;


impl KeyBuilder{
    ///
    /// 编码心跳包数据
    ///
    /// 获取心跳包返回的 token 16位字节字符串, 需要对其进行 AES-CBC-128 加密.
    /// ```
    /// let key = "0987654321qwerty";
    /// let token = "1234567890abcdef";
    /// // 加密密文为: 0x3E,0xB4,0x3E,0x37,0xC2,0x0A,0xFF,0x4C,0x58,0x72,0xCC,0x0D,0x04,0xD8,0x13,0x14
    /// // 转换ASCII: 3EB43E37C20AFF4C5872CC0D04D81314
    /// aqara_rs::builder::KeyBuilder::encode(key.as_bytes(),token.as_bytes());
    /// ```
    ///
    pub fn encode(key:&[u8],token:&[u8])->Result<Vec<u8>,crypto::symmetriccipher::SymmetricCipherError>{

        // 生成通用 Encryptor, 这里不需要填充, 直接获取 16 位字节
        let mut encryptor = crypto::aes::cbc_encryptor(
            crypto::aes::KeySize::KeySize128,
            key,
            INITIALIZE_AES_KEY_IV.as_ref(),
            crypto::blockmodes::NoPadding
        );

        // 准备 token[ 写入器 ]
        let mut result_final = Vec::<u8>::new();
        let mut buffer_read = crypto::buffer::RefReadBuffer::new(token);

        // 准备接收数据[ 读取器 ]
        let mut buffer = [0;MESSAGE_BUFF_SIZE];
        let mut buffer_write = crypto::buffer::RefWriteBuffer::new(&mut buffer);

        // 循环分析数据
        loop {
            let result = encryptor.encrypt(&mut buffer_read,&mut buffer_write,true)?;
            result_final.extend(buffer_write.take_read_buffer().take_remaining().iter().map(|&i| i));
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => { },
            }
        }
        Ok(result_final)
    }

    ///
    /// 将字节位转化成 ASCII 字符串
    ///
    pub fn hex2dex(hex:&Vec<u8>)->String{
        let mut dex = String::new();
        for x in hex.iter() {
            dex.push_str(format!("{:X}",x).as_str());
        }
        dex
    }


    ///
    /// 编码心跳包数据
    ///
    /// 获取心跳包返回的 token 16位字节字符串, 需要对其进行 AES-CBC-128 加密.
    /// 加密成 16 位字节 Key 之后需要再转化成 32 位 ASCII 字符串
    ///
    /// ```
    /// let key = "0987654321qwerty";
    /// let token = "1234567890abcdef";
    /// let msg = aqara_rs::builder::KeyBuilder::encode_str(key,token).unwrap();
    /// println!("Builder Key = {}",msg);
    /// ```
    ///
    pub fn encode_str(gateway_key:&str,token:&str)->Result<String,crypto::symmetriccipher::SymmetricCipherError>{
        let buf = Self::encode(gateway_key.as_bytes(),token.as_bytes())?;
        Ok(Self::hex2dex(&buf))
    }
}
