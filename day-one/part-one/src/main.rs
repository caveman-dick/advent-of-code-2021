use std::fs;

fn main() {
    let contents = fs::read_to_string("../input.txt");
    let contents = match contents {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let lines = contents.split('\n');

    let mut answer = 0;
    let mut previous_value = 0;
    for (idx, line) in lines.enumerate() {
        if line.is_empty() {
            continue;
        }

        let num= line.parse::<i32>();
        let num= match num {
            Ok(num) => num,
            Err(error) => {
                println!("Error parsing number '{}' - {:?}, skipping...", line, error);
                continue;
            }
        };

        if idx > 0 && previous_value < num {
            println!("{} is less than {}", previous_value, num);
            answer += 1;
        }

        previous_value = num;
    }
    println!("The answer is {}", answer);
}
