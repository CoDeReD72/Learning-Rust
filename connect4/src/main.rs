fn main() {

    let mut grid = vec![vec![0; 7]; 6];

    while(true){

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
        grid[row][col] = 1;

        print_grid(&grid);
    }
}

fn print_grid(grid : &Vec<Vec<i32>>){
    for row in grid.iter() {
        for col in row.iter() {
            print!("{} ", col);
        }
        println!("");
    }
}