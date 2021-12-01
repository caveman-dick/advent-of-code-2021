use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt");
    let contents = match contents {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let lines = contents.split('\n');

    let numbers: Vec<i32> = lines
        .enumerate()
        .map(|(_, value)| value.parse::<i32>().unwrap_or_default())
        .collect();

    let mut answer = 0;
    let mut previous_value = 0;

    for (idx, num) in numbers.iter().enumerate() {
        if idx > 0 && previous_value < *num {
            println!("{} is less than {}", previous_value, num);
            answer += 1;
        }

        previous_value = *num;
    }

    println!("The answer is {}", answer);
}
