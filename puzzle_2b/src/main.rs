fn main() {

    let lines = include_str!("../input.txt").lines();
    let mut sum_of_powers: u32 = 0;

    for line in lines {

        // First, split the game ID from the game data
        let split: Vec<&str> = line.split(":")
            .collect::<Vec<&str>>();
        let games: Vec<&str> = split[1].split(";").collect::<Vec<&str>>();

        // Keep track of maximum number of colors needed for this game
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        
        // Consider each game
        for game in games {

            // Keep track of the amount of each color in this game
            let (mut red, mut green, mut blue) = (0, 0, 0);
            let game_set: Vec<&str> = game.split(",").collect::<Vec<&str>>();
            
            // Consider each article in this game
            for article in game_set {
                let splitted_article: Vec<&str> = article.trim().split(" ").collect::<Vec<&str>>();
                let amount: u32 = splitted_article[0].parse().unwrap();
                let color: &str = splitted_article[1].trim();
                
                // Add the amount of this article to the total amount of this color
                match color {
                    "red" => red += amount,
                    "green" => green += amount,
                    "blue" => blue += amount,
                    _ => panic!("Invalid color")
                }
            }
            
            // Keep track of maximum number of colors needed
            if red > max_red {
                max_red = red;
            }
            if green > max_green {
                max_green = green;
            }
            if blue > max_blue {
                max_blue = blue;
            }

        }

        // Calculate magic number
        let power: u32 = max_red *  max_green * max_blue;
        sum_of_powers += power;
    } 

    println!("Sum of powers: {}", sum_of_powers);
}
