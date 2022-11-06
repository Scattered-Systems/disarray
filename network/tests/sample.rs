#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use libp2p::{
        core::upgrade,
        floodsub::{self, Floodsub, FloodsubEvent},
        identity,
        mdns::{
            MdnsEvent,
            // `TokioMdns` is available through the `mdns-tokio` feature.
            TokioMdns,
        },
        mplex, noise,
        swarm::{Swarm, SwarmBuilder, SwarmEvent},
        tcp, Multiaddr, NetworkBehaviour, PeerId, Transport,
    };
    use scsys::prelude::BoxResult;
    use std::error::Error;
    use tokio::io::{self, AsyncBufReadExt};

    // We create a custom  behaviour that combines floodsub and mDNS.
    // The derive generates a delegating `NetworkBehaviour` impl.
    #[derive(NetworkBehaviour)]
    #[behaviour(out_event = "ChatBehaviourEvent")]
    struct ChatBehaviour {
        floodsub: Floodsub,
        mdns: TokioMdns,
    }

    #[allow(clippy::large_enum_variant)]
    enum ChatBehaviourEvent {
        Floodsub(FloodsubEvent),
        Mdns(MdnsEvent),
    }

    impl From<FloodsubEvent> for ChatBehaviourEvent {
        fn from(event: FloodsubEvent) -> Self {
            Self::Floodsub(event)
        }
    }

    impl From<MdnsEvent> for ChatBehaviourEvent {
        fn from(event: MdnsEvent) -> Self {
            Self::Mdns(event)
        }
    }

    async fn chat_engine(swarm: &mut Swarm<ChatBehaviour>, topic: Option<&str>) -> BoxResult {
        let floodsub_topic = floodsub::Topic::new(topic.unwrap_or("chat"));
        let mut stdin = io::BufReader::new(io::stdin()).lines();
        loop {
            tokio::select! {
                line = stdin.next_line() => {
                    let line = line?.expect("stdin closed");
                    swarm.behaviour_mut().floodsub.publish(floodsub_topic.clone(), line.as_bytes());
                }
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("Listening on {:?}", address);
                        }
                        SwarmEvent::Behaviour(ChatBehaviourEvent::Floodsub(FloodsubEvent::Message(message))) => {
                            println!(
                                    "Received: '{:?}' from {:?}",
                                    String::from_utf8_lossy(&message.data),
                                    message.source
                                );
                        }
                        SwarmEvent::Behaviour(ChatBehaviourEvent::Mdns(event)) => {
                            match event {
                                MdnsEvent::Discovered(list) => {
                                    for (peer, _) in list {
                                        swarm.behaviour_mut().floodsub.add_node_to_partial_view(peer);
                                    }
                                }
                                MdnsEvent::Expired(list) => {
                                    for (peer, _) in list {
                                        if !swarm.behaviour().mdns.has_node(&peer) {
                                            swarm.behaviour_mut().floodsub.remove_node_from_partial_view(&peer);
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// The `tokio::main` attribute sets up a tokio runtime.
    #[tokio::test]
    async fn chat_tokio_test() -> Result<(), Box<dyn Error>> {
        env_logger::init();

        // Create a random PeerId
        let id_keys = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());
        println!("Local peer id: {:?}", peer_id);

        // Create a tokio-based TCP transport use noise for authenticated
        // encryption and Mplex for multiplexing of substreams on a TCP stream.
        let transport = tcp::TokioTcpTransport::new(tcp::GenTcpConfig::default().nodelay(true))
            .upgrade(upgrade::Version::V1)
            .authenticate(
                noise::NoiseAuthenticated::xx(&id_keys)
                    .expect("Signing libp2p-noise static DH keypair failed."),
            )
            .multiplex(mplex::MplexConfig::new())
            .boxed();

        // Create a Floodsub topic
        let floodsub_topic = floodsub::Topic::new("chat");

        // Create a Swarm to manage peers and events.
        let mut swarm = {
            let mdns = TokioMdns::new(Default::default())?;
            let mut behaviour = ChatBehaviour {
                floodsub: Floodsub::new(peer_id),
                mdns,
            };

            behaviour.floodsub.subscribe(floodsub_topic.clone());

            SwarmBuilder::new(transport, behaviour, peer_id)
                // We want the connection background tasks to be spawned
                // onto the tokio runtime.
                .executor(Box::new(|fut| {
                    tokio::spawn(fut);
                }))
                .build()
        };

        // Reach out to another node if specified
        if let Some(to_dial) = std::env::args().nth(1) {
            let addr: Multiaddr = to_dial.parse()?;
            swarm.dial(addr)?;
            println!("Dialed {:?}", to_dial);
        }

        // Read full lines from stdin
        let mut stdin = io::BufReader::new(io::stdin()).lines();

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        Ok(())
    }
}
