use std::fs::File;
use std::io::{self, Result, Write};

fn main() -> Result<()> {
    let mut line = String::new();
    let mut file = File::create("n_queens.cnf")?;
    println!("Enter a number to get with and height >3 of chess board: > 3");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    let n: usize = line
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Please provide a number > 3"))?;

    write_dimacs_header(&mut file, n)?;
    encode_n_queens_constraints(&mut file, n)?;
    Ok(())
}

fn write_dimacs_header(file: &mut File, n: usize) -> Result<()> {
    writeln!(file, "c N-Queens Problem for N={}", n)?;
    writeln!(file, "p cnf {} {}", n * n, (n * (n - 1)) * 3 + n * 2)?;
    Ok(())
}
fn encode_n_queens_constraints(file: &mut File, n: usize) -> Result<()> {
    for i in 0..n {
        encode_row_constraints(file, i, n)?;
    }
    for j in 0..n {
        encode_column_constraints(file, j, n)?;
    }
    for i in 0..2 * n - 1 {
        encode_diagonal_constraints(file, i, n, true)?;
        encode_diagonal_constraints(file, i, n, false)?;
    }
    Ok(())
}

fn encode_row_constraints(file: &mut File, i: usize, n: usize) -> Result<()> {
    let mut clause = Vec::new();
    for j in 0..n {
        clause.push(format!("{}", i * n + j + 1));
    }
    writeln!(file, "{} 0", clause.join(" "))?;
    for j in 0..n {
        for k in j + 1..n {
            writeln!(file, "-{} -{} 0", i * n + j + 1, i * n + k + 1)?;
        }
    }
    Ok(())
}

fn encode_column_constraints(file: &mut File, j: usize, n: usize) -> Result<()> {
    for i in 0..n {
        for k in i + 1..n {
            writeln!(file, "-{} -{} 0", i * n + j + 1, k * n + j + 1)?;
        }
    }
    Ok(())
}

fn encode_diagonal_constraints(
    file: &mut File,
    d: usize,
    n: usize,
    is_positive: bool,
) -> Result<()> {
    let mut constraints = Vec::new();
    if is_positive {
        let start = if d < n { 0 } else { d - n + 1 };
        let end = if d < n { d + 1 } else { n };
        for i in start..end {
            let j = d - i;
            constraints.push(i * n + j + 1);
        }
    } else {
        let start = if d < n { 0 } else { d - n + 1 };
        let end = if d < n { d + 1 } else { n };
        for i in start..end {
            let j = n - 1 - (d - i);
            constraints.push(i * n + j + 1);
        }
    }
    for i in 0..constraints.len() {
        for k in i + 1..constraints.len() {
            writeln!(file, "-{} -{} 0", constraints[i], constraints[k])?;
        }
    }
    Ok(())
}
