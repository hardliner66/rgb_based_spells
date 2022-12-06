use kdtree::KdTree;
use std::collections::HashMap;

const WORD_MORE: &str = "fortior"; // intensifier = 1.5
const WORD_LESS: &str = "debilior"; // intensifier = 0.66
const WORD_STRONG: &str = "maior"; // intensifier = 2.0
const WORD_WEAK: &str = "minor"; // intensifier = 0.5
const WORD_SOME_MORE: &str = "aliquantum"; // intensifier = 1.2
const WORD_SOME_LESS: &str = "aliquantulum"; // intensifier = 0.8

const WORD_FIRE: &str = "ignis"; // (255, 0, 0)
const WORD_WATER: &str = "aqua"; // (0, 0, 255)
const WORD_WIND: &str = "ventus"; // (0, 255, 0)
const WORD_EARTH: &str = "terra"; // (128, 128, 128)
const WORD_HEAL: &str = "sanus"; // (128, 255, 0)
const WORD_HARM: &str = "noxius"; // (255, 0, 0)
const WORD_LIGHT: &str = "lux"; // (255, 255, 255)
const WORD_DARK: &str = "tenebrae"; // (0, 0, 0)
const WORD_VOID: &str = "vacuus"; // (0, 0, 0)
const WORD_GOLD: &str = "aurum"; // (255, 215, 0)
const WORD_SILVER: &str = "argentum"; // (192, 192, 192)
const WORD_BRONZE: &str = "aes"; // (205, 127, 50)
const WORD_STEEL: &str = "ferrum"; // (192, 192, 192)
const WORD_COPPER: &str = "aes"; // (255, 215, 0)
const WORD_PLATINUM: &str = "platinum"; // (229, 228, 226)
const WORD_EMERALD: &str = "smaragdus"; // (80, 200, 120)
const WORD_RUBY: &str = "rubinus"; // (255, 0, 0)
const WORD_SAPPHIRE: &str = "sapphirus"; // (0, 0, 240)
const WORD_DIAMOND: &str = "diamas"; // (255, 255, 255)
const WORD_AMETHYST: &str = "amethystus"; // (153, 102, 204)
const WORD_JADE: &str = "nephriticus"; // (0, 168, 107)
const WORD_LIFE: &str = "vita"; // (0, 128, 128)
const WORD_DEATH: &str = "mors"; // (128, 0, 128)
const WORD_SUN: &str = "sol"; // (255, 255, 0)
const WORD_MOON: &str = "luna"; // (128, 128, 128)
const WORD_STAR: &str = "stella"; // (255, 255, 255)
const WORD_SKY: &str = "caelum"; // (135, 206, 235)
const WORD_CLOUD: &str = "nubes"; // (135, 206, 235)
const WORD_RAIN: &str = "pluvia"; // (0, 0, 240)
const WORD_SNOW: &str = "nix"; // (255, 255, 255)
const WORD_ICE: &str = "glacies"; // (128, 128, 128)
const WORD_SAND: &str = "arena"; // (244, 164, 96)
const WORD_GRASS: &str = "herba"; // (0, 255, 0)
const WORD_TREE: &str = "arbor"; // (0, 128, 0)
const WORD_FLOWER: &str = "flos"; // (255, 0, 255)
const WORD_SUNSET: &str = "occasus"; // (255, 165, 0)
const WORD_SUNRISE: &str = "orior"; // (255, 165, 0)
const WORD_SPRING: &str = "ver"; // (0, 255, 0)
const WORD_SUMMER: &str = "aestas"; // (255, 165, 0)
const WORD_FALL: &str = "autumnus"; // (255, 0, 0)
const WORD_WINTER: &str = "hiems"; // (0, 0, 255)
const WORD_BEACH: &str = "litus"; // (255, 255, 0)
const WORD_OCEAN: &str = "oceanus"; // (0, 0, 240)
const WORD_SEA: &str = "mare"; // (0, 0, 240)
const WORD_MOUNTAIN: &str = "mons"; // (128, 128, 128)
const WORD_RIVER: &str = "flumen"; // (0, 0, 240)
const WORD_STREAM: &str = "torrentem"; // (0, 0, 240)
const WORD_LAKE: &str = "lacus"; // (0, 0, 240)
const WORD_VALLEY: &str = "vallis"; // (128, 128, 0)
const WORD_FOREST: &str = "silva"; // (0, 128, 0)
const WORD_JUNGLE: &str = "jungla"; // (0, 128, 0)
const WORD_DESERT: &str = "desertum"; // (255, 165, 0)
const WORD_CAVE: &str = "spelunca"; // (128, 128, 128)
const WORD_MINE: &str = "mina"; // (128, 128, 128)

// Define a Spell struct that contains a name and an RGB value
struct Spell {
    name: String,
    color: (u8, u8, u8),
}

// Define a function that creates a spell using a single color
fn create_spell(name: &str, color: (u8, u8, u8)) -> Spell {
    Spell {
        name: name.to_string(),
        color,
    }
}

// Define a function that calculates the distance between two RGB values
fn color_distance(color1: &[f32], color2: &[f32]) -> f32 {
    // Calculate the difference between each channel and square it
    let diff_r = (color1[0] - color2[0]).powf(2.0);
    let diff_g = (color1[1] - color2[1]).powf(2.0);
    let diff_b = (color1[2] - color2[2]).powf(2.0);

    // Return the square root of the sum of the squared differences
    (diff_r + diff_g + diff_b).sqrt()
}

fn parse_sentence(
    sentence: &str,
    word_map: &HashMap<&str, (u8, u8, u8)>,
    intensifier_map: &HashMap<&str, f32>,
) -> (u8, u8, u8) {
    // Split the sentence into words
    let words = sentence.split(' ');

    // Initialize the red, green, and blue values to 0
    let mut red = 0.0f32;
    let mut green = 0.0f32;
    let mut blue = 0.0f32;

    // Keep track of the intensifier
    let mut intensifier = 1.0;

    // Loop through each word in the sentence
    for word in words {
        if let Some(value) = intensifier_map.get(word) {
            intensifier *= value;
        } else if let Some(&(r, g, b)) = word_map.get(word) {
            red += f32::from(r) * intensifier;
            green += f32::from(g) * intensifier;
            blue += f32::from(b) * intensifier;
            intensifier = 1.0;
        }
    }

    // Scale the RGB values down to fit within the 0-255 range
    let max_value = red.max(green).max(blue);
    red /= max_value;
    green /= max_value;
    blue /= max_value;

    // Convert the RGB values to u8 and return them
    (
        (red * 255.0) as u8,
        (green * 255.0) as u8,
        (blue * 255.0) as u8,
    )
}

fn build_intensifier_map() -> HashMap<&'static str, f32> {
    let mut intensifier_map = HashMap::new();
    intensifier_map.insert(WORD_MORE, 1.5);
    intensifier_map.insert(WORD_LESS, 0.66);
    intensifier_map.insert(WORD_STRONG, 2.0);
    intensifier_map.insert(WORD_WEAK, 0.5);
    intensifier_map.insert(WORD_SOME_MORE, 1.2);
    intensifier_map.insert(WORD_SOME_LESS, 0.8);
    intensifier_map
}

fn build_word_map() -> HashMap<&'static str, (u8, u8, u8)> {
    let mut word_map = HashMap::new();
    word_map.insert(WORD_FIRE, (255, 0, 0));
    word_map.insert(WORD_WATER, (0, 0, 255));
    word_map.insert(WORD_WIND, (0, 255, 0));
    word_map.insert(WORD_EARTH, (128, 128, 128));
    word_map.insert(WORD_HEAL, (128, 255, 0));
    word_map.insert(WORD_HARM, (255, 0, 0));
    word_map.insert(WORD_LIGHT, (255, 255, 255));
    word_map.insert(WORD_DARK, (0, 0, 0));
    word_map.insert(WORD_VOID, (0, 0, 0));
    word_map.insert(WORD_GOLD, (255, 215, 0));
    word_map.insert(WORD_SILVER, (192, 192, 192));
    word_map.insert(WORD_BRONZE, (205, 127, 50));
    word_map.insert(WORD_STEEL, (192, 192, 192));
    word_map.insert(WORD_COPPER, (255, 215, 0));
    word_map.insert(WORD_PLATINUM, (229, 228, 226));
    word_map.insert(WORD_EMERALD, (80, 200, 120));
    word_map.insert(WORD_RUBY, (255, 0, 0));
    word_map.insert(WORD_SAPPHIRE, (0, 0, 240));
    word_map.insert(WORD_DIAMOND, (255, 255, 255));
    word_map.insert(WORD_AMETHYST, (153, 102, 204));
    word_map.insert(WORD_JADE, (0, 168, 107));
    word_map.insert(WORD_LIFE, (0, 128, 128));
    word_map.insert(WORD_DEATH, (128, 0, 128));
    word_map.insert(WORD_SUN, (255, 255, 0));
    word_map.insert(WORD_MOON, (128, 128, 128));
    word_map.insert(WORD_STAR, (255, 255, 255));
    word_map.insert(WORD_SKY, (135, 206, 235));
    word_map.insert(WORD_CLOUD, (135, 206, 235));
    word_map.insert(WORD_RAIN, (0, 0, 240));
    word_map.insert(WORD_SNOW, (255, 255, 255));
    word_map.insert(WORD_ICE, (128, 128, 128));
    word_map.insert(WORD_SAND, (244, 164, 96));
    word_map.insert(WORD_GRASS, (0, 255, 0));
    word_map.insert(WORD_TREE, (0, 128, 0));
    word_map.insert(WORD_FLOWER, (255, 0, 255));
    word_map.insert(WORD_SUNSET, (255, 165, 0));
    word_map.insert(WORD_SUNRISE, (255, 165, 0));
    word_map.insert(WORD_SPRING, (0, 255, 0));
    word_map.insert(WORD_SUMMER, (255, 165, 0));
    word_map.insert(WORD_FALL, (255, 0, 0));
    word_map.insert(WORD_WINTER, (0, 0, 255));
    word_map.insert(WORD_BEACH, (255, 255, 0));
    word_map.insert(WORD_OCEAN, (0, 0, 240));
    word_map.insert(WORD_SEA, (0, 0, 240));
    word_map.insert(WORD_MOUNTAIN, (128, 128, 128));
    word_map.insert(WORD_RIVER, (0, 0, 240));
    word_map.insert(WORD_STREAM, (0, 0, 240));
    word_map.insert(WORD_LAKE, (0, 0, 240));
    word_map.insert(WORD_VALLEY, (128, 128, 0));
    word_map.insert(WORD_FOREST, (0, 128, 0));
    word_map.insert(WORD_JUNGLE, (0, 128, 0));
    word_map.insert(WORD_DESERT, (255, 165, 0));
    word_map.insert(WORD_CAVE, (128, 128, 128));
    word_map.insert(WORD_MINE, (128, 128, 128));
    word_map
}

fn create_spells() -> Vec<Spell> {
    vec![
        create_spell("Fireball", (255, 0, 0)),
        create_spell("Water jet", (0, 0, 255)),
        create_spell("Healing light", (255, 255, 0)),
        create_spell("Levitation", (0, 255, 0)),
        create_spell("Arcane shield", (123, 24, 102)),
        create_spell("Arcane blast", (64, 160, 128)),
        create_spell("Arcane explosion", (64, 255, 128)),
        create_spell("Arcane bolt", (140, 128, 64)),
        create_spell("Sleep spell", (128, 128, 0)),
        create_spell("Lightning bolt", (255, 165, 0)),
        create_spell("Shapeshift", (0, 255, 255)),
        create_spell("Telekinesis", (165, 42, 42)),
        create_spell("Fire blast", (255, 0, 128)),
        create_spell("Frostbite", (128, 0, 255)),
        create_spell("Mind control", (60, 255, 128)),
        create_spell("Earthquake", (128, 255, 128)),
        create_spell("Time warp", (255, 128, 0)),
        create_spell("Dimension door", (128, 255, 255)),
        create_spell("Soul steal", (0, 128, 128)),
        create_spell("Gravity well", (64, 0, 255)),
        create_spell("Firestorm", (255, 64, 0)),
        create_spell("Toxic gas", (64, 255, 0)),
        create_spell("Earth wall", (64, 255, 130)),
        create_spell("Invisibility", (128, 128, 128)),
        create_spell("Force field", (64, 172, 128)),
        create_spell("Acid rain", (128, 255, 180)),
        create_spell("Electric shock", (128, 0, 64)),
        create_spell("Healing touch", (0, 192, 64)),
        create_spell("Chilling fog", (64, 17, 192)),
        create_spell("Stone skin", (192, 255, 64)),
        create_spell("Teleport", (32, 192, 128)),
        create_spell("Blink", (128, 192, 255)),
        create_spell("Magic missile", (255, 128, 192)),
        create_spell("Arcane armor", (255, 182, 128)),
        create_spell("Ice shard", (30, 128, 255)),
        create_spell("Lightning strike", (128, 150, 255)),
        create_spell("Healing wave", (0, 128, 255)),
        create_spell("Healing rain", (128, 128, 255)),
        create_spell("Ice storm", (128, 64, 255)),
        create_spell("Time stop", (128, 255, 64)),
        create_spell("Charm", (125, 64, 192)),
        create_spell("Wind walk", (64, 255, 192)),
        create_spell("Frost nova", (192, 64, 255)),
        create_spell("Ignition", (255, 40, 2)),
        create_spell("Tidal wave", (128, 255, 0)),
        create_spell("Starlight", (0, 255, 128)),
        create_spell("Thunderbolt", (128, 73, 255)),
        create_spell("Displacement", (255, 64, 128)),
        create_spell("Portal", (64, 128, 255)),
        create_spell("Berserker", (255, 192, 64)),
        create_spell("Enrage", (180, 192, 64)),
        create_spell("Soul bind", (64, 192, 255)),
        create_spell("Enchantment", (255, 64, 192)),
        create_spell("Chilling mist", (192, 20, 64)),
        create_spell("Fire shield", (255, 128, 64)),
        create_spell("Flame shield", (255, 192, 128)),
    ]
}

fn main() {
    let intensifier_map = build_intensifier_map();
    let word_map = build_word_map();
    let spells = create_spells();

    // Create a k-d tree to store the predefined spells
    let mut tree = KdTree::new(3);

    for spell in spells {
        tree.add(
            [
                f32::from(spell.color.0),
                f32::from(spell.color.1),
                f32::from(spell.color.2),
            ],
            spell.name,
        )
        .unwrap();
    }

    // Example usage:
    // cargo run -- maior ignis minor aqua aliquantulum fortior terra
    let sentence = std::env::args().collect::<Vec<_>>().join(" ");
    let (red, green, blue) = parse_sentence(&sentence, &word_map, &intensifier_map);
    println!("RGB: ({}, {}, {})", red, green, blue);

    let rgb = [f32::from(red), f32::from(green), f32::from(blue)];

    // Find the nearest predefined spells to the mixed spells
    let mut nearest_spell = tree.iter_nearest(&rgb, &color_distance).unwrap();

    println!(
        "Nearest 3 Spells: {}, {}, {}",
        nearest_spell.next().unwrap().1,
        nearest_spell.next().unwrap().1,
        nearest_spell.next().unwrap().1
    );
}
