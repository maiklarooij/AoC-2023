use std::collections::HashMap;

fn main() {

    let max_colors: HashMap<&str, u16> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let lines = include_str!("../input.txt").lines();
    let mut sum_of_valid_games: u32 = 0;

    for line in lines {

        // First, split the game ID from the game data
        let split: Vec<&str> = line.split(":").collect::<Vec<&str>>();
        let game_id: u32 = split[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();
        let games: Vec<&str> = split[1].split(";").collect::<Vec<&str>>();

        // Keep track of valid games
        let mut valid_game: bool = true;
        
        // Consider each game
        for game in games {

            // Keep track of the amount of each color in this game
            let game_set: Vec<&str> = game.split(",").collect::<Vec<&str>>();
            
            // Consider each article in this game
            for article in game_set {
                let mut splitted_article = article.trim().split(" ");
                let amount: u16 = splitted_article.next().unwrap().parse().unwrap();
                let color: &str = splitted_article.next().unwrap().trim();

                if max_colors[color] < amount {
                    valid_game = false;
                    break;
                }
            }
        }

        // If this game is valid, add its ID to the sum
        if valid_game {
            sum_of_valid_games += game_id;
        }
    } 

    println!("Sum of valid games: {}", sum_of_valid_games);
}
