/*
   Appellation: disarray-network <library>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       Disarray is a complete toolkit for building efficient EVM compatible Multi-Chain Networks.
*/
#[doc(inline)]
pub use crate::{address::*, behaviour::*, primitives::*, settings::*, utils::*};
#[doc(inline)]
pub use disarray_minis as minis;

pub(crate) mod address;
pub(crate) mod behaviour;
pub(crate) mod primitives;
pub(crate) mod settings;
pub(crate) mod utils;

pub mod clients;
pub mod events;
pub mod messages;
pub mod nodes;
pub mod peers;
pub mod states;
pub mod status;
pub mod transports;

use clients::Client;
use events::{Event, EventLoop};
use minis::{codec::MainnetCodec, MainnetProtocol};
use peers::Peer;

use futures::{channel::mpsc, Stream};
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::request_response::{self, ProtocolSupport};
use libp2p::swarm::Swarm;
use scsys::AsyncResult;
use serde::{Deserialize, Serialize};
use std::iter;

pub async fn quickstart() -> AsyncResult {
    let network = Network::default();
    network.spawn().await?;

    Ok(())
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NetworkType {
    #[default]
    Mainnet = 0,
    Testnet = 1,
}

pub struct Network {
    pub addr: NetworkAddress,
    pub peer: Peer,
}

impl Network {
    pub fn new(addr: NetworkAddress, peer: Peer) -> Self {
        Self { addr, peer }
    }
    pub async fn setup(
        &self,
    ) -> Result<(Client, impl Stream<Item = Event>, EventLoop), Box<dyn std::error::Error>> {
        // Setup a transport using the initialized peer
        let transport = self.transport();
        // Initialize the network behaviour
        let behaviour = MainnetBehaviour::new(
            request_response::RequestResponse::new(
                MainnetCodec(),
                iter::once((MainnetProtocol(), ProtocolSupport::Full)),
                Default::default(),
            ),
            Kademlia::new(self.peer.clone().id, MemoryStore::new(self.peer.clone().id)),
        );
        // Setup the Swarm; links the lower layer transport logic with the higher layer network behaviour logic
        let swarm = Swarm::with_threadpool_executor(transport, behaviour, self.peer.id);
        // Initialize a Sender / Receiver pair for the commands
        let (command_sender, command_receiver) = mpsc::channel(0);
        // Initialize a Sender / Receiver pair for the events
        let (event_sender, event_receiver) = mpsc::channel(0);

        Ok((
            Client::new(command_sender),
            event_receiver,
            EventLoop::new(swarm, command_receiver, event_sender),
        ))
    }
    pub async fn spawn(&self) -> AsyncResult {
        let (mut netclient, mut _netevents, network_event_loop) =
            self.setup().await.expect("Network setup failed...");

        // Spawn the network task for it to run in the background.
        tokio::spawn(network_event_loop.run());

        netclient
            .start_listening(self.addr.clone().into())
            .await
            .expect("Listening not to fail.");

        netclient
            .dial(self.peer.id, self.addr.clone().into())
            .await
            .expect("Dial to succeed");
        Ok(())
    }
    pub fn transport(&self) -> BoxedTransport {
        crate::tokio_transport(true, self.peer.keypair.clone())
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::from(Peer::default())
    }
}

impl From<Peer> for Network {
    fn from(data: Peer) -> Network {
        Network::new(Default::default(), data)
    }
}
