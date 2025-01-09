// matrix 90 degree rotation

use crate::input::input;

type Matrix = Vec<Vec<i64>>;

pub fn q4() {
    let mut matrix = Matrix::new();
    let n = input(None::<&str>);
    for _ in 0..n {
        let mut row = Vec::new();
        let line: String = input(None::<&str>);
        row.append(
            line.split(" ")
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
                .as_mut(),
        );
        matrix.push(row);
    }
    let mut rotated = Matrix::new();
    for _ in 0..n {
        let mut row = Vec::new();
        for _ in 0..n {
            row.push(0);
        }
        rotated.push(row);
    }
    for i in 0..n {
        for j in 0..n {
            rotated[i][j] = matrix[j][i];
        }
    }
    rotated = rotated
        .iter()
        .map(|i| i.iter().rev().cloned().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!();
    for i in 0..n {
        for j in 0..n {
            print!("{} ", rotated[i][j]);
        }
        println!();
    }
}
