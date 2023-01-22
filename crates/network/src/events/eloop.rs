/*
    Appellation: events <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use crate::events::{Event, MainnetEvent};
use crate::protocol::reqres::{MainnetRequest, MainnetResponse};
use crate::{clients::Command, MainnetBehaviour};

use futures::channel::{mpsc, oneshot};
use futures::StreamExt;
use libp2p::core::{either::EitherError, PeerId};
use libp2p::kad::{GetProvidersOk, KademliaEvent, QueryId, QueryResult};
use libp2p::multiaddr::Protocol;
use libp2p::request_response::{self, RequestId};
use libp2p::swarm::{ConnectionHandlerUpgrErr, Swarm, SwarmEvent};
use std::collections::{hash_map, HashMap, HashSet};
use std::error::Error;
use tokio::io;

#[derive(Default)]
pub struct Pending {
    dial: HashMap<PeerId, oneshot::Sender<Result<(), Box<dyn Error + Send>>>>,
    start_providing: HashMap<QueryId, oneshot::Sender<()>>,
    get_providers: HashMap<QueryId, oneshot::Sender<HashSet<PeerId>>>,
    request_file: HashMap<RequestId, oneshot::Sender<Result<Vec<u8>, Box<dyn Error + Send>>>>,
}

pub struct EventLoop {
    swarm: Swarm<MainnetBehaviour>,
    command_receiver: mpsc::Receiver<Command>,
    event_sender: mpsc::Sender<Event>,
    pending: Pending,
}

impl EventLoop {
    pub fn new(
        swarm: Swarm<MainnetBehaviour>,
        command_receiver: mpsc::Receiver<Command>,
        event_sender: mpsc::Sender<Event>,
    ) -> Self {
        Self {
            swarm,
            command_receiver,
            event_sender,
            pending: Default::default(),
        }
    }

    pub async fn run(mut self) {
        loop {
            futures::select! {
                event = self.swarm.next() => self.handle_event(event.expect("Swarm stream to be infinite.")).await  ,
                command = self.command_receiver.next() => match command {
                    Some(c) => self.handle_command(c).await,
                    // Command channel closed, thus shutting down the network event loop.
                    None=>  return,
                },
            }
        }
    }

    async fn handle_event(
        &mut self,
        event: SwarmEvent<
            MainnetEvent,
            EitherError<ConnectionHandlerUpgrErr<io::Error>, io::Error>,
        >,
    ) {
        match event {
            SwarmEvent::Behaviour(MainnetEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed {
                    id,
                    result: QueryResult::StartProviding(_),
                    ..
                },
            )) => {
                let sender: oneshot::Sender<()> = self
                    .pending
                    .start_providing
                    .remove(&id)
                    .expect("Completed query to be previously pending.");
                let _ = sender.send(());
            }
            SwarmEvent::Behaviour(MainnetEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed {
                    id,
                    result:
                        QueryResult::GetProviders(Ok(GetProvidersOk::FoundProviders {
                            providers, ..
                        })),
                    ..
                },
            )) => {
                if let Some(sender) = self.pending.get_providers.remove(&id) {
                    sender.send(providers).expect("Receiver not to be dropped");

                    // Finish the query. We are only interested in the first result.
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .query_mut(&id)
                        .unwrap()
                        .finish();
                }
            }
            SwarmEvent::Behaviour(MainnetEvent::Kademlia(
                KademliaEvent::OutboundQueryProgressed {
                    result:
                        QueryResult::GetProviders(Ok(GetProvidersOk::FinishedWithNoAdditionalRecord {
                            ..
                        })),
                    ..
                },
            )) => {}
            SwarmEvent::Behaviour(MainnetEvent::Kademlia(_)) => {}
            SwarmEvent::Behaviour(MainnetEvent::RequestResponse(
                request_response::RequestResponseEvent::Message { message, .. },
            )) => match message {
                request_response::RequestResponseMessage::Request {
                    request, channel, ..
                } => {
                    self.event_sender
                        .try_send(Event::InboundRequest {
                            request: request.0,
                            channel,
                        })
                        .expect("Event receiver not to be dropped.");
                }
                request_response::RequestResponseMessage::Response {
                    request_id,
                    response,
                } => {
                    let _ = self
                        .pending
                        .request_file
                        .remove(&request_id)
                        .expect("Request to still be pending.")
                        .send(Ok(response.0));
                }
            },
            SwarmEvent::Behaviour(MainnetEvent::RequestResponse(
                request_response::RequestResponseEvent::OutboundFailure {
                    request_id, error, ..
                },
            )) => {
                let _ = self
                    .pending
                    .request_file
                    .remove(&request_id)
                    .expect("Request to still be pending.")
                    .send(Err(Box::new(error)));
            }
            SwarmEvent::Behaviour(MainnetEvent::RequestResponse(
                request_response::RequestResponseEvent::ResponseSent { .. },
            )) => {}
            SwarmEvent::NewListenAddr { address, .. } => {
                let local_peer_id = *self.swarm.local_peer_id();
                eprintln!(
                    "Local node is listening on {:?}",
                    address.with(Protocol::P2p(local_peer_id.into()))
                );
            }
            SwarmEvent::IncomingConnection { .. } => {}
            SwarmEvent::ConnectionEstablished {
                peer_id, endpoint, ..
            } => {
                if endpoint.is_dialer() {
                    if let Some(sender) = self.pending.dial.remove(&peer_id) {
                        let _ = sender.send(Ok(()));
                    }
                }
            }
            SwarmEvent::ConnectionClosed { .. } => {}
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                if let Some(peer_id) = peer_id {
                    if let Some(sender) = self.pending.dial.remove(&peer_id) {
                        let _ = sender.send(Err(Box::new(error)));
                    }
                }
            }
            SwarmEvent::IncomingConnectionError { .. } => {}
            SwarmEvent::Dialing(peer_id) => eprintln!("Dialing {peer_id}"),
            e => panic!("{e:?}"),
        }
    }

    async fn handle_command(&mut self, command: Command) {
        match command {
            Command::StartListening { addr, sender } => {
                let _ = match self.swarm.listen_on(addr) {
                    Ok(_) => sender.send(Ok(())),
                    Err(e) => sender.send(Err(Box::new(e))),
                };
            }
            Command::Dial {
                peer_id,
                peer_addr,
                sender,
            } => {
                if let hash_map::Entry::Vacant(e) = self.pending.dial.entry(peer_id) {
                    self.swarm
                        .behaviour_mut()
                        .kademlia
                        .add_address(&peer_id, peer_addr.clone());
                    match self
                        .swarm
                        .dial(peer_addr.with(Protocol::P2p(peer_id.into())))
                    {
                        Ok(()) => {
                            e.insert(sender);
                        }
                        Err(e) => {
                            let _ = sender.send(Err(Box::new(e)));
                        }
                    }
                } else {
                    todo!("Already dialing peer.");
                }
            }
            Command::StartProviding { file_name, sender } => {
                let query_id = self
                    .swarm
                    .behaviour_mut()
                    .kademlia
                    .start_providing(file_name.into_bytes().into())
                    .expect("No store error.");
                self.pending.start_providing.insert(query_id, sender);
            }
            Command::GetProviders { file_name, sender } => {
                let query_id = self
                    .swarm
                    .behaviour_mut()
                    .kademlia
                    .get_providers(file_name.into_bytes().into());
                self.pending.get_providers.insert(query_id, sender);
            }
            Command::RequestFile {
                file_name,
                peer,
                sender,
            } => {
                let request_id = self
                    .swarm
                    .behaviour_mut()
                    .reqres
                    .send_request(&peer, MainnetRequest(file_name));
                self.pending.request_file.insert(request_id, sender);
            }
            Command::RespondFile { file, channel } => {
                self.swarm
                    .behaviour_mut()
                    .reqres
                    .send_response(channel, MainnetResponse(file))
                    .expect("Connection to peer to be still open.");
            }
        }
    }
}
