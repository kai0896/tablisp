use macroquad::prelude::*;
pub mod application;
pub mod commands;
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
        commands::input(&mut state);

        clear_background(state.theme.background);

        // insert Bar
        let font_size = *&state.text_params_items.default.font_size as f32;

        let top_bar_height = font_size + state.insert_bar.padding * 2.0;
        let insert_bar_color = if state.mode == Mode::Cell {state.theme.cells} else {state.theme.insert_bar};
        draw_rectangle(0.0, 0.0, screen_width(), top_bar_height, insert_bar_color);

        let point_width = state.font_params.char_width;
        let point_padding = font_size * 0.15;
        let point_pos_pix = point_width * state.insert_bar.point_pos as f32;

        draw_text_ex(state.insert_bar.text.as_str(),
                     state.insert_bar.padding,
                     top_bar_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.08,
                     state.text_params_items.default);

        draw_rectangle(state.insert_bar.padding + point_pos_pix - 1.0,
                       top_bar_height/2.0 - font_size /2.0 - point_padding,
                       match state.mode {Mode::Normal => point_width + 1.0, Mode::Insert => 2.0, _ => 0.0},
                       font_size + point_padding * 2.0,
                       state.theme.selection);

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
        let x_offset = cl.margin + cell_height;

        for i in 0..state.cells_eval.len(){
            let x = x_offset + (cl.width + cl.margin) * i as f32;
            let y = cl.margin + top_bar_height;
            let text = ASCII_LOWER[i].to_string();
            draw_text_ex(&text,
                         x + cl.width / 2.0 - state.font_params.char_width / 2.0,
                         y + cell_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.05,
                         state.text_params_items.dark)
        }

        for i in 0..state.cells_eval[0].len(){
            let x = cl.margin + state.cell_data.padding;
            let y = cell_height + cl.margin + top_bar_height + i as f32 * (cell_height + cl.margin);
            let text = (i + 1).to_string();
            draw_text_ex(&text,
                         x,
                         y + cell_height/2.0 + state.font_params.offset_y/2.0 - font_size * 0.05,
                         state.text_params_items.dark)
        }

        // cells

        for i in 0..state.cells_eval.len(){
            for j in 0..state.cells_eval[i].len(){
                let x = x_offset + (cl.width + cl.margin) * i as f32;
                let y = cell_height + cl.margin + top_bar_height + j as f32 * (cell_height + cl.margin);
                draw_rectangle(x, y, cl.width, cell_height, state.theme.cells);

                if cl.selection == (i, j) {
                    draw_rectangle_lines(x, y, cl.width, cell_height, cl.margin, state.theme.selection);
                }

                let mut text = state.cells_eval[i][j].clone();
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

        next_frame().await
    }
}
