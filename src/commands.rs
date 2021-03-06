use crate::application::{State, Mode};
use crate::eval::calc::{compute_cells, get_cell_type};

pub type Command = fn(&mut State);

pub fn cell_to_insert(state: &mut State) {
    if let Some(_) = state.cells_eval.get(state.cell_data.selection.0)
                                     .and_then(|a| a.get(state.cell_data.selection.1)) {
        state.mode = Mode::Insert;
        state.insert_bar.point_pos = state.insert_bar.text.len();
    } else {
        state.info_bar.log.push("Cell not there yet TODO: creat cell".to_string());
    }
}

pub fn normal_right(state: &mut State) {
    if state.insert_bar.text.len() > 0 && state.insert_bar.point_pos < state.insert_bar.text.len() - 1 {
        state.insert_bar.point_pos += 1;
    }
}

pub fn normal_left(state: &mut State) {
    if state.insert_bar.point_pos > 0 {
        state.insert_bar.point_pos -= 1;
    }
}

pub fn normal_to_insert_i(state: &mut State) {
    state.mode = Mode::Insert;
}

pub fn normal_to_insert_a(state: &mut State) {
    state.mode = Mode::Insert;
    state.insert_bar.point_pos += 1;
}

pub fn insert_input(state: &mut State) {
    let keychar = state.last_key;
    state.insert_bar.text.insert(state.insert_bar.point_pos , keychar);
    state.insert_bar.point_pos += 1;
}

pub fn cell_down(state: &mut State) {
    cell_move(state, (1,0))
}

pub fn cell_up(state: &mut State) {
    cell_move(state, (-1,0))
}

pub fn cell_left(state: &mut State) {
    cell_move(state, (0,-1))
}

pub fn cell_right(state: &mut State) {
    cell_move(state, (0,1))
}

fn cell_move(state: &mut State, movement: (i32, i32)) {
    let cl = &mut state.cell_data;
    let move_cell = |o, m| {
        ((o as i32 + m) as i32).max(0) as usize
    };
    cl.selection = (move_cell(cl.selection.0, movement.0),
                    move_cell(cl.selection.1, movement.1));

    if let Some(cell) = state.cells_eval.get(cl.selection.0).and_then(|a| a.get(cl.selection.1)) {
        state.insert_bar.text = cell.content.clone();
    }
}

pub fn to_cell_mode(state: &mut State) {
    state.mode = Mode::Cell;
    let text = &state.insert_bar.text;
    state.cells_eval[state.cell_data.selection.0][state.cell_data.selection.1].content = text.clone();
    state.cells_eval[state.cell_data.selection.0][state.cell_data.selection.1].result = None;
    state.cells_eval[state.cell_data.selection.0][state.cell_data.selection.1].cell_type = get_cell_type(text);
    compute_cells(&mut state.cells_eval);
}

pub fn insert_to_normal(state: &mut State) {
    state.mode = Mode::Normal;
    if state.insert_bar.text.len() > 0{
        state.insert_bar.point_pos -= 1;
    }
}

pub fn insert_remove_char(state: &mut State) {
    if state.insert_bar.text.len() > 0{
        state.insert_bar.point_pos -= 1;
        state.insert_bar.text.remove(state.insert_bar.point_pos);
    } else {
        state.info_bar.log.push("Nothing there to remove".to_string());
    }
}
