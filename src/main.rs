use macroquad::prelude::*;
pub mod application;
pub mod commands;
pub mod eval;
pub mod input;
use application::Mode;

static ASCII_LOWER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E',
    'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y',
    'Z',
];

#[macroquad::main("Exit dialog")]
async fn main() {

    let mut state = application::init_state().await;

    loop {
        input::handle_input(&mut state);

        clear_background(state.theme.background);

        // insert Bar

        let font_size = *&state.text_params_items.default.font_size as f32;

        let top_bar_height = font_size + state.insert_bar.padding * 2.0;
        let insert_bar_color = if state.mode == Mode::Cell {state.theme.cells} else {state.theme.insert_bar};
        let point_width = state.font_params.char_width;
        let point_padding = font_size * 0.15;
        let point_pos_pix = point_width * state.insert_bar.point_pos as f32;

        // bar
        draw_rectangle(0.0, 0.0, screen_width(), top_bar_height, insert_bar_color);

        // text
        draw_text_ex(state.insert_bar.text.as_str(),
                     state.insert_bar.padding,
                     top_bar_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.08,
                     state.text_params_items.default);

        // cell positon
        let cell_pos_text = ASCII_LOWER[state.cell_data.selection.1].to_string()
            + &(state.cell_data.selection.0 + 1).to_string();
        draw_text_ex(&cell_pos_text,
                     screen_width() - state.insert_bar.padding - (cell_pos_text.len() as f32 * state.font_params.char_width),
                     top_bar_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.08,
                     state.text_params_items.dark);

        // point
        draw_rectangle(state.insert_bar.padding + point_pos_pix - 1.0,
                       top_bar_height/2.0 - font_size /2.0 - point_padding,
                       match state.mode {Mode::Normal => point_width + 1.0, Mode::Insert => 2.0, _ => 0.0},
                       font_size + point_padding * 2.0,
                       state.theme.selection);

        // char in normal mode point
        if state.mode == Mode::Normal && state.insert_bar.text.len() > 0 {
            let text = (state.insert_bar.text.as_bytes()[state.insert_bar.point_pos] as char).to_string();
            draw_text_ex(text.as_str(),
                         state.insert_bar.padding + point_pos_pix,
                         top_bar_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.08,
                         state.text_params_items.black);
        }

        // cell labels

        let cl = &state.cell_data;
        let cell_height = font_size + cl.padding * 2.0;
        let x_offset = cl.padding * 2.0 + state.font_params.char_width * 2.0;
        let cell_cols = (screen_width() / (cl.width + cl.margin) as f32) as usize + 1;
        let cell_rows = (screen_height() / (cell_height + cl.margin) as f32) as usize - 2;

        // col labels
        for i in 0..cell_cols{
            let x = x_offset + (cl.width + cl.margin) * i as f32;
            let y = cl.margin + top_bar_height;
            let text = ASCII_LOWER[i].to_string();
            draw_text_ex(&text,
                         x + cl.width / 2.0 - state.font_params.char_width / 2.0,
                         y + cell_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.05,
                         state.text_params_items.dark)
        }

        // row labels
        for i in 0..cell_rows{
            let text = (i + 1).to_string();
            let x = cl.padding + (2 - text.len()) as f32 * state.font_params.char_width;
            let y = cell_height + cl.margin + top_bar_height + i as f32 * (cell_height + cl.margin);
            draw_text_ex(&text,
                         x,
                         y + cell_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.05,
                         state.text_params_items.dark)
        }

        // cells

        for i in 0..cell_rows{
            for j in 0..cell_cols{
                let x = x_offset + (cl.width + cl.margin) * j as f32;
                let y = cell_height + cl.margin + top_bar_height + i as f32 * (cell_height + cl.margin);
                draw_rectangle(x, y, cl.width, cell_height, state.theme.cells);

                // draw outline of selected cell
                if cl.selection == (i, j) {
                    draw_rectangle_lines(x, y, cl.width, cell_height, cl.margin, state.theme.selection);
                }

                // get, truncate and draw the cell text
                let mut text = if let Some(cell) = state.cells_eval.get(i) .and_then(|a| a.get(j)) {
                    match &cell.result {
                        Some(str) => str.clone(),
                        None => cell.content.clone()
                    }
                }
                else {
                    "".to_string()
                };

                if text.len() > state.cell_data.width_char-1 {
                    text.truncate(state.cell_data.width_char - 2);
                    text.push('…');
                }

                draw_text_ex(&text,
                             x + cl.padding,
                             y + cell_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.05,
                             state.text_params_items.default)
            }
        }

        // info bar

        let info_bar_height = font_size + state.info_bar.padding * 2.0;
        let info_bar_pos_y = screen_height() - info_bar_height;
        draw_rectangle(0.0,
                       info_bar_pos_y,
                       screen_width(),
                       info_bar_height,
                       state.theme.background);

        if let Some(text) = state.info_bar.log.last() {
            draw_text_ex(text,
                         state.info_bar.padding,
                         info_bar_pos_y + info_bar_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.08,
                         state.text_params_items.default);
        }

        // wait for next frame
        next_frame().await
    }
}
