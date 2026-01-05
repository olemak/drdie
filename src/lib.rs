use regex::Regex;

#[cfg(feature = "wasm-bindgen")]
pub mod wasm;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DiceRoll {
    pub num_dice: u32,
    pub num_sides: u32,
    pub notation: String,
}

#[derive(Debug)]
pub enum DiceError {
    InvalidNotation(String),
}

impl std::fmt::Display for DiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiceError::InvalidNotation(msg) => write!(f, "Invalid dice notation: {}", msg),
        }
    }
}

impl std::error::Error for DiceError {}

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
}
