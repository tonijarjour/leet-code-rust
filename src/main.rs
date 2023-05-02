//mod two_sum;
//mod roman_to_int;
//mod longest_common_prefix;
mod valid_parentheses;

fn main() {
    //println!("{:?}", two_sum::solve(vec![3, 2, 4], 6));
    //println!("{:?}", roman_to_int::solve("MCMXCIV".to_string()));
    //println!("{:?}", longest_common_prefix::solve(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]));
    println!(
        "{}",
        valid_parentheses::solve("({}){()}()[[()]{}]".to_string())
    );
}
