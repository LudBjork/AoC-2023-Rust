use std::fs;

pub fn solution(filename: &str) -> String {
    let problem_input = fs::read_to_string(filename).expect("File to exist");

    println!("{}", problem_input);
    return problem_input;
}
