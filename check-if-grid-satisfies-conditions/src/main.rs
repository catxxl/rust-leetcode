struct Solution {
}

impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        for i in 0..rows {
            for j in 0..cols {
                let num = &grid[i][j];
                if i + 1 < rows {
                    if *num != grid[i + 1][j] {
                        return false;
                    }
                }
                if j + 1 < cols {
                    if *num == grid[i][j + 1] {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

fn main() {
    let grid = vec![
        vec![1],
        vec![2],
        vec![3]
    ];
    println!("satisfies_conditions: {}", Solution::satisfies_conditions(grid));
}
