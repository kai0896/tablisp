use std::fs;
pub mod calc;

fn main() {
    let path = "../test.csv";
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let cells = parse_csv(contents);
    let new_cells = calc::lookup_cell_references(cells);

    // let new_csv = make_csv(new_cells);
    println!("new cells: {:?}", new_cells);
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
    todo!()

}
