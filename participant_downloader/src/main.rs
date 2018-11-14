extern crate pbp_shared;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Write;
use pbp_shared::participant::{Participant, download_participants};

fn main() {
    let list = download_participants().expect("Failed to download the participant list.");
    let j = serde_json::to_string(&list).expect("Failed to serialize to JSON.");
    let mut file = File::create("../pbp_participants.json").expect("Failed to create file.");
    file.write_all(&j.as_bytes()).expect("Failed to write JSON to file.");

    let deser: Vec<Participant> = serde_json::from_str(&j).expect("Failed to deserialize JSON.");

    for p in deser {
        println!("Participant: {:?}", p);
    }
}

