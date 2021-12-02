use std::fs;

#[derive(Debug)]
struct Directions {
    direction: String,
    amount: i32,
}

fn main() {
    let contents = fs::read_to_string("./input.txt");
    let contents = match contents {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let lines = contents.split('\n');

    let directions: Vec<Directions> = lines
        .enumerate()
        .map(|(_, value)| value.split_whitespace())
        .map(|mut v| Directions { 
            direction: v.next().unwrap_or_default().to_string(), 
            amount: v.next().unwrap_or_default().parse::<i32>().unwrap_or_default(),
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for direction in directions.iter() {

        println!("{:?}", direction);

        match direction.direction.as_str() {
            "forward" => { 
                x += direction.amount;
                y += aim * direction.amount;
            },
            "back" => x -= direction.amount,
            "up" => aim -= direction.amount,
            "down" => aim += direction.amount,
            &_ => {},
        }

        println!("Current postion: {},{}", x, y);
    }
    
    println!("Answer = {}", x * y)
}
