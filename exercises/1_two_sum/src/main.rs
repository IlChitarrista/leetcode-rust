fn main() {
    let nums = vec![1,3,5,7];
    let target = 6;

    println!("{:?}", two_sums(nums, target));
}

fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut found_stuff = HashMap::new();
    let mut idx = -1;
    for item in nums {
        idx += 1;
        let temp_targ = target - item;
        if found_stuff.contains_key(&temp_targ) {
            let mut solved_idx = 0;
            match found_stuff.get(&temp_targ) {
                Some(num) => solved_idx = *num,
                None => unimplemented!(),
            }
            return vec![idx, solved_idx]
        } else {
            found_stuff.insert(item, idx);
          }
    }
    vec![0, 0]
}
