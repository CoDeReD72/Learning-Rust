fn main() {

    let mut grid = vec![vec![0; 7]; 6];

    let mut player1: bool = true;


    loop{


        print_grid(&grid);

        println!("Enter a column number: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut col: usize = input.trim().parse().unwrap();
        
        col -= 1; // Avoid 0-indexing

        let mut row = 5;
        while grid[row][col] != 0 {
            row -= 1;
        }

        if player1 == true{
            grid[row][col] = 1;
            player1 = false;
        } else {
            grid[row][col] = 2;
            player1 = true;
        }

        print_grid(&grid);

        check_win(&grid);
    }
}

fn print_grid(grid : &Vec<Vec<i32>>){
    for row in grid.iter() {
        for col in row.iter() {
            print!("{} ", col);
        }
        println!("");
    }
    println!("\n");
}

fn check_win(grid : &Vec<Vec<i32>>) -> bool{
    // Check horizontal
    for row in grid.iter() {
        let mut count = 0;
        for col in row.iter() {
            if *col == 1 {
                count += 1;
            } else {
                count = 0;
            }
            if count == 4 {
                println!("Horizontal win");
                return true;
            }
        }
    }

    // Check vertical

    for col in 0..7 {
        let mut count = 0;
        for row in 0..6 {
            if grid[row][col] == 1 {
                count += 1;
            } else {
                count = 0;
            }
            if count == 4 {
                println!("Vertical win");
                return true;
            }
        }
    }

    // Check diagonal


    return false; // Temp
}