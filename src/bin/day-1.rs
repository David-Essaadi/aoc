use std::path::Path;

fn main() {
    let contents = lib::read_file_contents(Path::new("inputs/input-1.txt"));
    solution_1(&contents);
    solution_2(&contents);
}

fn solution_1(contents: &String) {
    let max = contents
        .split("\n\n")    
        .map(get_total)
        .max()
        .unwrap_or(0);
    println!("solution 1 - max: {}", max)
}

fn solution_2(contents: &String) {
    let mut totals: Vec<i32> = contents        
        .split("\n\n")
        .map(get_total)
        .collect();
    totals.sort_unstable_by_key(|w| std::cmp::Reverse(*w));
    let total = totals.drain(0..3).reduce(|acc, curr| acc + curr).unwrap();
    println!("solution 2 - total top three: {}", total)
}

fn get_total(inventory: &str) -> i32 {
    inventory
        .split('\n')
        .map(|x| i32::from_str_radix(x, 10).unwrap_or(0))
        .reduce(|acc, curr| acc + curr)
        .unwrap()
}
