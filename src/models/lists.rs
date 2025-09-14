use crate::models::types::Item;
use lazy_static::lazy_static;

pub const MOONS: &[&str] = &[
    "Experimentation",
    "Assurance",
    "Vow",
    "Offense",
    "March",
    "Adamance",
    "Rend",
    "Dine",
    "Titan",
    "Artifice",
    "Embrion",
    "Liquidation",
    "Company",
];

lazy_static! {
    pub static ref STORE_ITEMS: Vec<Item> = vec![
        Item {
            name: String::from("Shovel"),
            price: 30,
            weight: 8.0,
            description: String::from("A standard-issue shovel. Digs things."),
        },
        Item {
            name: String::from("Spray Paint"),
            price: 50,
            weight: 0.0,
            description: String::from("For marking paths or walls."),
        },
        Item {
            name: String::from("Stun Grenade"),
            price: 40,
            weight: 5.0,
            description: String::from("A non-lethal grenade to stun creatures."),
        },
        Item {
            name: String::from("TZP-Inhalant"),
            price: 120,
            weight: 0.0,
            description: String::from(
                "Makes you move faster and use less stamina + Squeaky voice."
            ),
        },
        Item {
            name: String::from("Walkie-Talkie"),
            price: 12,
            weight: 0.0,
            description: String::from(
                "Allows Map Wide Communication (Uses Battery Charge) (Has Large Charge)."
            ),
        },
        Item {
            name: String::from("Zap Gun"),
            price: 400,
            weight: 11.0,
            description: String::from("Scans for Monsters and Stuns them (uses battery charge)."),
        },
        Item {
            name: String::from("Weed Killer"),
            price: 25,
            weight: 0.0,
            description: String::from("Sprays a non-lethal poison. (Counts as a sound source)"),
        },
        Item {
            name: String::from("Company Cruiser"),
            price: 400,
            weight: 0.0,
            description: String::from(
                "Can be bought for free one time if destroyed or lost. Cannot be carried."
            ),
        },
        Item {
            name: String::from("Belt Bag"),
            price: 45,
            weight: 15.5,
            description: String::from(
                "A small bag that can be carried in a single hand. Holds 4 items."
            ),
        },
        Item {
            name: String::from("Survival Kit"),
            price: 138,
            weight: 0.0,
            description: String::from(
                "A kit containing 4 Flashlights, 4 Walkie-talkies, and a Shovel."
            ),
        },
        Item {
            name: String::from("Flashlight"),
            price: 15,
            weight: 5.0,
            description: String::from(
                "A small portable light source. (Uses battery charge) (Has Large Charge)"
            ),
        },
        Item {
            name: String::from("Pro-flashlight"),
            price: 25,
            weight: 5.0,
            description: String::from(
                "A powerful portable light source. (Uses battery charge) (Has Large Charge)"
            ),
        },
    ];
}
pub const SHIP_UPGRADE: &[&str] = &[
    "Teleporter",
    "Inverse Teleporter",
    "Loud Horn",
    "Signal Translator",
];
pub const SHIP_DECORATIONS: &[&str] = &["Cozy Lights", "Decoy Suit", "Brown Suit", "Purple Suit"];
pub const BESTIARY: &[(&str, &str)] = &[
    (
        "Barber",
        "A humanoid clay creature carrying scissors; invisible at range, slashes players when close.",
    ),
    (
        "Bracken",
        "Shadowy predator that stalks silently; flees when watched, but enrages under prolonged gaze.",
    ),
    (
        "Bunker Spider",
        "Large territorial spider; hides in webs and attacks aggressively if disturbed.",
    ),
    (
        "Butler",
        "Blobfish-like janitor; sweeps harmlessly until it stabs isolated players, then bursts into hornets on death.",
    ),
    (
        "Coil-Head",
        "Spring-neck mannequin that moves only when not observed; impossible to kill permanently.",
    ),
    (
        "Ghost Girl",
        "Spectral girl visible only to her target; suddenly appears and instantly kills them.",
    ),
    (
        "Hoarding Bug",
        "Insect that gathers scrap into nests; mostly harmless unless provoked.",
    ),
    (
        "Hygrodere",
        "Slow amorphous slime; kills only by engulfment, but moves very slowly.",
    ),
    (
        "Jester",
        "Jack-in-the-box entity; after winding up, pops open into a deadly monster.",
    ),
    (
        "Maneater",
        "Sentient larva; harmless when cared for, lethal if allowed to mature.",
    ),
    (
        "Masked",
        "Possessive mask that turns victims into Masked entities; hunts and converts players.",
    ),
    (
        "Nutcracker",
        "Animated wooden soldier with shotgun; shoots players on sight, weak when eye is exposed.",
    ),
    (
        "Snare Flea",
        "Ceiling-dwelling insect; drops onto players’ heads to suffocate them.",
    ),
    (
        "Spore Lizard",
        "Small reptile; timid and releases harmless spores when threatened.",
    ),
    (
        "Thumper",
        "Shark-like biped; deaf but fast, sprints and bites rapidly to kill.",
    ),
    (
        "Baboon Hawk",
        "Primate-bird hybrid in flocks; harasses players, steals scrap, fights other creatures.",
    ),
    (
        "Earth Leviathan",
        "Colossal sandworm; erupts from ground after warning, instantly crushing targets.",
    ),
    (
        "Eyeless Dog",
        "Blind canine hunter; tracks by sound, deadly in packs.",
    ),
    (
        "Forest Keeper",
        "Giant satyr-like predator; fast and lethal outdoors, grabs and eats victims.",
    ),
    (
        "Old Bird",
        "Massive mech-like humanoid; flies and attacks with missiles and melee strikes.",
    ),
    (
        "Circuit Bee",
        "Aggressive bees defending ground hives; swarm with electric shocks.",
    ),
    (
        "Manticoil",
        "Four-winged bird; completely harmless, flees when approached.",
    ),
    (
        "Roaming Locust",
        "Insect swarm; harmless, scatters if disturbed, attracted to light.",
    ),
    (
        "Tulip Snake",
        "Snake that latches onto players’ heads; can lift them briefly before detaching.",
    ),
    (
        "Giant Sapsucker",
        "Huge woodpecker-like bird; defends nest violently, chases intruders long distances.",
    ),
];
