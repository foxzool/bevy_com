use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use bevy_octopus::network::ConnectTo;
use bevy_octopus::transformer::{BincodeTransformer, JsonTransformer, NetworkMessageTransformer};

use crate::common::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    let mut app = App::new();

    shared_setup(&mut app);

    app.add_transformer::<PlayerInformation, JsonTransformer>(JSON_CHANNEL)
        .add_transformer::<PlayerInformation, BincodeTransformer>(BINCODE_CHANNEL)
        .add_systems(Startup, setup_clients)
        .add_systems(
            Update,
            (
                send_raw_message_to_channel,
                send_json_message,
                // send_bincode_message,
            )
                .run_if(on_timer(Duration::from_secs_f64(1.0))),
        )
        .add_systems(
            Update,
            (handle_raw_packet, handle_message_events, handle_node_events),
        )
        .run()
}

fn setup_clients(mut commands: Commands) {
    commands.spawn((RAW_CHANNEL, ConnectTo::new("ws://127.0.0.1:38551")));
    commands.spawn((RAW_CHANNEL, ConnectTo::new("ws://127.0.0.1:7003")));
    commands.spawn((JSON_CHANNEL, ConnectTo::new("ws://127.0.0.1:7004")));
    commands.spawn((BINCODE_CHANNEL, ConnectTo::new("ws://127.0.0.1:7005")));
}
