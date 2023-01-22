/*
    Appellation: mainnet <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{behaviour::*, client::*, commands::*, events::*, network::*};

pub(crate) mod behaviour;
pub(crate) mod client;
pub(crate) mod commands;
pub(crate) mod events;
pub(crate) mod network;

use crate::protocol::{codec::MainnetCodec, MainnetProtocol};

use futures::{channel::mpsc, Stream};
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::request_response::{self, ProtocolSupport};
use libp2p::swarm::Swarm;
use std::iter;

/// Creates the network components, namely:
///
/// - The network client to interact with the network layer from anywhere
///   within your application.
///
/// - The network event stream, e.g. for incoming requests.
///
/// - The network task driving the network itself.
pub async fn new(
    secret_key_seed: Option<u8>,
) -> Result<(Client, impl Stream<Item = Event>, EventLoop), Box<dyn std::error::Error>> {
    let peer = match secret_key_seed {
        Some(v) => crate::peers::Peer::try_from(v)?,
        None => crate::peers::Peer::default()
    };
    


    let transport = crate::transports::tokio_transport(true, peer.clone().keypair);

    // Build the Swarm, connecting the lower layer transport logic with the
    // higher layer network behaviour logic.
    let swarm = Swarm::with_threadpool_executor(
        transport,
        MainnetBehaviour {
            kademlia: Kademlia::new(peer.clone().id, MemoryStore::new(peer.clone().id)),
            reqres: request_response::RequestResponse::new(
                MainnetCodec(),
                iter::once((MainnetProtocol(), ProtocolSupport::Full)),
                Default::default(),
            ),
        },
        peer.id,
    );

    let (command_sender, command_receiver) = mpsc::channel(0);
    let (event_sender, event_receiver) = mpsc::channel(0);

    Ok((
        Client::new(command_sender),
        event_receiver,
        EventLoop::new(swarm, command_receiver, event_sender),
    ))
}

pub async fn startup(addr: libp2p::Multiaddr) -> scsys::AsyncResult {
    let id_keys = libp2p::identity::Keypair::generate_ed25519();
    let peer_id = libp2p::PeerId::from(id_keys.public());
    let (mut netclient, mut _netevents, network_event_loop) =
        new(None).await.expect("Failed to startup the p2p network");

    // Spawn the network task for it to run in the background.
    tokio::spawn(network_event_loop.run());

    netclient
        .start_listening(addr.clone())
        .await
        .expect("Listening not to fail.");

    netclient
        .dial(peer_id, addr)
        .await
        .expect("Dial to succeed");
    Ok(())
}
