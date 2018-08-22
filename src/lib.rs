#![deny(
    missing_docs, missing_debug_implementations, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications
)]

//! Wardice crate

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;

use rand::{thread_rng, Rng};
use std::fmt;

/// Represent all dice type available
#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Dice {
    /// Represent a Fortune dice
    Fortune,
    /// Represent a Misfortune dice
    Misfortune,
    /// Represent a Expertise dice
    Expertise,
    /// Represent a Characteristic dice
    Characteristic,
    /// Represent a Challenge dice
    Challenge,
    /// Represent a Conservative dice
    Conservative,
    /// Represent a Reckless dice
    Reckless,
}

impl fmt::Debug for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Dice::Fortune => write!(f, "Fortune"),
            Dice::Misfortune => write!(f, "Misfortune"),
            Dice::Expertise => write!(f, "Expertise"),
            Dice::Characteristic => write!(f, "Characteristic"),
            Dice::Challenge => write!(f, "Challenge"),
            Dice::Conservative => write!(f, "Conservative"),
            Dice::Reckless => write!(f, "Reckless"),
        }
    }
}

/// Represent all dice faces
#[derive(Clone, Copy)]
pub enum Face {
    /// Blank face => ""
    Blank,
    /// 1 Hammer face => "!"
    Hammer,
    /// 1 Hammer+ face => "#"
    HammerP,
    /// 2 Hammers face => "!!"
    HammerD,
    /// 1 Hammer / Wait face => "!("
    HammerW,
    /// 1 Hammer / Eagle face => "!$"
    HammerE,
    /// 1 Hammer / Tear face => "!*"
    HammerT,
    /// 1 Eagle face => "$"
    Eagle,
    /// 2 Eagles face => "$$"
    EagleD,
    /// 1 Skull face => "%"
    Skull,
    /// 2 Skull face => "%%"
    SkullD,
    /// 1 Blade face => "@"
    Blade,
    /// 2 Blade face => "@@"
    BladeD,
    /// Chaos face => "&"
    Chaos,
    /// Comet face => "^"
    Comet,
}
impl serde::Serialize for Face {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Face::Blank => serializer.serialize_str(""),
            Face::Hammer => serializer.serialize_str("!"),
            Face::HammerP => serializer.serialize_str("#"),
            Face::HammerD => serializer.serialize_str("!!"),
            Face::HammerW => serializer.serialize_str("!("),
            Face::HammerE => serializer.serialize_str("!$"),
            Face::HammerT => serializer.serialize_str("!*"),
            Face::Eagle => serializer.serialize_str("$"),
            Face::EagleD => serializer.serialize_str("$$"),
            Face::Skull => serializer.serialize_str("%"),
            Face::SkullD => serializer.serialize_str("%%"),
            Face::Blade => serializer.serialize_str("@"),
            Face::BladeD => serializer.serialize_str("@@"),
            Face::Chaos => serializer.serialize_str("&"),
            Face::Comet => serializer.serialize_str("^"),
        }
    }
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

static FORTUNE_DICE: &'static [Face] = &[Face::Blank, Face::Hammer, Face::Blank, Face::Hammer, Face::Blank, Face::Eagle];

static MISFORTUNE_DICE: &'static [Face] = &[Face::Blade, Face::Blank, Face::Skull, Face::Blank, Face::Blade, Face::Blank];

static EXPERTISE_DICE: &'static [Face] = &[Face::HammerP, Face::Eagle, Face::Blank, Face::Hammer, Face::Comet, Face::Eagle];

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

fn roll(dice: &[Face]) -> &Face {
    let mut rng = thread_rng();

    return match rng.choose(dice) {
        Some(face) => face,
        None => {
            println!("roll_dice: some found return blank");
            return &Face::Blank;
        }
    };
}

fn roll_dice(dice: Dice) -> (Dice, &'static Face) {
    match dice {
        Dice::Fortune => (Dice::Fortune, roll(FORTUNE_DICE)),
        Dice::Misfortune => (Dice::Misfortune, roll(MISFORTUNE_DICE)),
        Dice::Expertise => (Dice::Expertise, roll(EXPERTISE_DICE)),
        Dice::Characteristic => (Dice::Characteristic, roll(CHARACTERISTIC_DICE)),
        Dice::Challenge => (Dice::Challenge, roll(CHALLENGE_DICE)),
        Dice::Conservative => (Dice::Conservative, roll(CONSERVATIVE_DICE)),
        Dice::Reckless => (Dice::Reckless, roll(RECKLESS_DICE)),
    }
}

fn reroll_dice(l: &mut Vec<(Dice, &'static Face)>, roll: (Dice, &'static Face)) {
    match roll.1 {
        Face::HammerP => {
            let r = roll_dice(roll.0);
            l.push(r);
            reroll_dice(l, r);
        }
        _ => (),
    }
}

/// Roll dices.
///
/// # Examples
///
/// ```
/// let dices = [Dice::Characteristic,Dice::Characteristic,Dice::Fortune,Dice::Challenge]
/// let resuls = roll_dices(dices);
///
/// ```
pub fn roll_dices(dices: Vec<Dice>) -> Vec<(Dice, &'static Face)> {
    let mut rolls: Vec<(Dice, &'static Face)> = dices.into_iter().map(roll_dice).collect();

    let mut rerolls: Vec<(Dice, &'static Face)> = Vec::new();
    rolls.clone().into_iter().map(|r| reroll_dice(&mut rerolls, r)).collect::<()>();

    rolls.append(&mut rerolls);
    rolls
}
