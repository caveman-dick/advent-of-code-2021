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

    for (idx, num) in numbers.iter().enumerate() {
        if idx == 0 {
            continue;
        }

        let previous_value = &numbers[idx-1];

        if previous_value < num {
            println!("{} is less than {}", previous_value, num);
            answer += 1;
        }
    }

    println!("The answer to part one is {}", answer);

    answer = 0;
    for (idx, num) in numbers.iter().enumerate() {
        if idx < 3 {
            continue;
        }

        let previous_value = numbers[idx-1] + numbers[idx-2] + numbers[idx-3];
        let current_value = numbers[idx] + numbers[idx-1] + numbers[idx-2];

        if previous_value < current_value {
            println!("{} is less than {}", previous_value, num);
            answer += 1;
        }
    }

    println!("The answer to part two is {}", answer);
}
