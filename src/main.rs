use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dies = if args.len() < 2 {
        "1d6".to_string()
    } else {
        args[1].clone()
    };

    let full_pattern = Regex::new(r"^(\d+)d(\d+)$").unwrap();
    let shorthand_pattern = Regex::new(r"^(\d+)$").unwrap();
    
    let (num_dice, num_sides) = if let Some(caps) = full_pattern.captures(&dies) {
        (caps[1].parse::<u32>().unwrap(), caps[2].parse::<u32>().unwrap())
    } else if let Some(caps) = shorthand_pattern.captures(&dies) {
        (1, caps[1].parse::<u32>().unwrap())
    } else {
        eprintln!("Invalid dice notation. Use format like '5d6' or just '20' for 1d20");
        std::process::exit(1);
    };
    
    println!("Rolling {} dice with {} sides", num_dice, num_sides);
}
