use mylib::{backtracker::backtrack, queens::build_queens, var_des::VarDes};

fn print_solution<T>(context: &[T])
where
    T: std::fmt::Display,
{
    let d: Vec<String> = context
        .iter()
        .enumerate()
        .map(|c| format!("{}:{} ", c.0, c.1))
        .collect();
    println!(" {} ", d.concat());
}

fn main() {
    let config = build_queens(22);

    let res = backtrack(&config);
    print_solution(&res);
}
