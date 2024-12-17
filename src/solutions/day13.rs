use std::io::BufRead;

use crate::common::io::file_reader;

pub fn run(file_path: &str) -> (i64, i64) {
    let equations = parse_equations(file_path);
    let sol_range = 0..100;
    let easy: i64 = equations.iter()
        .filter_map(Equation::solve)
        .filter(|(a, b)| sol_range.contains(a) && sol_range.contains(b))
        .map(|(a, b)| a * 3 + b )
        .sum();

    let adjusted_equations: Vec<Equation> = equations.iter()
        .map(|e| Equation{matrix: e.matrix, target: Equation::add(&e.target, 10000000000000)})
        .collect();

    let less_easy: i64 = adjusted_equations.iter()
        .filter_map(Equation::solve)
        .map(|(a, b)| a * 3 + b )
        .sum();
    
    return (easy, less_easy);
}

fn parse_equations(file_path: &str) -> Vec<Equation> {
    let mut reader = file_reader(file_path).lines().peekable();

    let mut res = Vec::new();
    while reader.peek().is_some() {
        let (a_x, a_y) = extract_xy(&reader.next().unwrap().unwrap(), "Button _: X+");
        let (b_x, b_y) = extract_xy(&reader.next().unwrap().unwrap(), "Button _: X+");
        let (prize_x, prize_y) =   extract_xy(&reader.next().unwrap().unwrap(), "Prize: X=");

        reader.next();

        res.push(Equation{
            matrix: [[a_x, b_x], [a_y, b_y]],
            target: [prize_x, prize_y]
        });
    }
    return res;
}

fn extract_xy(button_line: &str, string_to_skip: &str) -> (i64, i64) {
    // can check 'startswith' if we _really_ feel like calling some functions

    let x = button_line.chars()
        .skip(string_to_skip.len())
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>().unwrap();

    let y = {
        let y_reversed = button_line.chars().rev()
        .take_while(|c| c.is_digit(10))
        .collect::<String>();

        y_reversed.chars().rev().collect::<String>().parse::<i64>().unwrap()
    };

    return (x,y);
}


struct Equation{
    matrix: [[i64; 2]; 2],
    target: [i64; 2]
}
impl Equation {
    fn solve(&self) -> Option<(i64, i64)> {
        let coefficients = Equation::mul_matrix_vec(&self.adjugate(), &self.target);
        let d = self.det();
        if coefficients[0] % d != 0 || coefficients[1] % d != 0 {
            return None
        }
        let [a_press, b_press] = Equation::div(&coefficients, d);
        return Some((a_press, b_press));
    }

    fn det(&self) -> i64                { self.matrix[0][0]*self.matrix[1][1] - self.matrix[0][1]*self.matrix[1][0] }
    fn adjugate(&self) -> [[i64; 2]; 2] { [[self.matrix[1][1], -self.matrix[0][1]], [-self.matrix[1][0], self.matrix[0][0]]] }

    fn mul_matrix_vec(m: &[[i64; 2]; 2], v: &[i64; 2]) -> [i64; 2] {
        [
            m[0][0]*v[0] + m[0][1]*v[1],
            m[1][0]*v[0] + m[1][1]*v[1],
        ]
    }
    fn div(v: &[i64; 2], div: i64) -> [i64; 2] { [v[0]/div, v[1]/div] }
    fn add(v: &[i64; 2], c: i64) -> [i64; 2]   { [v[0]+c, v[1]+c] }

}