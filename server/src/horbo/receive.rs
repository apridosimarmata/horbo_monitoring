use std::{collections::HashMap, fmt::Error, io::ErrorKind};

use super::horbo::{Metric, Metrics, ProtocolDefinition};


pub fn unpack_metrics <'a>(
    def: &ProtocolDefinition,
    metrics_id_map: &HashMap<usize, String>,
    data: [u8; 1024], len:usize) -> Result<Metrics, ErrorKind>{
    /* ensuring can check start flag, version, & length */
    if data.len() < 4{
        return Err(ErrorKind::InvalidData);
    }

    /* check start flag */
    if data[0] != 255 {
        return Err(ErrorKind::InvalidData);

    }

    /* check protocol version */
    if data[1] != def.protocol_version {
        return Err(ErrorKind::InvalidData);

    }

    let length_bytes: &[u8] = &data[2..4];
    let length = ((length_bytes[0] as u16) << 8) | (length_bytes[1] as u16);
    

    /* compare actual length vs length in header */
    if length as usize != len{
        return Err(ErrorKind::InvalidData);
    }

    let body_bytes: &[u8] = &data[4..length as usize];

    let mut metrics: Metrics =  Metrics {
        items: HashMap::new()
    };

    let mut pos: usize = 0;
    /* unpack metrics one by one */
    loop {
        if pos+4 >= len as usize{
            break;
        }

        // get id
        let id = body_bytes[pos] as usize;

        let metric_key = metrics_id_map.get(&id)
        .expect(format!("unknown metric {}", id).as_str());

        // get length, drop if not 4 bytes
        pos += 1;
        if body_bytes[pos] as usize != 4 {
            pos+= body_bytes[pos] as usize;
            continue;
        }

        pos+=1;

        // extract usage
        let usage_byte: [u8; 4] = body_bytes[pos..pos+4].try_into().unwrap(); 
        let usage = f32::from_be_bytes(usage_byte);

        metrics.items.insert(metric_key.clone(),Metric{
            usage: usage,
        });

        pos+=4;
    }

    return Ok(metrics)

}