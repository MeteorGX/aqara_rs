use aqara_rs::builder::{KeyBuilder};

#[test]
fn key_works()->Result<(),Box<crypto::symmetriccipher::SymmetricCipherError>>{
    let key = "0987654321qwerty";
    let token = "1234567890abcdef";
    let msg = KeyBuilder::encode_str(key,token)?;
    println!("Builder Key = {}",msg);
    Ok(())
}




