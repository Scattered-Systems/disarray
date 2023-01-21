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

use futures::{channel::mpsc, Stream};
use libp2p::kad::{record::store::MemoryStore, Kademlia};
use libp2p::request_response::{self, ProtocolSupport};
use libp2p::{
    core::upgrade,
    identity::{self, ed25519},
    mplex, noise,
    swarm::Swarm,
    Transport,
};
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
    // Create a public/private key pair, either random or based on a seed.
    let id_keys = match secret_key_seed {
        Some(seed) => {
            let mut bytes = [0u8; 32];
            bytes[0] = seed;
            let secret_key = ed25519::SecretKey::from_bytes(&mut bytes).expect(
                "this returns `Err` only if the length is wrong; the length is correct; qed",
            );
            identity::Keypair::Ed25519(secret_key.into())
        }
        None => identity::Keypair::generate_ed25519(),
    };
    let peer_id = id_keys.public().to_peer_id();

    let transport =
        libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(
                noise::NoiseAuthenticated::xx(&id_keys)
                    .expect("Signing libp2p-noise static DH keypair failed."),
            )
            .multiplex(mplex::MplexConfig::new())
            .boxed();

    // Build the Swarm, connecting the lower layer transport logic with the
    // higher layer network behaviour logic.
    let swarm = Swarm::with_threadpool_executor(
        transport,
        MainnetBehaviour {
            kademlia: Kademlia::new(peer_id, MemoryStore::new(peer_id)),
            reqres: request_response::RequestResponse::new(
                MainnetCodec(),
                iter::once((MainnetProtocol(), ProtocolSupport::Full)),
                Default::default(),
            ),
        },
        peer_id,
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
