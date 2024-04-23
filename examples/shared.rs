#![allow(dead_code)]

use std::{ops::Deref, time::Duration};

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use serde::{Deserialize, Serialize};

use bevy_ecs_net::prelude::*;

#[derive(Component)]
pub struct ClientMarker;

#[derive(Component)]
pub struct ServerMarker;

#[derive(Component)]
pub struct RawPacketMarker;

#[derive(Component)]
pub struct JsonMarker;

#[derive(Component)]
pub struct BincodeMarker;

/// shared app setup
pub fn shared_setup(app: &mut App) {
    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
        LogPlugin {
            filter: "bevy_ecs_net=debug".to_string(),
            ..default()
        },
    ))
    .add_plugins(BevyComPlugin);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerInformation {
    pub health: usize,
    pub position: (u32, u32, u32),
}

impl NetworkMessage for PlayerInformation {
    const NAME: &'static str = "PlayerInfo";
}

pub fn handle_error_events(
    mut new_network_events: EventReader<NetworkErrorEvent>,
    q_net_node: Query<&NetworkNode>,
) {
    for event in new_network_events.read() {
        let net = q_net_node.get(event.source).unwrap();
        error!("{:?} got Error: {:?}", net.local_addr, event.error);
    }
}

pub fn handle_message_events(
    mut new_network_events: EventReader<NetworkData<PlayerInformation>>,
    q_net_node: Query<&NetworkNode>,
) {
    for event in new_network_events.read() {
        let net = q_net_node.get(event.source).unwrap();
        let player_info = event.deref();
        info!("{} Received: {:?}", net, &player_info);
    }
}

/// if you recv message directly from receiver,  typed message to wait handled may be missed
pub fn receive_raw_messages(
    q_server: Query<&NetworkNode, (With<ServerMarker>, With<RawPacketMarker>)>,
) {
    for net_node in q_server.iter() {
        while let Ok(Some(packet)) = net_node.recv_channel().receiver.try_recv() {
            info!(
                "{} Received: {:?}",
                net_node,
                String::from_utf8(packet.bytes.to_vec()).unwrap()
            );
        }
    }
}
