#![doc = include_str!("../README.md")]
// #![warn(missing_docs)]

use std::fmt::{Debug, Display};

use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Entity, EventWriter, Query};
use kanal::{unbounded, Receiver, Sender};

use crate::{network::NetworkEvent, prelude::NetworkResource};
use crate::network::NetworkNode;

pub mod decoder;
pub mod error;
pub mod event;
pub mod manager;
pub mod network;
pub mod prelude;

#[cfg(feature = "udp")]
pub mod udp;

#[cfg(feature = "tcp")]
pub mod tcp;

pub type ChannelName = String;

pub struct BevyComPlugin;

impl Plugin for BevyComPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkResource>()
            .add_event::<NetworkEvent>()
            .add_systems(Update, node_error_event);

        #[cfg(feature = "udp")]
        app.add_plugins(udp::UdpPlugin);

        #[cfg(feature = "tcp")]
        app.add_plugins(tcp::TcpPlugin);
    }
}

#[derive()]
pub struct AsyncChannel<T> {
    pub sender: Sender<T>,
    pub receiver: Receiver<T>,
}

impl<T> AsyncChannel<T> {
    fn new() -> Self {
        let (sender, receiver) = unbounded();

        Self { sender, receiver }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
/// A [`ConnectionId`] denotes a single connection
pub struct ConnectionId {
    /// The key of the connection.
    pub id: u32,
}

impl Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Connection with ID={0}", self.id))
    }
}



/// send network node error channel to events
fn node_error_event(q_net: Query<(Entity ,&NetworkNode)>, mut node_events: EventWriter<NetworkEvent>) {
    for (entity, net_node) in q_net.iter() {
        while let Ok(Some(error)) = net_node.error_channel().receiver.try_recv() {
            node_events.send(NetworkEvent::Error(entity, error));
        }
    }
}