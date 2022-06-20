fn main() {
    let x: i32 = 100;
    solution(x);
}

fn solution(x: i32) -> i32 {
    if x > 214748367 {
        return 0;
    }
    let num = x.to_string();
    let negative = if x < 0 {
                       true
                   } else {
                         false
                     };
    let reverse: String = if negative == true {
                            num.chars().rev().collect()
                          } else {
                                num.chars().rev().collect()
                            };
    println!("{},{},{}", num, reverse, negative);
    0
}
