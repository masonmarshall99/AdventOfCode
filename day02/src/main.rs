use std::fs;

fn main() {
    println!("Part 1 Sum: {}", part_1("input.txt"));

    println!("Part 2 Sum: {}", part_2("input.txt"));
}

fn part_1(file: &str) -> u64 {
    let full_content = fs::read_to_string(file)
        .expect("{file} could not be read.");
    let content = full_content.trim();

    let mut sum = 0;
    
    for ids in content.split(",") {
        let bounds: Vec<u64> = ids.split("-")
            .map(|num| str::parse(num)
                .expect(&format!("Bound value {num} was not able to be parsed as a number.")))
            .collect();
        
        for id in bounds[0]..=bounds[1] {
            let id_str = id.to_string();
            
            let pivot = id_str.len()/2;
            let sequence = &id_str[0..pivot];
            let follower = &id_str[pivot..id_str.len()];

            
            if sequence == follower {
                sum += id;
                //println!("{id}: {sequence} = {follower}");
                //println!("Added {id} to sum");
            }
        }
    }

    return sum;
}

fn part_2(file: &str) -> u64 {
    let full_content = fs::read_to_string(file)
        .expect("{file} could not be read.");
    let content = full_content.trim();

    let mut sum = 0;
    
    for ids in content.split(",") {
        let bounds: Vec<u64> = ids.split("-")
            .map(|num| str::parse(num)
                .expect(&format!("Bound value {num} was not able to be parsed as a number.")))
            .collect();
        
        for id in bounds[0]..=bounds[1] {
            let id_str = id.to_string();
            let mut invalid = false;

            let lower = 0;
            for upper in 1..=id_str.len()/2 {
                let range = upper - lower;
                let slice = &id_str[lower..upper];

                if id_str.len() % range == 0 {
                    let mut mult_flag = true;
                    for mult in 1..id_str.len()/range {
                        let follower = &id_str[lower+range*mult..upper+range*mult];
                        if slice != follower {
                           mult_flag = false; 
                        }
                    }
                    if mult_flag {
                        invalid = true;
                    }
                }
            }

            if invalid {
                sum += id;
                //println!("Added {id} to sum");
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 1227775554);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 4174379265);
    }
}
