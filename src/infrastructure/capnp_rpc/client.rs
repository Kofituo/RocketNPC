use crate::hello_world_capnp::hello_world;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use std::marker::PhantomData;
use std::net::{SocketAddr, ToSocketAddrs};

use futures::AsyncReadExt;
use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tokio::task::LocalSet;

pub struct RpcClient {
    receiver: oneshot::Receiver<String>,
}

impl RpcClient {
    fn new(message: String) -> Self {
        let (sender, receiver) = oneshot::channel();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        let server_addr: String = "127.0.0.1:4000".to_string();
        let addr = server_addr
            .to_socket_addrs()
            .unwrap()
            .next()
            .expect("could not parse address");
        std::thread::spawn(move || {
            let local = LocalSet::new();
            local.spawn_local(async move {
                let stream = rocket::tokio::net::TcpStream::connect(&addr).await?;
                stream.set_nodelay(true).unwrap();
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let rpc_network = Box::new(twoparty::VatNetwork::new(
                    futures::io::BufReader::new(reader),
                    futures::io::BufWriter::new(writer),
                    rpc_twoparty_capnp::Side::Client,
                    Default::default(),
                ));
                let rpc_system = RpcSystem::new(rpc_network, None);
                let mut rpc_system = rpc_system;
                let hello_world: hello_world::Client =
                    rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
                rocket::tokio::task::spawn_local(rpc_system);

                let mut request = hello_world.say_hello_request();
                request.get().init_request().set_name(&message[..]);
                let reply = request.send().promise.await?;
                let reply_message = reply.get()?.get_reply()?.get_message()?.to_str()?;
                println!("received: {}", reply_message);
                //send the message to the receiver
                sender.send(reply_message.to_string()).unwrap();
                Ok::<(), Box<dyn std::error::Error>>(())
            });
            rt.block_on(local);
        });
        Self { receiver }
    }

    async fn get_response(mut self) -> Result<String, String> {
        match self.receiver.await {
            Ok(response) => Ok(response),
            Err(error) => Err(error.to_string()),
        }
    }
}
pub async fn run_client(message: String) -> Result<String, String> {
    let mut client = RpcClient::new(message);
    client.get_response().await
}
