// EXO1 : Multiply two numbers
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

// EXO2 : Multiply two matrices
fn multiply_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a[0].len() != b.len() {
        panic!("The number of columns in the first matrix must be equal to the number of rows in the second matrix");
    }

    let mut result = vec![vec![0; b[0].len()]; a.len()];

    for i in 0..a.len() {
        for j in 0..b[0].len() {
            for k in 0..a[0].len() {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    return result;
}

// EXO3 : Transpose a matrix
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }

    return result;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

// EXO4 : Is palindrome
fn is_palindrome(s: &str) -> bool{
    let s = s.replace(&[',', '!', '.', '\'', ':', ' '][..], "");
    let mut low = 0;
    let mut high = s.len() - 1;
    while (low < high) {
        if (s.as_bytes()[low].to_ascii_lowercase() != s.as_bytes()[high].to_ascii_lowercase()) {
            return false;
        }
        low += 1;
        high -= 1;
    }
    return true;
}

fn main() {
    /*let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));*/

    /*let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    let matrix_b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    let result_matrix = multiply_matrices(&matrix_a, &matrix_b);

    // Print the result
    for i in 0..result_matrix.len() {
        for j in 0..result_matrix[0].len() {
            print!("{} ", result_matrix[i][j]);
        }
        println!();
    }*/

    /*let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);*/

    let palindrome1 = "A man, a plan, a canal, Panama";
    let palindrome2 = "Madam, in Eden, I'm Adam";
    let non_palindrome = "Having class 8:30am is fun!!";

    println!("Is '{}' a palindrome? {}", palindrome1, is_palindrome(palindrome1)); // True
    println!("Is '{}' a palindrome? {}", palindrome2, is_palindrome(palindrome2)); // True
    println!("Is '{}' a palindrome? {}", non_palindrome, is_palindrome(non_palindrome)); // False
}
