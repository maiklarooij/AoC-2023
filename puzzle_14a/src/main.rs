fn main() {
    let rocks = include_str!("../input.txt").lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let nr_cols = rocks[0].len();
    let nr_rows = rocks.len();
    let mut total_load = 0;

    for i in 0..nr_cols {
        
        // Tilted north: keep track of the last cube and the number of rounded cubes at the moment
        let mut last_cube = 0;
        let mut nr_rounded = 0;
        for j in 0..nr_rows {
            
            match rocks[j][i] {

                // Keep track of last cube where rocks will fall onto
                '#' => {
                    last_cube = j+1;
                    nr_rounded = 0;
                }
                '.' => continue,

                // Calculate the load for the current cube: last steady rock - round stones before
                'O' => {
                    total_load += nr_rows - last_cube - nr_rounded;
                    nr_rounded += 1;
                }
                _ => panic!("Invalid character")
            }
        }
    }
    println!("Total load: {}", total_load);
}
