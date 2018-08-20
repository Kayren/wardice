extern crate serde_json;
extern crate wardice;

use wardice::{roll_dices, Dice};

fn main() {
    println!("Hello, wardice!");
    let dices = vec![Dice::Characteristic, Dice::Characteristic, Dice::Expertise, Dice::Fortune, Dice::Challenge];
    let f = roll_dices(dices);
    println!("{:?}", f);
    let serialized = serde_json::to_string(&f).unwrap();
    println!("json: {:?}", serialized);
}
