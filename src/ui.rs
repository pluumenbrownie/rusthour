
// mod board;
#[path ="solvers.rs"]
pub mod solvers;

use std::fs::{self, ReadDir};

use solvers::board::Board;
use read_input::prelude::*;
use regex::Regex;


pub fn list_boards() -> Result<ReadDir, std::io::Error> {
    fs::read_dir("./gameboards")
}


pub fn print_boards() {
    let paths = list_boards().unwrap();
    // number, letter x, number, underscore, an identifier
    let re = Regex::new(r"(?<name>\d+x\d+_[0-9a-zA-Z]+)").unwrap();

    for path in paths {
        // print the path
        println!("Path: {}", path.as_ref().unwrap().path().display());

        // variable to put the name in
        let mut name = String::new();
        // run regex and do magic
        re.captures(
            path.map(|dir_entry| dir_entry.file_name())
            .unwrap()
            .to_str()
            .unwrap()
        )
            // put result in name variable
            .map(|result| result.expand("$name", &mut name));

        // set to placeholder when result was empty
        if name == "" {
            name = String::from("No valid name.");
        }
        println!("Name: {}", name);
    }
}


pub fn play(filename: &str) -> Result<u64, ()> {
    let re = Regex::new(r"\d+").unwrap();
    let Some(size) = re.captures(filename) else {panic!("Regex failed.")};
    let mut board = Board::new(size[0].parse::<u8>().expect("Parsing failed."));
    board.fill(filename);

    let mut score = 0;

    board.show(); 
    while !board.is_won()? {
        let mut moves = board.possible_moves()?;
        let available_vehicles = moves.iter().map(
                |m| m.get_id_string()
            ).collect::<Vec<_>>();

        let chosen_vehicle: String = input()
            .repeat_msg("Vehicle to move: ")
            .err("Input parsing failed.")
            .add_err_test(move |x: &String| available_vehicles.contains(&x.to_uppercase()), 
                "This vehicle cannot move."
            ).get().to_uppercase();

        moves = moves.into_iter()
            .filter(|m| m.get_id_string() == chosen_vehicle)
            .collect::<Vec<_>>();

        if moves.len() == 1 {
            board.move_vehicle(moves.pop().unwrap());
        } else {
            let valid_distances: Vec<i8> = moves.iter()
                .map(|m| m.direction)
                .collect();

            let distance_input: i8 = input()
                .repeat_msg(format!("Distance to move (possible: {valid_distances:?}): "))
                .err("Input parsing failed.")
                .inside_err(
                    valid_distances, 
                    "This vehicle cannot move there."
                ).get();
            
            board.move_vehicle(moves.into_iter().find(
                |m| m.direction == distance_input).unwrap()
            )
        }
        
        score += 1;
        board.show(); 
    }

    println!("You solved the game!");
    board.export("results/solution.csv");
    Ok(score)
}