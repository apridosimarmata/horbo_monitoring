use std::net::UdpSocket;
use super::horbo::{Metrics, ProtocolDefinition};

pub fn send_metrics(ip: &str, def: &ProtocolDefinition, data: Metrics) -> std::io::Result<()> {
    /* creating socket */
    let destination = format!("{}:{}", ip, def.dest_port);
    let socket = UdpSocket::bind(format!("{}:{}", ip, def.source_port)).expect("can't use source port");

    let mut byte_stream: Vec<u8> =Vec::new();

    /* start flag */
    match u8::from_str_radix(&def.start_flag, 16) {
        Ok(value) => {
            byte_stream.push(value); 
        }
        Err(e) => {
            eprintln!("Error parsing hex string: {}", e);
        }
    }

    /* protocol version */
    byte_stream.push(def.protocol_version.to_be_bytes()[0]);


    /* building message body + length */
    let mut message_body: Vec<u8> = Vec::new();
    let mut msg_length:usize = def.message_length.reserved_bytes + def.start_flag.len();

    for item in data.items{
        /* check if metric is defined in protocol definition */
        let id = match def.message_body.get(&item.0){
            Some(field) => {field.id as u8},
            None =>{
                panic!("metric is not recognized: \"{}\"", item.0)
            }
        };


        /* construct metric bytes with following format: [id] [length] [usage] */
        let mut metric_bytes : Vec<u8> = id.to_be_bytes().into_iter().collect();
        let mut usage : Vec<u8> = item.1.usage.to_be_bytes().into_iter().collect();
        let mut length : Vec<u8> = (usage.len() as u8).to_be_bytes().into_iter().collect();
        metric_bytes.append(&mut length);
        metric_bytes.append(&mut usage);

        msg_length+= metric_bytes.len();
        message_body.append(&mut metric_bytes);
    }

    let mut msg_length_bytes: Vec<u8>  = (msg_length as u16).to_be_bytes().into_iter().collect();

    /* append length first and the body */
    byte_stream.append(&mut msg_length_bytes);
    byte_stream.append(&mut message_body);

    /* send metrics */
    match socket.send_to(&byte_stream, destination) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}
