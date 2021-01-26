mod etcs5;

use serde_json::Value;

//decoder function type definition
type Decoder = fn(String) -> String;

//packet error definition
enum PacketError {
    UndefinedNid(usize),
}

//cut the whole package binary string into several slices
fn etcs_package_slice(bin_str: String) -> Result<Vec<String>, String> {
    let mut ptr = 0; //pointer pointing at the first bit of current packet
    let mut result: Vec<String> = vec![];
    //while the pointer is less than length of binary string
    while ptr < bin_str.len() {
        //length of the current packet
        let len = usize::from_str_radix(&bin_str[ptr + 10..ptr + 23], 2).unwrap();
        result.push(String::from(&bin_str[ptr..ptr + len]));
        ptr = ptr + len; //update pointer
    }
    Ok(result)
}

//convert telegram binary string into Packet Object
fn from_bin_str(bin_str: String) -> Value {
    unimplemented!()
}

//convert telegram json string into Packet Object
fn from_json_str(json_str: String) -> Value {
    match serde_json::from_str(&json_str) {
        Ok(value) => value,
        Err(err) => panic!(err)
    }
}

//encode Packet Object to binary string
fn to_bin_str(packet: Value) -> String {
    let nid = packet["nid_packet"].as_u64();
    match nid {
        //Etcs Packet 5
        Some(5) => {
            String::from("str")
        }
        _ => String::from("str")
    }
}

//decode Packet Object to binary json string
fn to_json_str(packet: Value) -> String {
    unimplemented!()
}

//convert json string to binary string
fn encode(json_str: String) -> String {
    let obj = from_json_str(json_str);
    to_bin_str(obj)
}

//convert binary string to json string
fn decode(bin_str: String) -> String {
    let obj = from_bin_str(bin_str);
    to_json_str(obj)
}



