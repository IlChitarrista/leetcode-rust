fn main() {
    let s = String::from("aslkdfniawuhe wrhservòawnaròos eufòojfbwenr89abnp9wefnbdsaf n");
    println!("{}", solution(s));
    let s = String::from("aslkdfniawuhe wrhservòawnaròos eufòojfbwenr89abnp9wefnbdsaf n");
    println!("{}", best_solution(s));
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

fn best_solution(s: String) -> i32 {
    use std::collections::HashMap;
    let s = s.as_bytes();
    let (mut _l_idx, mut r_idx) = (0, 0);
    let (mut temp_max, mut max) = (0, 0);
    let mut temp: HashMap<char, usize> = HashMap::new();
    while r_idx < s.len() {
        let curr_val = temp.get(&(s[r_idx] as char));
        let curr = s[r_idx] as char;
        if curr_val == None {
            temp.insert(curr, r_idx);
            temp_max += 1;
            r_idx += 1;
        } else {
            temp.remove(&(s[_l_idx] as char));
            _l_idx += 1;
            if temp_max > max {
                max = temp_max;
            }
            temp_max = temp.keys().len();
          }
    }
    if temp_max > max {
        max = temp_max;
    }
    max as i32
}
