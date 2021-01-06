use aqara_rs::prelude::{Res, ResponseEvent};
use aqara_rs::session::{Multicast, Unicast};
use aqara_rs::device::Gateway;

struct Echo;

impl Echo{
    fn new() -> Box<Self> {
        Box::new(Self{})
    }
}

impl ResponseEvent for Echo{


    fn join_multicast(&self,ctx:Vec<u8>,client:Multicast){
        println!("[multicast] Vec = {:?}",ctx);
        client.send(ctx.as_slice()).unwrap();
    }

    fn join_unicast(&self,ctx:Vec<u8>,client:Unicast){
        println!("[unicast] Vec = {:?}",ctx);

        client.send(ctx.as_slice()).unwrap();
    }
}

fn main() -> Res<()> {

    let server = Gateway::with_capacity(1024)?;
    server.run(Echo::new())?;

    Ok(())
}
