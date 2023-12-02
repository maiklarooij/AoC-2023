const MAX_RED: u16 = 12;
const MAX_GREEN: u16 = 13;
const MAX_BLUE: u16 = 14;

fn main() {

    let lines = include_str!("../input.txt").lines();
    let mut sum_of_valid_games = 0;

    for line in lines {

        // First, split the game ID from the game data
        let split = line.split(":").collect::<Vec<&str>>();
        let game_id = split[0].split(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let games = split[1].split(";").collect::<Vec<&str>>();

        // Keep track of valid games
        let mut valid_game = true;
        
        // Consider each game
        for game in games {

            // Keep track of the amount of each color in this game
            let (mut red, mut green, mut blue) = (0, 0, 0);
            let game_set = game.split(",").collect::<Vec<&str>>();
            
            // Consider each article in this game
            for article in game_set {
                let mut splitted_article = article.trim().split(" ");
                let amount: u16 = splitted_article.next().unwrap().parse().unwrap();
                let color: &str = splitted_article.next().unwrap().trim();
                
                // Add the amount of this article to the total amount of this color
                match color {
                    "red" => red += amount,
                    "green" => green += amount,
                    "blue" => blue += amount,
                    _ => panic!("Invalid color")
                }
            }
            
            // If any color exceeds the maximum amount, this game is invalid
            if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
                valid_game = false;
                break;
            }

        }

        // If this game is valid, add its ID to the sum
        if valid_game {
            sum_of_valid_games += game_id;
        }

    } 

    println!("Sum of valid games: {}", sum_of_valid_games);
}
