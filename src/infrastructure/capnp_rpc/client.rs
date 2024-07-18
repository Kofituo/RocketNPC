use std::fmt::Debug;
use std::marker::PhantomData;
use std::net::ToSocketAddrs;

use capnp::capability::{Promise, Response};
use capnp::text::Reader;
use capnp::Error;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use serde::Serialize;
use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tokio::task::LocalSet;

use crate::hello_world_capnp::hello_world;
use crate::hello_world_capnp::hello_world::say_hello_results;

pub struct RpcClient<Client: RpcResponse> {
    receiver: oneshot::Receiver<Client::OutputJson>,
    phantom_data: PhantomData<fn() -> Client>,
}

impl<Client: RpcResponse> RpcClient<Client> {
    fn new(message: Client::InputData) -> Self {
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
                let reply = Client::new(rpc_system).get_promise(message).await?;
                let reply_message = Client::extract_response(reply)?;
                println!("received: {:?}", reply_message);
                //send the message to the receiver
                sender.send(reply_message).unwrap();
                Ok::<(), Box<dyn std::error::Error>>(())
            });
            rt.block_on(local);
        });
        Self {
            receiver,
            phantom_data: Default::default(),
        }
    }

    async fn get_response(mut self) -> Result<Client::OutputJson, String> {
        match self.receiver.await {
            Ok(response) => Ok(response),
            Err(error) => Err(error.to_string()),
        }
    }
}

pub trait RpcResponse {
    type InputData: Send + 'static;
    type CapNpResult;
    type OutputJson: Serialize + Debug + Send + 'static;
    fn new(rpc_system: RpcSystem<rpc_twoparty_capnp::Side>) -> Self;
    fn get_promise(self, data: Self::InputData) -> Promise<Response<Self::CapNpResult>, Error>;
    // get raw reponse
    fn extract_response(response: Response<Self::CapNpResult>) -> capnp::Result<Self::OutputJson>;
}

// sample of how to implement the above trait
impl RpcResponse for hello_world::Client {
    type InputData = String;
    type CapNpResult = say_hello_results::Owned;
    type OutputJson = String;

    fn new(rpc_system: RpcSystem<rpc_twoparty_capnp::Side>) -> Self {
        new_capnp_client(rpc_system)
    }
    fn get_promise(self, message: Self::InputData) -> Promise<Response<Self::CapNpResult>, Error> {
        let mut request = self.say_hello_request();
        request.get().init_request().set_name(&message[..]);
        request.send().promise
    }

    fn extract_response(response: Response<Self::CapNpResult>) -> capnp::Result<String> {
        response
            .get()?
            .get_reply()?
            .get_message()?
            .to_string()
            .map_err(|e| capnp::Error::failed(e.to_string()))
    }
}

// helper function to generate new cnp client
pub fn new_capnp_client<T: capnp::capability::FromClientHook>(
    rpc_system: RpcSystem<rpc_twoparty_capnp::Side>,
) -> T {
    let mut rpc_system = rpc_system;
    let out = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    rocket::tokio::task::spawn_local(rpc_system);
    out
}

// helper method
pub fn get_string_from_reader(reader: Reader) -> capnp::Result<String> {
    reader
        .to_string()
        .map_err(|e| capnp::Error::failed(e.to_string()))
}

pub async fn run_client<Client: RpcResponse>(
    message: Client::InputData,
) -> Result<Client::OutputJson, String> {
    let mut client: RpcClient<Client> = RpcClient::new(message);
    client.get_response().await
}
