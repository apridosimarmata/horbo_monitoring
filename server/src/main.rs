use  std::{collections::HashMap, net::UdpSocket};

use horbo::{horbo::{Metrics, ProtocolDefinition}, receive::unpack_metrics};
mod horbo;

fn main() {
    let x = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");
    let definition : ProtocolDefinition = match horbo::horbo::init(){
        Ok(def) => def,
        Err(e) => {
            panic!("{}", e)
        }
    };

    let mut buffer: [u8; 1024] = [0; 1024]; // You can adjust the buffer size as needed

    let mut metrics_id_map: HashMap<usize, String> =  HashMap::new();

    for item in definition.message_body.iter(){
        metrics_id_map.insert(item.1.id, item.0.clone());
    }

    loop {
        match x.recv_from(&mut buffer) {
            Ok((bytes_received, sender_address)) => {
                let metrics:Option<Metrics> = match unpack_metrics(&definition, &metrics_id_map, buffer, bytes_received){
                    Ok(m) => Some(m),
                    Err(e) => {
                        println!("got error unpacking metrics {}",e );
                        None
                    }
                };

                if metrics.is_some() {
                    println!("{:?}", metrics)
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}
