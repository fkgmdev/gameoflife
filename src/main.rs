fn main() {
    let mut matrix = vec![vec![false; 10]; 10];
    // matrix[5][5] = true;
    // matrix[5][6] = true;
    // matrix[6][5] = true;
    matrix[5][4] = true;
    matrix[5][5] = true;
    matrix[5][6] = true;

    for _ in 0..11 {
        let mut next = matrix.clone();
        for (ci, column) in matrix.iter().enumerate() {
            for (ri, &square) in column.iter().enumerate() {
                let neighbours = count_neighbours(&matrix, ci, ri);
                next[ci][ri] = match (square, neighbours) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                }
            }
        }
        matrix = next;
        print_matrix(&matrix);
    }
}

fn count_neighbours(matrix: &Vec<Vec<bool>>, ci: usize, ri: usize) -> i32 {
    let mut n = 0;
    for dc in [-1i32, 0, 1] {
        for dr in [-1i32, 0, 1] {
            if dc == 0 && dr == 0 {
                continue;
            }
            let c = ci as i32 + dc;
            let r = ri as i32 + dr;
            if c < 0 || r < 0 {
                continue;
            }
            if let Some(&alive) = matrix.get(c as usize).and_then(|col| col.get(r as usize)) {
                if alive {
                    n += 1;
                }
            }
        }
    }
    n
}
fn print_matrix(matrix: &Vec<Vec<bool>>) {
    for row in matrix {
        for cell in row {
            match cell {
                true => {
                    print!("█");
                }
                false => print!(" "),
            }
            print!("-");
        }
        println!("");
    }
}
