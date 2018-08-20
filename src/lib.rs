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
#[derive(Serialize, Deserialize)]
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
#[derive(Serialize, Deserialize)]
pub enum Face<'a> {
    /// Blank face
    Blank,
    /// 1 Hammer face
    Hammer,
    /// 1 Hammer+ face
    HammerP,
    /// 2 Hammers face
    HammerD,
    /// 1 Hammer / Wait face
    HammerW,
    /// 1 Hammer / Eagle face
    HammerE,
    /// 1 Hammer / Tear face
    HammerT,
    /// 1 Eagle face
    Eagle,
    /// 2 Eagles face
    EagleD,
    /// 1 Skull face
    Skull,
    /// 2 Skull face
    SkullD,
    /// 1 Blade face
    Blade,
    /// 2 Blade face
    BladeD,
    /// Chaos face
    Chaos,
    /// Comet face
    Comet,
}

impl<'a> fmt::Debug for Face<'a> {
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

const FORTUNE_DICE: [Face; 6] = [Face::Blank, Face::Hammer, Face::Blank, Face::Hammer, Face::Blank, Face::Eagle];

const MISFORTUNE_DICE: [Face; 6] = [Face::Blade, Face::Blank, Face::Skull, Face::Blank, Face::Blade, Face::Blank];

const EXPERTISE_DICE: [Face; 6] = [Face::HammerP, Face::Eagle, Face::Blank, Face::Hammer, Face::Comet, Face::Eagle];

const CHALLENGE_DICE: [Face; 8] = [
    Face::Blade,
    Face::Skull,
    Face::BladeD,
    Face::Blade,
    Face::Chaos,
    Face::Blank,
    Face::SkullD,
    Face::BladeD,
];

const CHARACTERISTIC_DICE: [Face; 8] = [
    Face::Eagle,
    Face::Hammer,
    Face::Blank,
    Face::Hammer,
    Face::Hammer,
    Face::Eagle,
    Face::Blank,
    Face::Hammer,
];

const CONSERVATIVE_DICE: [Face; 10] = [
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

const RECKLESS_DICE: [Face; 10] = [
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

// Represent a roll result
#[derive(Serialize, Deserialize)]
struct DiceResult<'a> {
    dice: Dice,
    face: &'a Face,
}

impl<'a> fmt::Debug for DiceResult<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ dice: {:?}, face: {:?} }}", &self.dice, &self.face)
    }
}

fn roll<'a>(dice: [Face; 6]) -> &'a Face {
    let mut rng = thread_rng();

    return match rng.choose(&dice) {
        Some(face) => face,
        None => {
            println!("roll_dice: some found return blank");
            return &Face::Blank;
        }
    };
}

fn roll_dice<'a>(dice: Dice) -> DiceResult<'a> {
    match dice {
        Dice::Fortune => DiceResult {
            dice: Dice::Fortune,
            face: roll(FORTUNE_DICE),
        },
        Dice::Misfortune => DiceResult {
            dice: Dice::Misfortune,
            face: roll(MISFORTUNE_DICE),
        },
        Dice::Expertise => DiceResult {
            dice: Dice::Expertise,
            face: roll(EXPERTISE_DICE),
        },
        Dice::Characteristic => DiceResult {
            dice: Dice::Characteristic,
            face: roll(CHARACTERISTIC_DICE),
        },
        Dice::Challenge => DiceResult {
            dice: Dice::Challenge,
            face: roll(CHALLENGE_DICE),
        },
        Dice::Conservative => DiceResult {
            dice: Dice::Conservative,
            face: roll(CONSERVATIVE_DICE),
        },
        Dice::Reckless => DiceResult {
            dice: Dice::Reckless,
            face: roll(RECKLESS_DICE),
        },
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
pub fn roll_dices<'a>(dices: Vec<Dice>) -> Vec<DiceResult<'a>> {
    return dices.into_iter().map(roll_dice).collect();
}
