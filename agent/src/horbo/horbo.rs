use serde::Deserialize; use std::collections::HashMap;
// Import the trait
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Metrics{
    pub items: HashMap<String, Metric>,
}

#[derive(Debug)]
pub struct Metric {
    pub usage: f32,
}

#[derive(Debug, Deserialize)]
pub struct MessageField {
    pub id: usize,
}

#[derive(Debug, Deserialize)]
pub struct MessageLength {
    pub reserved_bytes: usize,
}

#[derive(Debug, Deserialize)]
struct RootProto {
    protocol: ProtocolDefinition,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolDefinition {
    pub source_port: String,
    pub dest_port: String,
    pub start_flag: String,
    pub message_length: MessageLength,
    pub protocol_version: u8,
    pub message_body: HashMap<String, MessageField>,
}

fn load_protocol_definition(filepath: &str) -> Result<ProtocolDefinition, io::Error> {
    let contents = fs::read_to_string(filepath)?;
    let root: RootProto = serde_yaml::from_str(&contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(root.protocol)
}

pub fn init() -> Result<ProtocolDefinition, io::Error> {
    let filepath = "horbo.yaml"; // Create a file named protocol.yaml with your YAML content
    match load_protocol_definition(filepath) {
        Ok(definition) => {
            Ok(definition)
        }
        Err(e) => {
            Err(e)
        }
    }
}