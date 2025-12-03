use std::fs;

fn main() {
    println!("Part 1 Password: {}", part_1("input.txt"));

    println!("Part 2 Password: {}", part_2("input.txt"));
}

fn part_1(file: &str) -> i32 {
    let contents = fs::read_to_string(file)
        .expect("input.txt not found or invalid");
    
    let mut count = 0;
    let mut pos = 50;

    for line in contents.lines() {
        let direction = &line[0..1];
        let mov: i32 = str::parse(&line[1..])
            .expect("Number on line could not be parsed");
        
        match direction {
            "R" => pos += mov,
            "L" => pos -= mov,
            _ => panic!("Direction did not match L/R."),
        }

        pos %= 100;
        if pos == 0 {
            count += 1;
        }
    }

    return count;
}

fn part_2(file: &str) -> i32 {
    let contents = fs::read_to_string(file)
        .expect("input.txt not found or invalid");
    
    let mut count = 0;
    let mut pos = 50;
    
    for line in contents.lines() {
        let direction = &line[0..1];
        let mov: i32 = str::parse(&line[1..])
            .expect("Number on line could not be parsed");
        
        match direction {
            "R" => {
                let mut new_pos = pos + mov;
                println!("{line}: From {pos} to {new_pos} (Move by {mov})");
                while new_pos >= 100 {
                    count += 1;
                    new_pos -= 100;
                    println!("Count increased to {count}. new_pos changed to {new_pos}");
                }
                pos = new_pos;
            },
            "L" => {
                let mut new_pos = pos - mov;
                println!("{line}: From {pos} to {new_pos} (Move by {mov})");
                if pos == 0 && mov % 100 != 0 {
                    count -= 1;
                    println!("Starting from {pos} with mov ({mov}) not divisible by 100.\nCount reduced to {count} preemptively.");
                }
                
                while new_pos <= 0 {
                    count += 1;
                    new_pos += 100;
                    println!("Count increased to {count}. new_pos changed to {new_pos}");
                }
                pos = new_pos % 100;

            },
            _ => panic!("Direction did not match L/R."),
        }
        println!();
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 3);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 6);
    }
}
