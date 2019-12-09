use std::fs;



use crate::intcode;

pub fn solve() -> Option<i32> {
    let filename = "input/day2.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let original_input: Vec<i32> = contents
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    let mut answer = None;
    for noun in 0..100 {
        for verb in 0..100 {
            let result = try_input(&original_input, noun, verb);
            if result == 19_690_720 {
                answer = Some(100 * noun + verb);
                break;
            }
        }
    }
    answer
}

fn try_input(data: &[i32], noun: i32, verb: i32) -> i32 {
    let mut try_data = data.to_owned();
    try_data[1] = noun;
    try_data[2] = verb;
    let output = intcode::program(try_data);
    output[0]
}


