use std::collections::HashMap;

fn main() {
    println!("Part 1 Product: {}", part_1("input.txt", 1000));

    println!("Part 2: {}", part_2("input.txt"));
}

fn part_1(file: &str, connections: usize) -> i64 {
    let contents = std::fs::read_to_string(file)
        .expect("Could not read file.");
    
    //println!("Read file.");

    let mut uf: HashMap<(i64, i64, i64), (i64, i64, i64)> = HashMap::new();

    // Points
    let mut points: Vec<(i64, i64, i64)> = Vec::new();
    for line in contents.lines() {
        let mut tokens = line.split(",");
        points.push(
            (tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap())      
        );
    }

    //println!("Ingested points.");

    for p in &points {
        uf.insert(p.clone(), p.clone());
    }
    

    // Distances
    let mut d_p2p: Vec<(i64, (i64, i64, i64), (i64, i64, i64))> = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let distance = (points[i].0-points[j].0).pow(2)
                + (points[i].1-points[j].1).pow(2)
                + (points[i].2-points[j].2).pow(2);
            d_p2p.push((distance, points[i], points[j]));
        }
    }
    d_p2p.sort_by(|a,b| a.0.cmp(&b.0));
    d_p2p = d_p2p[0..connections].to_vec();

    //println!("Calculated squared distances.");

    for (_dist, px, py) in d_p2p {
        let mut parent_x = &px;
        while parent_x != uf.get(parent_x).unwrap() {
            parent_x = uf.get(parent_x).unwrap();
        }
        let mut parent_y = &py;
        while parent_y != uf.get(parent_y).unwrap() {
            parent_y = uf.get(parent_y).unwrap();
        }
        uf.insert(parent_x.clone(), parent_y.clone());
    }

    //println!("Set up union find.");
    
    let mut sizes: HashMap<(i64, i64, i64), i64> = HashMap::new();
    for p in &points {
        sizes.insert(p.clone(), 0);
    }

    for p in &points {
        let mut parent = p;
        while parent != uf.get(parent).unwrap() {
            parent = uf.get(parent).unwrap();
        }
        let count = sizes.get(parent).unwrap();
        sizes.insert(parent.clone(), count+1);
    }

    let mut size_list: Vec<i64> = sizes.values().map(|a| a.clone()).collect();
    size_list.sort();
    size_list.reverse();

    //println!("Calculated sizes.");

    //println!("{:?}", uf);
    //println!("{:?}", size_list);

    return size_list[0..3].into_iter().product();
}

fn part_2(file: &str) -> i64 {
    let contents = std::fs::read_to_string(file)
        .expect("Could not read file.");
    
    //println!("Read file.");

    let mut uf: HashMap<(i64, i64, i64), (i64, i64, i64)> = HashMap::new();

    // Points
    let mut points: Vec<(i64, i64, i64)> = Vec::new();
    for line in contents.lines() {
        let mut tokens = line.split(",");
        points.push(
            (tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap(),
            tokens.next().unwrap().parse().unwrap())      
        );
    }

    //println!("Ingested points.");

    for p in &points {
        uf.insert(p.clone(), p.clone());
    }
    

    // Distances
    let mut d_p2p: Vec<(i64, (i64, i64, i64), (i64, i64, i64))> = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let distance = (points[i].0-points[j].0).pow(2)
                + (points[i].1-points[j].1).pow(2)
                + (points[i].2-points[j].2).pow(2);
            d_p2p.push((distance, points[i], points[j]));
        }
    }
    d_p2p.sort_by(|a,b| a.0.cmp(&b.0));
    
    //println!("Calculated squared distances.");

    let mut result = 0;
    for (_dist, px, py) in d_p2p {
        let mut parent_x = &px;
        while parent_x != uf.get(parent_x).unwrap() {
            parent_x = uf.get(parent_x).unwrap();
        }
        let mut parent_y = &py;
        while parent_y != uf.get(parent_y).unwrap() {
            parent_y = uf.get(parent_y).unwrap();
        }
        uf.insert(parent_x.clone(), parent_y.clone());

        let mut sizes: HashMap<(i64, i64, i64), i64> = HashMap::new();
        for p in &points {
            sizes.insert(p.clone(), 0);
        }

        for p in &points {
            let mut parent = p;
            while parent != uf.get(parent).unwrap() {
                parent = uf.get(parent).unwrap();
            }
            let count = sizes.get(parent).unwrap();
            sizes.insert(parent.clone(), count+1);
        }

        let mut size_list: Vec<i64> = sizes.values().map(|a| a.clone()).collect();
        size_list.sort();
        size_list.reverse();

        if size_list[0] == points.len() as i64 {
            result = px.0 * py.0;
            break;
        }

    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("test_input.txt", 10), 40);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("test_input.txt"), 25272);
    }
}
