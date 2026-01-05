use rand::Rng;
use regex::Regex;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiceRoll {
    pub num_dice: u32,
    pub num_sides: u32,
    pub notation: String,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct RollOptions {
    pub explode: bool,
    pub keep: Option<u32>,
    pub drop: Option<u32>,
    pub success: Option<u32>,
    pub crit: Option<u32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RollResult {
    pub rolls: Vec<u32>,
    pub kept_rolls: Vec<u32>,
    pub total: u32,
    pub successes: u32,
    pub crits: u32,
    pub notation: String,
}

#[derive(Debug)]
pub enum DiceError {
    InvalidNotation(String),
    InvalidOptions(String),
}

impl std::fmt::Display for DiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceError::InvalidNotation(msg) => write!(f, "Invalid dice notation: {}", msg),
            DiceError::InvalidOptions(msg) => write!(f, "Invalid options: {}", msg),
        }
    }
}

impl std::error::Error for DiceError {}

/// Roll dice with the given notation and options
pub fn roll_dice(notation: &str, options: &RollOptions) -> Result<RollResult, DiceError> {
    let dice_roll = parse_dice_notation(notation)?;

    // Validate options
    if options.keep.is_some() && options.drop.is_some() {
        return Err(DiceError::InvalidOptions(
            "Cannot use both --keep and --drop".to_string(),
        ));
    }

    let mut rng = rand::rng();
    let mut rolls = Vec::new();

    // Roll initial dice
    for _ in 0..dice_roll.num_dice {
        let roll = rng.random_range(1..=dice_roll.num_sides);
        rolls.push(roll);

        // Handle exploding dice
        if options.explode && roll == dice_roll.num_sides {
            let mut explosion_roll = roll;
            while explosion_roll == dice_roll.num_sides {
                explosion_roll = rng.random_range(1..=dice_roll.num_sides);
                rolls.push(explosion_roll);
            }
        }
    }

    // Apply keep/drop
    let mut kept_rolls = rolls.clone();
    if let Some(keep_count) = options.keep {
        kept_rolls.sort_by(|a, b| b.cmp(a)); // Sort descending
        kept_rolls.truncate(keep_count as usize);
    } else if let Some(drop_count) = options.drop {
        kept_rolls.sort_by(|a, b| b.cmp(a)); // Sort descending
        let drop_idx = drop_count as usize;
        if drop_idx < kept_rolls.len() {
            kept_rolls = kept_rolls[drop_idx..].to_vec();
        } else {
            kept_rolls.clear();
        }
    }

    // Calculate total
    let total: u32 = kept_rolls.iter().sum();

    // Determine success and crit thresholds
    let (success_threshold, crit_threshold) = match (options.success, options.crit) {
        (Some(s), Some(c)) => (s, c),
        (Some(s), None) => (s, dice_roll.num_sides + 1), // No crits if not specified
        (None, Some(c)) => (c, c), // Crit implies success when no success specified
        (None, None) => (dice_roll.num_sides + 1, dice_roll.num_sides + 1), // No success/crit counting
    };

    // Count successes and crits
    let mut successes = 0;
    let mut crits = 0;

    if options.success.is_some() || options.crit.is_some() {
        for &roll in &kept_rolls {
            if roll >= success_threshold {
                successes += 1;
                if roll >= crit_threshold {
                    crits += 1;
                }
            }
        }
    }

    Ok(RollResult {
        rolls,
        kept_rolls,
        total,
        successes,
        crits,
        notation: dice_roll.notation,
    })
}

/// Parse dice notation like "5d6", "20" (for 1d20), or defaults to "1d6"
pub fn parse_dice_notation(notation: &str) -> Result<DiceRoll, DiceError> {
    let full_pattern = Regex::new(r"^(\d+)d(\d+)$").unwrap();
    let shorthand_pattern = Regex::new(r"^(\d+)$").unwrap();

    let (num_dice, num_sides) = if let Some(caps) = full_pattern.captures(notation) {
        (
            caps[1].parse::<u32>().unwrap(),
            caps[2].parse::<u32>().unwrap(),
        )
    } else if let Some(caps) = shorthand_pattern.captures(notation) {
        (1, caps[1].parse::<u32>().unwrap())
    } else {
        return Err(DiceError::InvalidNotation(
            "Use format like '5d6' or just '20' for 1d20".to_string(),
        ));
    };

    Ok(DiceRoll {
        num_dice,
        num_sides,
        notation: notation.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_notation() {
        let roll = parse_dice_notation("3d6").unwrap();
        assert_eq!(roll.num_dice, 3);
        assert_eq!(roll.num_sides, 6);
    }

    #[test]
    fn test_shorthand_notation() {
        let roll = parse_dice_notation("20").unwrap();
        assert_eq!(roll.num_dice, 1);
        assert_eq!(roll.num_sides, 20);
    }

    #[test]
    fn test_invalid_notation() {
        assert!(parse_dice_notation("invalid").is_err());
    }

    #[test]
    fn test_roll_dice_basic() {
        let result = roll_dice("3d6", &RollOptions::default()).unwrap();
        assert_eq!(result.rolls.len(), 3);
        assert_eq!(result.kept_rolls.len(), 3);
        assert!(result.total >= 3 && result.total <= 18);
    }

    #[test]
    fn test_roll_dice_keep() {
        let options = RollOptions {
            keep: Some(2),
            ..Default::default()
        };
        let result = roll_dice("4d6", &options).unwrap();
        assert_eq!(result.rolls.len(), 4);
        assert_eq!(result.kept_rolls.len(), 2);
    }

    #[test]
    fn test_roll_dice_drop() {
        let options = RollOptions {
            drop: Some(1),
            ..Default::default()
        };
        let result = roll_dice("2d20", &options).unwrap();
        assert_eq!(result.rolls.len(), 2);
        assert_eq!(result.kept_rolls.len(), 1);
    }

    #[test]
    fn test_cannot_use_keep_and_drop() {
        let options = RollOptions {
            keep: Some(1),
            drop: Some(1),
            ..Default::default()
        };
        assert!(roll_dice("2d20", &options).is_err());
    }

    #[test]
    fn test_success_counting() {
        let options = RollOptions {
            success: Some(1), // All dice are successes
            ..Default::default()
        };
        let result = roll_dice("3d6", &options).unwrap();
        assert_eq!(result.successes as usize, result.kept_rolls.len());
    }

    #[test]
    fn test_crit_implies_success() {
        let options = RollOptions {
            crit: Some(100), // Nothing can crit
            ..Default::default()
        };
        let result = roll_dice("3d6", &options).unwrap();
        assert_eq!(result.crits, 0);
        assert_eq!(result.successes, 0);
    }
}
