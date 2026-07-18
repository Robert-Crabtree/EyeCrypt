use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
    pub name: String,
    pub panels: Vec<Panel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub name: String,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Symbol {
    /// State of the symbol.
    /// Eye Glyphs use values 0-4.
    pub state: u8,
}