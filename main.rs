mod data; // read comlete tables from data.rs module
const ROWS: usize = 6;
const COLS: usize = 256;
const NUMBERS: [usize; 6] = [2, 3, 9, 10, 11, 12]; // A row of MixColumn matrix

// From the given NUMBERS computes the final multiplication tables
// First element of each row contains the multiplied constant from the NUMBERS
fn mixcolumn_tables() -> Vec<Vec<i32>> {
    let mut final_tables: Vec<Vec<i32>> = Vec::new();
    for i in 0..ROWS {
        let mut row_data: Vec<i32> = Vec::new();

        for j in 0..COLS {
            let temp =
                data::ALOGTABLE[(data::LOGTABLE[NUMBERS[i]] + data::LOGTABLE[j]) % 255] as usize;
            row_data.push(temp.try_into().unwrap());
        }
        final_tables.push(row_data);
    }
    final_tables
}

fn main() {
    let mult_table_a = mixcolumn_tables();

    println!("6 tables containing all possible multiplications:");
    for row in mult_table_a {
        println!("{:?}", row);
    }
}
