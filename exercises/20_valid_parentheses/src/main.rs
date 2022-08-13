fn main() {
    let s = String::from("]");
    println!("{}", solution(s));
}

fn check(qual: Parentheses, stack: &mut Vec<Parentheses>) -> bool {
    if stack.len() > 0 && qual == stack[stack.len() - 1] {
        stack.pop();
        true
    } else {
          false
      }
}

#[derive(PartialEq)]
enum Parentheses {
    Round,
    Square,
    Curly
}

fn solution(s: String) -> bool {
    let mut stack: Vec<Parentheses> = vec![];
    for item in s.chars() {
        match item {
            '(' => stack.push(Parentheses::Round),
            '[' => stack.push(Parentheses::Square),
            '{' => stack.push(Parentheses::Curly),
            _ => (),
        }
        let mut valid = match item {
                        ')' => check(Parentheses::Round, &mut stack),
                        ']' => check(Parentheses::Square, &mut stack),
                        '}' => check(Parentheses::Curly, &mut stack),
                        _ => true,
                    };
        if valid == false {
            return false
        }
    }
    if stack.len() == 0 {
        true
    } else {
          false
      }
}
