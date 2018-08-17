extern crate wardice;

use wardice::{roll_dices, Dice};

fn main() {
    println!("Hello, wardice!");
    let dices = vec![Dice::Characteristic, Dice::Characteristic, Dice::Fortune, Dice::Challenge];
    let f = roll_dices(dices);
    println!("{:?}", f);
}
