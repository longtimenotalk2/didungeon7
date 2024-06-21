use std::{fs::{self, File}, io::{Read, Write}};

use miniserde::json;

use crate::game701::board::Board;

use anyhow::Result;

impl Board {
    pub fn save_default(&self) {
        let path = "save/save0.sav";
        fs::create_dir("/save").unwrap_or(());
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(_) => {
                File::open(path).unwrap()
            }
        };
        let data = json::to_string(&self);
        file.write_all(data.as_bytes()).unwrap_or(());
    }

    pub fn load_default() -> Result<Board> {
        let path = "save/save0.sav";
        let data = fs::read_to_string(path)?;
        let board: Board = json::from_str(&data)?;
        Ok(board)
    }
}