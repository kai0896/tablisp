use macroquad::prelude::*;
use std::fs;
use crate::eval::parse_csv;
use crate::eval::calc::Cell;
use crate::eval::calc::compute_cells;
use crate::input::*;

pub struct State {
    pub mode: Mode,
    pub text_params_items: TextParamsItems,
    pub font_params: FontParams,
    pub insert_bar: InsertBar,
    // pub cells_raw: Vec<Vec<String>>,
    pub cells_eval: Vec<Vec<Cell>>,
    pub cell_data: CellData,
    pub theme: Theme,
    pub keymap: KeyMap,
    pub last_key: char,
}

pub struct FontParams {
    pub char_width: f32,
    pub offset_y: f32,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Mode {
    Normal,
    Insert,
    Cell
}

pub struct InsertBar {
    pub text: String,
    pub padding: f32,
    pub point_pos: usize,
}

pub struct CellData {
    pub padding: f32,
    pub margin: f32,
    pub width_char: usize,
    pub width: f32,
    pub selection: (usize, usize),
}

pub struct Theme {
    pub background: Color,
    pub text: Color,
    pub text_dark: Color,
    pub selection: Color,
    pub insert_bar: Color,
    pub cells: Color,
}

pub struct TextParamsItems {
    pub default: TextParams,
    pub dark: TextParams,
    pub black: TextParams
}

pub async fn init_state() -> State {

    let font = load_ttf_font("../assets/fonts/robotoMono/RobotoMono-Regular.ttf").await.unwrap();

    let theme = Theme {
        background: Color::from_rgba(24, 26, 19, 255),
        text: Color::from_rgba(229, 213, 180, 255),
        text_dark: Color::from_rgba(107, 101, 84, 255),
        selection: Color::from_rgba(229, 198, 114, 255),
        insert_bar: Color::from_rgba(46, 42, 32, 255),
        cells: Color::from_rgba(17, 18, 11, 255)
    };

    let default = TextParams {
        font,
        font_size: 24,
        font_scale: 1.0,
        font_scale_aspect: 1.0,
        color: theme.text,
    };

    let mut dark = default.clone();
    dark.color = theme.text_dark;
    let mut black = default.clone();
    black.color = BLACK;

    let text_params_items = TextParamsItems {
        default,
        dark,
        black,
    };

    // let cells_eval = vec!(vec!("".to_string(), "2".to_string(), "4".to_string()),
    //                       vec!("".to_string(), "Hi".to_string()));
    let csv_content = fs::read_to_string("../assets/test.csv").unwrap();
    let mut cells_eval = parse_csv(csv_content);
    compute_cells(&mut cells_eval);

    let text_dimensions = measure_text("Hay",
                                       Some(default.font),
                                       default.font_size,
                                       1.0);
    let font_params = FontParams {
        char_width: text_dimensions.width/3.0,
        offset_y: text_dimensions.offset_y,

    };

    let width_char = 12;
    let cell_data = CellData {
        padding: 8.0,
        margin: 4.0,
        width_char,
        width: width_char as f32 * font_params.char_width,
        selection: (0,0)
    };

    let insert_bar = InsertBar {
        text: cells_eval[cell_data.selection.0][cell_data.selection.1].content.clone(),
        padding: 16.0,
        point_pos: 0
    };

    let keymap = init_input();

    State {
        mode: Mode::Cell,
        text_params_items,
        font_params,
        insert_bar,
        cells_eval,
        cell_data,
        theme,
        keymap,
        last_key: ' ',
    }
}
