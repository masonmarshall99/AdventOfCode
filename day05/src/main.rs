use std::fs;
use std::ops::Range;
use std::cmp::Ordering;

fn main() {
    println!("Part 1 - Fresh Ingredients: {}", part_1("input.txt"));

    println!("Part 2 - Number of IDs: {}", part_2("input.txt"));
}

fn part_1(file: &str) -> i64 {
    let contents = fs::read_to_string(file)
        .expect("File could not be read.");
    let halves: Vec<&str> = contents.split("\n\n")
        .collect();
    
    let fresh_numbers: Vec<Range<i64>> = halves[0]
        .lines()
        .map(|line| -> Vec<i64> {
            line.split("-")
            .map(|num|
                num.parse()
                    .expect("Number could not be parsed.")
            )
            .collect()
        })
        .map(|bounds| -> Range<i64> {
            Range { start: bounds[0], end: bounds[1]+1 }
        })
        .collect();
    //println!("{:?}", fresh_numbers);

    let ingredients: Vec<i64> = halves[1]
        .lines()
        .map(|line|
            line.parse()
                .expect("Number could not be parsed.")
        )
        .collect();
    //println!("{:?}", ingredients);

    let mut total = 0;
    
    for id in ingredients {
        let mut fresh = false;

        for set in &fresh_numbers {
            if set.contains(&id) {
                fresh = true;
            }
        }

        if fresh {
            total += 1;
        }
    }

    return total;
}

fn part_2(file: &str) -> i64 {
    let contents = fs::read_to_string(file)
        .expect("File could not be read.");
    let halves: Vec<&str> = contents.split("\n\n")
        .collect();
    
    let mut fresh_numbers: Vec<Range<i64>> = halves[0]
        .lines()
        .map(|line| -> Vec<i64> {
            line.split("-")
            .map(|num|
                num.parse()
                    .expect("Number could not be parsed.")
            )
            .collect()
        })
        .map(|bounds| -> Range<i64> {
            Range { start: bounds[0], end: bounds[1]+1 }
        })
        .collect();
    fresh_numbers.sort_by(|a, b| {
        if a.start < b.start {
            return Ordering::Less;
        }
        else if a.start == b.start {
            return Ordering::Equal;
        }
        else {
            return Ordering::Greater;
        }
    });

    //println!("{:?}", fresh_numbers);

    let mut range = 0..0;
    let mut non_overlap = Vec::new();

    for bounds in &fresh_numbers {
        if bounds.start < range.end {
            range.end = range.end.max(bounds.end);
        } else {
            non_overlap.push(range);
            range = bounds.start..bounds.end;
        }
    }
    non_overlap.push(range);

    //println!("{:?}", non_overlap);

    return non_overlap.iter().map(|r| r.end - r.start).sum();
     
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt"), 3);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 14);
    }
}
