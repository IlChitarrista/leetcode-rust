fn main() {
    let x : i32 = -121;
    println!("{}", solution(x));
}

fn solution(x: i32) -> bool {
    let x = x.to_string();
    let reversed: String = x.chars().rev().collect();
    if x == reversed {
        return true;
    }
    false
}
