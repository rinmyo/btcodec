use serde_json::{Value, Map};
use serde_json::json;
use crate::tool::{bin_str2int, get_range};
use serde_json::value::Value::{Number, Object};

fn from_bin_str(bin_str: &str) -> Value {
    let range = get_range(0);

    let bin2int = |step|{
        bin_str2int(bin_str, range(step))
    };

    let mut packet_map = Map::new();

    //information packet identification= 0000 0101
    packet_map.insert(String::from("NID_PACKET"), json!(bin2int(8)));
    //verification direction
    packet_map.insert(String::from("Q_DIR"), json!(bin2int(2)));
    //length of information packet
    packet_map.insert(String::from("L_PACKET"), json!(bin2int(13)));
    //scale of distance/length
    packet_map.insert(String::from("Q_SCALE"), json!(bin2int(2)));
    //FUCK NVIDIA!
    packet_map.insert(String::from("D_LINK"), json!(bin2int(15)));
    //0: same 1: diff
    packet_map.insert(String::from("Q_NEWCOUNTRY"), json!(bin2int(1)));
    //if Q_NEWCOUNTRY == 1
    if assert_eq!(packet_map.get("Q_NEWCOUNTRY"), json!(1)) {
        packet_map.insert(String::from("NID_C"), json!(bit2int(10)));
    } else {
        packet_map.insert(String::from("NID_BG"), json!(bin2int(14)));
    }

    Object(packet_map)
}