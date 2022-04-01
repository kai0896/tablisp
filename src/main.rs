use std::env;
use std::fs;
pub mod calc;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(path) => {
            let contents = fs::read_to_string(path)
                .expect("Something went wrong reading the file");

            let cells = parse_csv(contents);
            let new_cells = calc::compute_cells(cells);

            let new_csv = make_csv(new_cells);
            print!("new cells: \n\n{}", new_csv);

            let mut save_path = path.clone();
            save_path.insert_str(path.len()-4, "_calced");
            fs::write(save_path, new_csv).expect("Unable to write file");

        }
        None => println!("Error: No file path provided as argument!")
    }
}

fn parse_csv(contents: String) -> Vec<Vec<String>> {

    let mut row_vec: Vec<Vec<String>> = Vec::new();

    for line_str in contents.lines() {
        let line_vec = line_str.split(',')
                               .map(|a| a.trim())
                               .map(String::from)
                               .collect::<Vec<String>>();
        row_vec.push(line_vec);
    }
    row_vec
}

fn make_csv(cells: Vec<Vec<String>>) -> String {
    let mut res = String::new();
    for i in 0..cells.len(){
        for j in 0..cells[i].len(){
            let cell = cells[i][j].clone();
            res.push_str(cell.as_str());
            res.push_str(", ");
        }
        res.pop();
        res.pop();
        res.push('\n')
    }
    res
}
