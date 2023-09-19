fn main() {
    let contents = lib::read_file_contents("inputs/input-1.txt");
    let result_1 = solution_1(&contents);
    println!("solution 1 - max: {}", result_1);
    let result_2 = solution_2(&contents);
    println!("solution 2 - total top three: {}", result_2);
}

fn solution_1(contents: &String) -> i32 {
    contents
        .split("\n\n")    
        .map(get_total)
        .max()
        .unwrap_or(0)
}

fn solution_2(contents: &String) -> i32 {
    let mut totals: Vec<i32> = contents        
        .split("\n\n")
        .map(get_total)
        .collect();
    totals.sort_unstable_by_key(|w| std::cmp::Reverse(*w));
    totals.drain(0..3).reduce(|acc, curr| acc + curr).unwrap()
}

fn get_total(inventory: &str) -> i32 {
    inventory
        .split('\n')
        .map(|x| i32::from_str_radix(x, 10).unwrap_or(0))
        .reduce(|acc, curr| acc + curr)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use lib::read_file_contents;

    use super::*;

    #[test]
    fn test_solution_1() {
        let contents = read_file_contents("inputs/input-1.txt");
        assert_eq!(solution_1(&contents), 71780);
    }

    #[test]
    fn test_solution_2() {
        let contents = read_file_contents("inputs/input-1.txt");
        assert_eq!(solution_2(&contents), 212489);
    }
    
    #[test]
    fn test_total() {
        let inv = "100\n200\n300";
        assert_eq!(get_total(inv), 600)
    }
}
