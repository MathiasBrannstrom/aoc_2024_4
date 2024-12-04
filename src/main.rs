fn main() {
    println!("Hello, world!");
    pt_two();
}

fn pt_one() {
    let data = include_str!("input.txt");
    let grid = parse_data(data);

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let search_string = "XMAS".chars().collect::<Vec<char>>();

    let mut found_strings = 0;

    for i in 0..grid_height {
        for j in 0..grid_width {
            if grid[i][j] != search_string[0] {
                continue;
            }

            for direction in directions {
                for u in 1..search_string.len() {
                    let y = i as i32 + (u as i32)* direction.0;
                    let x = j as i32 + (u as i32) * direction.1;
                    
                    if y < 0 || y >= (grid_height as i32) || x < 0 || x >= (grid_width as i32) {
                        break;
                    }

                    if grid[y as usize][x as usize] != search_string[u as usize] {
                        break;
                    }

                    if u == search_string.len() - 1 {
                        found_strings += 1;
                    }
                }
            }
        }
    }

    println!("Found {} strings", found_strings);
    
}

fn pt_two() {
    let data = include_str!("input.txt");
    let grid = parse_data(data);

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut found_strings = 0;

    for i in 1..grid_height-1 {
        for j in 1..grid_width-1 {
            if grid[i][j] != 'A' {
                continue;
            }

            let leg_one = (grid[i-1][j-1], grid[i+1][j+1]);

            if leg_one.0 != 'M' && leg_one.0 != 'S' {
                continue;
            }

            if (leg_one.0 == 'M' && leg_one.1 != 'S') || (leg_one.0 == 'S' && leg_one.1 != 'M') {
                continue;
            } 

            let leg_two = (grid[i-1][j+1], grid[i+1][j-1]);

            if leg_two.0 != 'M' && leg_two.0 != 'S' {
                continue;
            }

            if (leg_two.0 == 'M' && leg_two.1 != 'S') || (leg_two.0 == 'S' && leg_two.1 != 'M') {
                continue;
            } 

            found_strings += 1;
        }
    }

    println!("Found {} X-MASes", found_strings);
    
}



fn parse_data(data: &str) -> Vec<Vec<char>>
    {
    data.lines().map(|line| line.chars().collect()).collect()
}