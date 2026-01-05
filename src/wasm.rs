use wasm_bindgen::prelude::*;
use crate::{roll_dice as core_roll_dice, RollOptions};

#[wasm_bindgen]
pub fn roll_dice(
    notation: &str,
    explode: Option<bool>,
    keep: Option<u32>,
    drop: Option<u32>,
    success: Option<u32>,
    crit: Option<u32>,
) -> Result<JsValue, JsValue> {
    let options = RollOptions {
        explode: explode.unwrap_or(false),
        keep,
        drop,
        success,
        crit,
    };

    match core_roll_dice(notation, &options) {
        Ok(result) => serde_wasm_bindgen::to_value(&result)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub struct WasmDiceRoller {
    notation: String,
    options: RollOptions,
}

#[wasm_bindgen]
impl WasmDiceRoller {
    #[wasm_bindgen(constructor)]
    pub fn new(notation: String) -> Self {
        Self {
            notation,
            options: RollOptions::default(),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_explode(&mut self, explode: bool) {
        self.options.explode = explode;
    }

    #[wasm_bindgen(setter)]
    pub fn set_keep(&mut self, keep: Option<u32>) {
        self.options.keep = keep;
    }

    #[wasm_bindgen(setter)]
    pub fn set_drop(&mut self, drop: Option<u32>) {
        self.options.drop = drop;
    }

    #[wasm_bindgen(setter)]
    pub fn set_success(&mut self, success: Option<u32>) {
        self.options.success = success;
    }

    #[wasm_bindgen(setter)]
    pub fn set_crit(&mut self, crit: Option<u32>) {
        self.options.crit = crit;
    }

    pub fn roll(&self) -> Result<JsValue, JsValue> {
        match core_roll_dice(&self.notation, &self.options) {
            Ok(result) => serde_wasm_bindgen::to_value(&result)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e))),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    }
}
