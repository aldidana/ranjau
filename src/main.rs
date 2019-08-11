mod mine;

use mine::MineField;
use std::io::stdin;

fn main() {
    let mut mine_field = MineField::new(3, 3);

    mine_field.generate_mines();

    let mut x = 0;
    let mut y = 0;

    println!("You have 5 tries");

    let mut counter = 1;

    mine_field.print_fields();

    loop {
        if counter == 5 {
            println!("You've won!");
            break;
        }

        println!("Enter coordinate of x: ");

        let mut input_x = String::new();
        stdin().read_line(&mut input_x)
            .ok()
            .expect("Error get x value");

        let trimmed_x = input_x.trim();

        match trimmed_x.parse::<u32>() {
            Ok(val) => x = val,
            Err(..) => println!("x is not an integer, {}", trimmed_x),
        };

        println!("Enter coordinate of y: ");

        let mut input_y = String::new();
        stdin().read_line(&mut input_y)
            .ok()
            .expect("Error get y value");

        let trimmed_y = input_y.trim();

        match trimmed_y.parse::<u32>() {
            Ok(val) => y = val,
            Err(..) => println!("y is not an integer, {}", trimmed_y),
        };

        if mine_field.find_by_coord(x, y) {
            println!("Game Over");
            break;
        }

        counter += 1;
    }

    mine_field.print_fields_solved();
}
