use clap::Parser;
use drdie::{roll_dice, RollOptions};

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

    /// Drop highest N dice (disadvantage)
    #[arg(long)]
    drop: Option<u32>,

    /// Success threshold - count dice meeting or exceeding this value
    #[arg(long)]
    success: Option<u32>,

    /// Crit threshold (defaults to max die value if flag present without value)
    #[arg(long)]
    crit: Option<u32>,

    /// Output in JSON format
    #[arg(long)]
    json: bool,

    /// Verbose output with all details
    #[arg(long, short)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let options = RollOptions {
        explode: args.explode,
        keep: args.keep,
        drop: args.drop,
        success: args.success,
        crit: args.crit,
    };

    match roll_dice(&args.dice, &options) {
        Ok(result) => {
            if args.json {
                // JSON output
                match serde_json::to_string_pretty(&result) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        eprintln!("Error serializing to JSON: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if args.verbose {
                // Verbose output
                println!("Dice: {}", result.notation);
                println!("All rolls: {:?}", result.rolls);
                if result.rolls != result.kept_rolls {
                    println!("Kept rolls: {:?}", result.kept_rolls);
                }
                println!("Total: {}", result.total);
                if result.successes > 0 || options.success.is_some() || options.crit.is_some() {
                    println!("Successes: {}", result.successes);
                    println!("Crits: {}", result.crits);
                }
            } else {
                // Simple output
                if result.successes > 0 || options.success.is_some() || options.crit.is_some() {
                    // If counting successes/crits, show those
                    if result.crits > 0 {
                        println!("{} ({} crits)", result.successes, result.crits);
                    } else {
                        println!("{}", result.successes);
                    }
                } else {
                    // Otherwise show total
                    println!("{}", result.total);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
