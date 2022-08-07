fn main() {
    let s = String::from("MCMXCIV");
    println!("{}", solution(s));
}

fn solution(s: String) -> i32 {
    use std::collections::HashMap;
    let mut symbols: HashMap<char, i32> = HashMap::new();
    symbols.insert('I', 1);
    symbols.insert('V', 5);
    symbols.insert('X', 10);
    symbols.insert('L', 50);
    symbols.insert('C', 100);
    symbols.insert('D', 500);
    symbols.insert('M', 1000);
    let mut array: Vec<i32> = vec![];
    for item in s.chars() {
        array.push(*symbols.get(&item).unwrap());
    }
    let mut idx = 0;
    let mut sum = 0;
    let len = array.len();
    while idx < len {
        if (idx + 1 < len && array[idx] < array[idx + 1]) {
            sum += array[idx + 1] - array[idx];
            idx += 2;
        } else {
            sum += array[idx];
            idx += 1;
          }
    }
    sum
}
