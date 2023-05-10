use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub token: String,
    pub agent: Agent,
    pub contract: Contract,
    pub faction: Faction,
    pub ship: Ship,
}

#[derive(Serialize, Deserialize)]
pub struct Agent {
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i32
}

#[derive(Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub faction_symbol: String,
    pub contact_type: String,
    pub terms: ContractTerm,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct ContractTerm {
    pub deadline: DateTime<Utc>,
    pub payment: ContractTermPayment,
    pub deliver: Vec<ContractTermDeliverable>
}

#[derive(Serialize, Deserialize)]
pub struct ContractTermPayment {
    pub on_accepted: i32,
    pub on_fulfilled: i32
}

#[derive(Serialize, Deserialize)]
pub struct ContractTermDeliverable {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32
}

#[derive(Serialize, Deserialize)]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<FactionTrait>
}

#[derive(Serialize, Deserialize)]
pub struct FactionTrait {
    pub symbol: String,
    pub name: String,
    pub description: String
}

#[derive(Serialize, Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub nav: ShipNav,
    pub crew: Crew,
    pub fuel: Fuel,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
    pub modules: Vec<ShipModule>,
    pub mounts: Vec<ShipMount>,
    pub registration: ShipRegistration,
    pub cargo: ShipCargo
}

#[derive(Serialize, Deserialize)]
pub struct ShipNav {
    pub system_symbol: String,
    pub waypoint_symbol: String,

}

#[derive(Serialize, Deserialize)]
pub struct NavRoute {
    pub departure: Location,
    pub destination: Location,
    pub arrival: String,
    pub departure_time: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub symbol: String,
    pub loc_type: String,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32
}

#[derive(Serialize, Deserialize)]
pub struct Crew {
    pub current: i32,
    pub capacity: i32,
    pub required: i32,
    pub rotation: String,
    pub morale: i32,
    pub wages: i32
}

#[derive(Serialize, Deserialize)]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
    pub consumed: FuelConsumption
}

#[derive(Serialize, Deserialize)]
pub struct FuelConsumption {
    pub amount: i32,
    pub timestamp: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct Frame {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub fuel_capacity: i32,
    pub condition: i32,
    pub requirements: HashMap<String, i32>
}

#[derive(Serialize, Deserialize)]
pub struct Reactor {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: i32,
    pub power_output: i32,
    pub requirements: HashMap<String, i32>
}

#[derive(Serialize, Deserialize)]
pub struct Engine {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub condition: i32,
    pub speed: i32,
    pub requirements: HashMap<String, i32>
}

#[derive(Serialize, Deserialize)]
pub struct ShipModule {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub capacity: i32,
    pub requirements: HashMap<String, i32>
}

#[derive(Serialize, Deserialize)]
pub struct ShipMount {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub strength: i32,
    pub requirements: HashMap<String, i32>,
    pub deposits: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct ShipRegistration {
    pub name: String,
    pub faction_symbol: String,
    pub role: String
}

#[derive(Serialize, Deserialize)]
pub struct ShipCargo {
    pub capacity: i32,
    pub units: i32,
    pub inventory: Vec<ShipCargoInventory>
}

#[derive(Serialize, Deserialize)]
pub struct ShipCargoInventory {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: i32
}