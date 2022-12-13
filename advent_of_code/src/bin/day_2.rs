use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const WIN_SCORE: i8 = 6;
const DRAW_SCORE: i8 = 3;
const LOSE_SCORE: i8 = 0;

#[derive(Copy, Clone)]
enum ResultType {
    Rock,
    Paper,
    Scissors
}

fn calc_score(my_result: ResultType, opponent_result: ResultType) -> i8 {
    // Decide Winner
    let my_result_int = my_result as i8;
    let opponent_result_int = opponent_result as i8;
    let win_needed_value = (opponent_result_int + 1) % 3;
    println!("Checking results for {} VS {}", my_result_int, opponent_result_int);
    println!("Opponent will be beaten by {}", win_needed_value);
    // If my result beats opponent
    return if my_result_int == win_needed_value {
        WIN_SCORE + my_result_int + 1
    } else if opponent_result_int == my_result_int {
        DRAW_SCORE + my_result_int + 1
    } else {
        LOSE_SCORE + my_result_int + 1
    }
}


fn calc_scorev2(win_result: &str, opponent_result: ResultType) -> i8 {
    match win_result {
        // LOSE
        "X" => {
            let my_result = (opponent_result as i8 - 1).rem_euclid( 3);
            LOSE_SCORE + my_result + 1
        },
        // DRAW
        "Y" => {
            let my_result = opponent_result as i8;
            DRAW_SCORE + my_result + 1
        },
        // WIN
        "Z" => {
            let my_result = (opponent_result as i8 + 1).rem_euclid(3);
            WIN_SCORE + my_result + 1
        },
        _ => 0
    }
}

fn main() {
    // let a = ResultType::Scissors;
    // let b = ResultType::Scissors;
    // let score_test = score(a, b);
    // println!("Score: {}", score_test)
    let filename = "resources/2022-day-2";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_score: i64 = 0;

    /*
        A, X = Rock = 1 point
        B, Y = Paper = 2 points
        C, Z = Scissors = 3 points

        ROCK < PAPER < SCISSORS < ROCK
     */

    let opponent_input = HashMap::from([
        ("A", ResultType::Rock),
        ("B", ResultType::Paper),
        ("C", ResultType::Scissors),
    ]);

    let my_input = HashMap::from([
        ("X", ResultType::Rock),
        ("Y", ResultType::Paper),
        ("Z", ResultType::Scissors),
    ]);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        let values = line_str.split_once(' ');
        let score = match values {
            Some((col1, col2)) => calc_scorev2(col2, opponent_input[col1]),
            None => 0,
        };
        total_score += score as i64;
    }

    println!("{}", total_score);
}