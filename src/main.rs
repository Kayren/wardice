extern crate wardice;

use wardice::roll;

fn main() {
    println!("Hello, wardice!");
    let f = roll(wardice::FORTUNE_DICE);
    println!("{:?}", f);
}
