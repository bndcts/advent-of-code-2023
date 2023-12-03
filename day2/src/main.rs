use std::fs;
use std::collections::HashMap;

const MAX_RED: isize = 12;
const MAX_GREEN: isize = 13;
const MAX_BLUE: isize = 14;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    let prepared_data = split_data(split_contents);
    first_puzzle(prepared_data);
}

fn first_puzzle(prepared_data: HashMap<isize, Vec<[isize; 3]>>) {
    let mut total: isize = 0;
    for (game_id, games) in prepared_data {
        total += game_id;
        for game in games {
            let red = game[0];
            let green = game[1];
            let blue = game[2];
            if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
                total -= game_id;
                break;
            }
        }
    }
    println!("Total:\n{total}");
}

fn split_data(split_contents: Vec<&str>) -> HashMap<isize, Vec<[isize; 3]>> {
    let mut map_of_games = HashMap::new(); //HashMap<isize, Vec<[i8,3]>> = HashMap::new();
    for line in split_contents {
        let game_id_split = line.split(":").collect::<Vec<&str>>();
        let game_id = game_id_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<isize>().expect("GameID should have been able to parse");
        let games = game_id_split[1].split(";").collect::<Vec<&str>>();
        let mut games_data: Vec<[isize; 3]> = Vec::new();
        for i in games {
            let split_game = i.split(",").collect::<Vec<&str>>();
            let mut game_data = [0, 0, 0]; // red, green, blue
            for j in split_game {
                let split_j = j.split(" ").collect::<Vec<&str>>();
                let mut index_of_color = 0;
                if j.contains("red") {
                    index_of_color = 0;
                } else if j.contains("green") {
                    index_of_color = 1;
                } else if j.contains("blue") {
                    index_of_color = 2;
                }
                game_data[index_of_color] = split_j[1].parse().unwrap();
            }
            games_data.push(game_data);
        }
        map_of_games.insert(game_id, games_data);
    }
    return map_of_games
}