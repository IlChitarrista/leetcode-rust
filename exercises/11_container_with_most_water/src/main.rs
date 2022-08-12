fn main() {
    let height = vec![2,3,4,5,18,17,6];
    println!("{}", solution(height));
}

fn solution(height: Vec<i32>) -> i32 {
    let (mut l_idx, mut r_idx) = (0, height.len() - 1);
    let mut result = 0;
    let mut dist = height.len() - 1;
    while l_idx != r_idx {
        let mut temp_result = min_num(height[l_idx], height[r_idx]) * dist as i32;
        if temp_result > result {
            result = temp_result;
        }
        if height[l_idx] < height[r_idx] {
            l_idx += 1;
            dist -= 1;
        } else {
              r_idx -= 1;
              dist -= 1;
          }
    }
    result
}

fn min_num(num1: i32, num2: i32) -> i32 {
    if num1 > num2 {
        return num2;
    } else {
          return num1;
      }
}

