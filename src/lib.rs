use std::fmt::{Debug, Display};

use bevy::app::{App, Plugin};
use kanal::{Receiver, Sender, unbounded};

use crate::{network::NetworkErrorEvent, prelude::NetworkResource};

pub mod event;
pub mod manager;
pub mod prelude;

pub mod error;

pub mod decoder;
pub mod network;

pub type ChannelName = String;

pub struct BevyComPlugin;

impl Plugin for BevyComPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetworkResource>()
            .add_event::<NetworkErrorEvent>();

        #[cfg(feature = "udp")]
        app.add_plugins(udp::UdpPlugin);
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

#[cfg(feature = "udp")]
pub mod udp;
