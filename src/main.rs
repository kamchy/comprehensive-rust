fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    for i in 0..3 {
        for j in 0..3 {
            res[i][j] = matrix[j][i];
        }
    }
    return res;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for r in matrix {
        print!("[");
        for v in r {
            print!(" {}", v);
        }

        println!("]");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
