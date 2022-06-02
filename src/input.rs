use macroquad::prelude::*;
use crate::application::{State, Mode};
use crate::commands::*;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum KeyComp {
    Mod(KeyCode),
    NoMod(char),
    AnyChar,
    Ctrl(char),
    Alt(char),
}

pub struct KeyMap(HashMap<Mode, HashMap<KeyComp, Command>>);

pub fn init_input() -> KeyMap {
    let mut keys_cell: HashMap<KeyComp, Command> = HashMap::new();
    keys_cell.insert(KeyComp::NoMod('i'), cell_to_insert);
    keys_cell.insert(KeyComp::NoMod('j'), cell_down);
    keys_cell.insert(KeyComp::NoMod('k'), cell_up);
    keys_cell.insert(KeyComp::NoMod('h'), cell_left);
    keys_cell.insert(KeyComp::NoMod('l'), cell_right);

    let mut keys_insert: HashMap<KeyComp, Command> = HashMap::new();
    keys_insert.insert(KeyComp::AnyChar, insert_input);
    keys_insert.insert(KeyComp::Mod(KeyCode::Enter), to_cell_mode);
    keys_insert.insert(KeyComp::Mod(KeyCode::Escape), insert_to_normal);
    keys_insert.insert(KeyComp::Mod(KeyCode::Backspace), insert_remove_char);

    let mut keys_normal: HashMap<KeyComp, Command> = HashMap::new();
    keys_normal.insert(KeyComp::NoMod('l'), normal_left);
    keys_normal.insert(KeyComp::NoMod('h'), normal_right);
    keys_normal.insert(KeyComp::NoMod('i'), normal_to_insert_i);
    keys_normal.insert(KeyComp::NoMod('a'), normal_to_insert_a);
    keys_normal.insert(KeyComp::Mod(KeyCode::Enter), to_cell_mode);
    keys_normal.insert(KeyComp::Mod(KeyCode::Escape), to_cell_mode);

    let mut keymap = HashMap::new();
    keymap.insert(Mode::Cell, keys_cell);
    keymap.insert(Mode::Insert, keys_insert);
    keymap.insert(Mode::Normal, keys_normal);

    KeyMap(keymap)
}

pub fn handle_input(state: &mut State) {
    if let Some(keychar) = get_char_pressed() {
        if let Some(mode) = state.keymap.0.get(&state.mode) {
            if let Some(command) = mode.get(&KeyComp::NoMod(keychar)) {
                command(state);
            } else {
                if let Some(command) = mode.get(&KeyComp::AnyChar) {
                    state.last_key = keychar;
                    command(state);
                } else {
                    state.info_bar.log.push(format!("{} is undefined in {:?} mode", keychar, state.mode));
                }
            }
        }
        return
    }

    if let Some(key) = get_last_key_pressed(){
        if let Some(mode) = state.keymap.0.get(&state.mode) {
            if let Some(command) = mode.get(&KeyComp::Mod(key)) {
                command(state);
            }
        }
    }
}
