fn main() {
    let mut strs = vec![];
    strs.push(String::from("flow"));
    strs.push(String::from("flower"));
    strs.push(String::from("flor"));
    println!("The result is {}", solution(strs));
}

fn solution(strs: Vec<String>) -> String {
    let (mut min_len, mut base_idx) = (strs[0].len(), 0);
    let mut int_idx = 0;
    for item in &strs {
        if item.len() < 1 {
            return String::new();
        } else if item.len() < min_len {
            base_idx = int_idx;
            min_len = item.len();
            int_idx += 1;
          } else {
                int_idx += 1;
            }
    }
    let (mut _curr, mut valid, mut output) = (strs[base_idx].as_bytes()[0] as char, true, String::new());
    let mut idx = 0;
    while valid && idx < strs[base_idx].len() {
        _curr = strs[base_idx].as_bytes()[idx] as char;
        for item in &strs {
            if item.as_bytes()[idx] as char == _curr {
            } else {
                valid = false;
              }
        }
        if valid == true {
            output.push(_curr);
        }
        idx += 1;
    }
    output
}

