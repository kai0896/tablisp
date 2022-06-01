use std::fs;
pub mod calc;
use calc::Cell;

pub fn eval_csv_at_path(path: String) {
    let mut cells = parse_csv(path.clone());
    calc::compute_cells(&mut cells);

    let new_csv = make_csv(cells);
    print!("new cells: \n\n{}", new_csv);

    let mut save_path = path.clone();
    save_path.insert_str(path.len()-4, "_calced");
    fs::write(save_path, new_csv).expect("Unable to write file");
}

pub fn parse_csv(contents: String) -> Vec<Vec<Cell>> {

    let mut row_vec: Vec<Vec<Cell>> = Vec::new();

    for line_str in contents.lines() {
        let line_vec = line_str.split(',')
                               .map(|a| a.trim())
                               .map(String::from)
                               .map(|a| Cell {content: a, result: None})
                               .collect::<Vec<Cell>>();
        row_vec.push(line_vec);
    }
    row_vec
}

pub fn make_csv(cells: Vec<Vec<Cell>>) -> String {
    let mut res = String::new();
    for i in 0..cells.len(){
        for j in 0..cells[i].len(){
            let cell = cells[i][j].content.clone();
            res.push_str(cell.as_str());
            res.push_str(", ");
        }
        res.pop();
        res.pop();
        res.push('\n')
    }
    res
}
