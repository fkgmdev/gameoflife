fn main() {
    // matrix[column][row]
    let mut matrix = vec![vec![false; 10]; 10];

    // Preset: glider (translates one cell diagonally every 4 generations).
    matrix[1][0] = true;
    matrix[2][1] = true;
    matrix[0][2] = true;
    matrix[1][2] = true;
    matrix[2][2] = true;

    // Other presets to try (comment out the glider above first):
    //
    // Blinker (period-2 oscillator):
    // matrix[5][4] = true;
    // matrix[5][5] = true;
    // matrix[5][6] = true;
    //
    // Block (still life, never changes):
    // matrix[4][4] = true;
    // matrix[5][4] = true;
    // matrix[4][5] = true;
    // matrix[5][5] = true;
    //
    // Toad (period-2 oscillator):
    // matrix[4][5] = true;
    // matrix[5][5] = true;
    // matrix[6][5] = true;
    // matrix[3][6] = true;
    // matrix[4][6] = true;
    // matrix[5][6] = true;

    for generation in 1..=16 {
        matrix = step(&matrix);
        println!("--- generation {generation} ---");
        print_matrix(&matrix);
    }
}

/// Advance the whole grid by one generation. Uses a double buffer so cells
/// are all evaluated against the same previous state.
fn step(matrix: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut next = matrix.to_vec();
    for (ci, column) in matrix.iter().enumerate() {
        for (ri, &square) in column.iter().enumerate() {
            let neighbours = count_neighbours(matrix, ci, ri);
            next[ci][ri] = match (square, neighbours) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    next
}

fn count_neighbours(matrix: &[Vec<bool>], ci: usize, ri: usize) -> i32 {
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

fn print_matrix(matrix: &[Vec<bool>]) {
    // matrix is [column][row]; print rows so the output reads like the grid.
    let rows = matrix.first().map_or(0, |column| column.len());
    let cols = matrix.len();
    for r in 0..rows {
        for c in 0..cols {
            print!("{}", if matrix[c][r] { '█' } else { ' ' });
            print!(" ");
        }
        println!();
    }
}
