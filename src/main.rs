use std::collections::HashMap;
use thiserror::*;
use tungstenite::{connect, Message};
use url::Url;
use serde::*;
use serde_json::*;

fn main() {
    let (mut socket, _response) =
        connect(Url::parse("wss://zkillboard.com/websocket/").unwrap()).expect("Can't connect");

    socket.write_message(Message::Text(r#"{"action":"sub","channel":"killstream"}"#.into())).unwrap();

    loop {
        let msg = socket.read_message().expect("Error reading message");

        // if it's text, it's json. convert it into the objects and print those.
        // if it's a ping, don't do anything, it's blank data.
        // if it's something else, panic, idk what to do with that.

        match msg {
            tungstenite::Message::Text(s) => {
                let jdata: serde_json::Value = serde_json::from_str(&s).unwrap();
                println!("{:?}", jdata);
                let km: KillMail = serde_json::from_value(jdata).unwrap();
                println!("{:?}", km);
            }
            tungstenite::Message::Ping(m) => {println!("Ping: {:?}", m)}
            _ => { panic!() }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct KillMail {
    attackers: Vec<Attacker>,
    killmail_id: usize,
    killmail_time: String,
    solar_system_id: usize,
    victim: Victim,
}

#[derive(Serialize, Deserialize, Debug)]
struct ZKillBoard {
    #[serde(rename(deserialize = "locationID"))]
    location_id: usize,

    hash: String,
    #[serde(rename(deserialize = "fittedValue"))]
    fitted_value: f64,

    #[serde(rename(deserialize = "droppedValue"))]
    dropped_value: f64,

    #[serde(rename(deserialize = "destroyedValue"))]
    destroyed_value: f64,

    #[serde(rename(deserialize = "totalValue"))]
    total_value: f64,
    
    points: usize,
    npc: bool,
    solo: bool,
    awox: bool,
    labels: Vec<String>,
    href: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Attacker {
    alliance_id: Option<usize>,
    character_id: Option<usize>,
    corporation_id: Option<usize>,
    faction_id: Option<usize>,
    final_blow: bool,
    security_status: f32,
    ship_type_id: usize,
    weapon_type_id: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct Victim {
    alliance_id: usize,
    character_id: usize,
    corporation_id: usize,
    damage_taken: usize,
    items: Vec<Item>,
    ship_type_id: usize
}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    flag: usize,
    item_type_id: usize,
    quantity_destroyed: Option<usize>,
    singleton: usize,
    position: Option<Position>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64
}