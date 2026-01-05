use clap::Parser;
use drdie::parse_dice_notation;

#[derive(Parser)]
#[command(name = "drdie")]
#[command(about = "A dice rolling CLI", long_about = None)]
struct Args {
    /// Dice notation (e.g., "5d6" or "20" for 1d20)
    #[arg(default_value = "1d6")]
    dice: String,

    /// Exploding dice - reroll maximum values
    #[arg(long)]
    explode: bool,

    /// Keep highest N dice
    #[arg(long)]
    keep: Option<u32>,

    /// Success threshold - count dice meeting or exceeding this value
    #[arg(long)]
    success: Option<u32>,
}

fn main() {
    let args = Args::parse();

    match parse_dice_notation(&args.dice) {
        Ok(roll) => {
            println!("Rolling {} dice with {} sides", roll.num_dice, roll.num_sides);
            
            if args.explode {
                println!("  (with exploding dice)");
            }
            if let Some(keep) = args.keep {
                println!("  (keeping highest {})", keep);
            }
            if let Some(threshold) = args.success {
                println!("  (counting successes >= {})", threshold);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
