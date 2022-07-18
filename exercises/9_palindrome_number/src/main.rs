fn main() {
    let x : i32 = 101;
    println!("{}", solution(x));
    println!("{}", solution2(x));
}

fn solution(x: i32) -> bool {
    if (x < 0) {
        return false;
    }

    let x = x.to_string();
    let reversed: String = x.chars().rev().collect();
    if x == reversed {
        return true;
    }
    false
}

//Best Solution from Leetcode

fn solution2(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut writeableNum = x;
    let mut reversedNum = 0;
    while writeableNum > 0 {
        reversedNum = writeableNum%10 + reversedNum*10;
        writeableNum /= 10;
    }
    reversedNum==x
}
