use std::env;
use std::fs::File;
use std::io::{Result, Write};

fn write_solution_matrix_to_file(solution_matrix: &Vec<Vec<f64>>) -> Result<()> {
    let mut file = File::create("solution_matrix.txt")?;

    for row in solution_matrix {
        let row_string: Vec<String> = row.iter().map(|value| value.to_string()).collect();
        let row_line = row_string.join(" ") + "\n";
        file.write_all(row_line.as_bytes())?;
    }

    Ok(())
}

fn compute_d(d: &mut Vec<Vec<f64>>, times: usize, length: usize, dt: f64, dx: f64, l: f64, gamma: f64, c: f64) {
    for t in 1..times-1 {
        for i in 2..length-2 {
            let outer_fact = (1.0 / (c.powi(2) * dt.powi(2)) + gamma / (2.0 * dt)).recip();
            let p1 = 1.0 / dx.powi(2) * (d[t][i-1] - 2.0 * d[t][i] + d[t][i+1]);
            let p2 = 1.0 / (c.powi(2) * dt.powi(2)) * (d[t-1][i] - 2.0 * d[t][i]);
            let p3 = gamma / (2.0 * dt) * d[t-1][i];
            let p4 = l.powi(2) / dx.powi(4) * (d[t][i+2] - 4.0 * d[t][i+1] + 6.0 * d[t][i] - 4.0 * d[t][i-1] + d[t][i-2]);
            d[t+1][i] = outer_fact * (p1 - p2 + p3 - p4);
        }
    }
}

fn main() {
	let args: Vec<String> = env::args().collect();
    if args.len() != 9 {
        eprintln!("Error: Invalid number of arguments.");
        eprintln!("Usage: ./main Nx Nt L f dt EAk_2 gamma dx");
    }
    let n_x: usize = args[1].parse().unwrap();
    let n_t: usize = args[2].parse().unwrap();
    let l: f64 = args[3].parse().unwrap();
    let f: f64 = args[4].parse().unwrap();
    let dt: f64 = args[5].parse().unwrap();
    let EAk_2: f64 = args[6].parse().unwrap();
    let gamma: f64 = args[7].parse().unwrap();
	let dx: f64 = args[8].parse().unwrap();
	let c:f64 = 2.0*l*f;
 
    let mut ya: Vec<f64> = (0..70).map(|i| i as f64 * 0.01 / 69.0).collect();
    let mut yb: Vec<f64> = (0..31).map(|i| (30 - i) as f64 * 0.01 / 30.0).collect();

    let mut y0 = Vec::new();
    y0.append(&mut ya);
    y0.append(&mut yb);


    let mut solution_matrix = vec![vec![0.0; n_x]; n_t];
    solution_matrix[0] = y0.clone();
    solution_matrix[1] = y0.clone();

    compute_d(&mut solution_matrix, n_t, n_x, dt, dx, EAk_2, gamma, c);

    if let Err(err) = write_solution_matrix_to_file(&solution_matrix) {
        eprintln!("Error writing to file: {}", err);
    }
}