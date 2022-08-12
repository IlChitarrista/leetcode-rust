fn main() {
    let s = String::from("abcabcbb");
    println!("{}", solution(s));
}

fn solution(s: String) -> i32 {
    use std::collections::HashMap;
    let s = s.as_bytes();
    let mut idx = 0;
    let mut max = 0;
    while idx < s.len() {
        let mut temp: HashMap<char, usize> = HashMap::new();
        let mut int_idx = idx;
        let mut temp_max = 0;
        while int_idx < s.len() {
            let contained = s[int_idx] as char;
            let contained_idx = temp.get(&(s[int_idx] as char));
            if contained_idx != None {
                idx = match contained_idx {
                    Some(idx) => *idx,
                    None => panic!("The Program has been externally modified"),
                };
                break;
            } else {
                temp.insert(contained, int_idx);
                temp_max += 1;
                int_idx += 1;
              }
        }
        if temp_max > max {
            max = temp_max;
        }
        if s.len() - (idx + 1) <= max {
            return max as i32;
        }
        idx += 1;
    }
    max as i32
}
