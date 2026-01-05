use wasm_bindgen::prelude::*;
use crate::{parse_dice_notation, DiceRoll};

#[wasm_bindgen]
pub fn roll_dice(notation: &str) -> Result<JsValue, JsValue> {
    match parse_dice_notation(notation) {
        Ok(roll) => {
            serde_wasm_bindgen::to_value(&roll)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub struct WasmDiceRoller {
    notation: String,
    explode: bool,
    keep: Option<u32>,
    success: Option<u32>,
}

#[wasm_bindgen]
impl WasmDiceRoller {
    #[wasm_bindgen(constructor)]
    pub fn new(notation: String) -> Self {
        Self {
            notation,
            explode: false,
            keep: None,
            success: None,
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_explode(&mut self, explode: bool) {
        self.explode = explode;
    }

    #[wasm_bindgen(setter)]
    pub fn set_keep(&mut self, keep: Option<u32>) {
        self.keep = keep;
    }

    #[wasm_bindgen(setter)]
    pub fn set_success(&mut self, success: Option<u32>) {
        self.success = success;
    }

    pub fn roll(&self) -> Result<JsValue, JsValue> {
        roll_dice(&self.notation)
    }
}
