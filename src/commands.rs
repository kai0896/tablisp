use macroquad::prelude::*;
use crate::application::{State, Mode};

pub fn input(state: &mut State) {
    if let Some(key) = get_last_key_pressed(){
        match state.mode {
            Mode::Normal => {
                match key {
                    KeyCode::Escape => {
                        state.mode = Mode::Cell;
                        state.cells_eval[state.cell_data.selection.0][state.cell_data.selection.1] = state.insert_bar.text.clone();
                    }
                    KeyCode::Enter => {
                        state.mode = Mode::Cell;
                        state.cells_eval[state.cell_data.selection.0][state.cell_data.selection.1] = state.insert_bar.text.clone();
                    }
                    _ => (),
                }
            },
            Mode::Insert => {
                match key {
                    KeyCode::Backspace => {
                        state.insert_bar.point_pos -= 1;
                        state.insert_bar.text.remove(state.insert_bar.point_pos);
                    }
                    KeyCode::Escape => {
                        state.mode = Mode::Normal;
                        state.insert_bar.point_pos -= 1;
                    }
                    KeyCode::Enter => {
                        state.mode = Mode::Cell;
                        state.cells_eval[state.cell_data.selection.0][state.cell_data.selection.1] = state.insert_bar.text.clone();
                    }
                    _ => (),
                }
            },
            Mode::Cell => (),
        }
    }

    if let Some(keychar) = get_char_pressed() {
        debug!("{}", keychar);

        match state.mode {
            Mode::Normal => {
                match keychar {
                    'l' => state.insert_bar.point_pos += 1,
                    'h' => state.insert_bar.point_pos -= 1,
                    'i' => state.mode = Mode::Insert,
                    'a' => {
                        state.mode = Mode::Insert;
                        state.insert_bar.point_pos += 1;
                    }
                    _ => (),
                }
            },
            Mode::Insert => {
                state.insert_bar.text.insert(state.insert_bar.point_pos, keychar);
                state.insert_bar.point_pos += 1;
            },
            Mode::Cell => {
                let cl = &mut state.cell_data;
                match keychar {
                    'l' => cl.selection = (cl.selection.0+1, cl.selection.1),
                    'h' => cl.selection = (cl.selection.0-1, cl.selection.1),
                    'k' => cl.selection = (cl.selection.0, cl.selection.1-1),
                    'j' => cl.selection = (cl.selection.0, cl.selection.1+1),
                    'i' => {
                        state.mode = Mode::Insert;
                        state.insert_bar.point_pos = state.insert_bar.text.len();
                    },
                    _ => (),
                }
                state.insert_bar.text = state.cells_eval[cl.selection.0][cl.selection.1].clone();
            }
            _ => (),
        }
    }
}
