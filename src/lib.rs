extern crate rand;

use rand::{thread_rng, Rng};
use std::fmt;

enum Face {
    Blank,   // Blank face
    Hammer,  // 1 Hammer face
    HammerP, // 1 Hammer+ face
    HammerD, // 2 Hammers face
    HammerW, // 1 Hammer / Wait face
    HammerE, // 1 Hammer / Eagle face
    HammerT, // 1 Hammer / Tear face
    Eagle,   // 1 Eagle face
    EagleD,  // 2 Eagles face
    Skull,   // 1 Skull face
    SkullD,  // 2 Skull face
    Blade,   // 1 Blade face
    BladeD,  // 2 Blade face
    Chaos,   // Chaos face
    Comet,   // Comet face
}

impl fmt::Debug for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Face::Blank => write!(f, "Blank"),
            Face::Hammer => write!(f, "Hammer"),
            Face::HammerP => write!(f, "Hammer/+"),
            Face::HammerD => write!(f, "Hammer/Hammer"),
            Face::HammerW => write!(f, "Hammer/Wait"),
            Face::HammerE => write!(f, "Hammer/Eagle"),
            Face::HammerT => write!(f, "Hammer/Tear"),
            Face::Eagle => write!(f, "Eagle"),
            Face::EagleD => write!(f, "Eagle/Eagle"),
            Face::Skull => write!(f, "Skull"),
            Face::SkullD => write!(f, "Skull/Skull"),
            Face::Blade => write!(f, "Blade"),
            Face::BladeD => write!(f, "Blade/Blade"),
            Face::Chaos => write!(f, "Chaos"),
            Face::Comet => write!(f, "Comet"),
        }
    }
}

static FORTUNE_DICE: &'static [Face] = &[
    Face::Blank,
    Face::Hammer,
    Face::Blank,
    Face::Hammer,
    Face::Blank,
    Face::Eagle,
];

static MISFORTUNE_DICE: &'static [Face] = &[
    Face::Blade,
    Face::Blank,
    Face::Skull,
    Face::Blank,
    Face::Blade,
    Face::Blank,
];

static EXPERTISE_DICE: &'static [Face] = &[
    Face::HammerP,
    Face::Eagle,
    Face::Blank,
    Face::Hammer,
    Face::Comet,
    Face::Eagle,
];

static CHALLENGE_DICE: &'static [Face] = &[
    Face::Blade,
    Face::Skull,
    Face::BladeD,
    Face::Blade,
    Face::Chaos,
    Face::Blank,
    Face::SkullD,
    Face::BladeD,
];

static CHARACTERISTIC_DICE: &'static [Face] = &[
    Face::Eagle,
    Face::Hammer,
    Face::Blank,
    Face::Hammer,
    Face::Hammer,
    Face::Eagle,
    Face::Blank,
    Face::Hammer,
];

static CONSERVATIVE_DICE: &'static [Face] = &[
    Face::Hammer,
    Face::Eagle,
    Face::Hammer,
    Face::HammerW,
    Face::Hammer,
    Face::Blank,
    Face::HammerE,
    Face::Hammer,
    Face::Eagle,
    Face::HammerW,
];

static RECKLESS_DICE: &'static [Face] = &[
    Face::HammerE,
    Face::HammerD,
    Face::Blank,
    Face::Skull,
    Face::EagleD,
    Face::HammerT,
    Face::Blank,
    Face::HammerT,
    Face::Skull,
    Face::HammerD,
];

pub fn roll_fortune() {
    let mut rng = thread_rng();
    println!("{:?}", rng.choose(&FORTUNE_DICE));
}
