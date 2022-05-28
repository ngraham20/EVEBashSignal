use serde::{Serialize, Deserialize};
use tungstenite::{connect, Message, stream::MaybeTlsStream, WebSocket};
use url::{Url};
use crate::error::Error;

pub struct KillMailStreamBuilder {
    character_id: Option<usize>,
    corporation_id: Option<usize>,
    alliance_id: Option<usize>,
}
impl KillMailStreamBuilder {
    pub fn builder() -> Self {
        KillMailStreamBuilder {
            character_id: None,
            corporation_id: None,
            alliance_id: None
        }
    }
    pub fn character_id(mut self, id: usize) -> Result<Self, Error>  {
        self.character_id = Some(id);
        Ok(self)
    }
    pub fn corporation_id(mut self, id: usize) -> Result<Self, Error> {
        self.corporation_id = Some(id);
        Ok(self)
    }
    pub fn alliance_id(mut self, id: usize) -> Result<Self, Error> {
        self.alliance_id = Some(id);
        Ok(self)
    }
    pub fn build(&self) -> KillMailStream {
        KillMailStream {
            socket: None
        }
    }
}

pub struct KillMailStream {
    socket: Option<WebSocket<MaybeTlsStream<std::net::TcpStream>>>
}

impl KillMailStream {

    pub fn connect(&mut self) -> Result<(), Error> {
        let (mut socket, _response)
            = connect(Url::parse("wss://zkillboard.com/websocket/").unwrap()).expect("Can't connect");
        self.socket = Some(socket);
        Ok(())
    }
    pub fn send_message(&mut self, message: String) -> Result<(), Error> {
        if let Some(s) = self.socket.as_mut() {
            s.write_message(Message::Text(format!("{{\"action\":\"sub\",\"channel\":\"killstream\"}}").into()));
        }
        Ok(())
    }
    /// this should probably be something like "subscribe" or something. It should internally loop
    /// and not just activate once. 
    pub fn receive_message(&mut self) -> Result<(), Error> {
        if let Some(s) = self.socket.as_mut() {
            let msg = s.read_message().expect("Error reading message");
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

        Ok(())
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
    alliance_id: Option<usize>,
    character_id: Option<usize>,
    corporation_id: Option<usize>,
    damage_taken: usize,
    items: Option<Vec<Item>>,
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