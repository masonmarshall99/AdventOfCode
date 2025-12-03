use std::fs;

fn main() {
    println!("Sum: {}", part_1("input.txt"))
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
        
        for id in bounds[0]..bounds[1]+1 {
            let id_str = id.to_string();
            
            let pivot = id_str.len()/2;
            let sequence = &id_str[0..pivot];
            let follower = &id_str[pivot..id_str.len()];

            
            if sequence == follower {
                sum += id;
                println!("{id}: {pivot} = {follower}");
                println!("Added {id} to sum");
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
}
