fn main() {
    let x: i32 = -100;
    println!("{}", solution(x));
}

fn solution(x: i32) -> i32 {
    let mut x_array = vec![];
	let mut negative = false;
	for digit in x.to_string().chars().rev() {
		x_array.push(digit.to_digit(10));
	}
	if x_array[x_array.len()-1] == None {
		x_array.pop();
		negative = true;
	}
	let mut idx = 0;
	let mut initial = false;
	let mut solution = vec![];
	if x_array.len() <= 1 && x_array[0].unwrap() == 0 {
		return 0;
	}
	while idx < x_array.len() {
		while initial == false {
			if x_array[idx].unwrap() != 0 {
				initial = true;
				solution.push(x_array[idx].unwrap());
				idx += 1;
			} else {
			  	idx += 1;
			  }
		}
		if idx < x_array.len() {
			solution.push(x_array[idx].unwrap());
			idx += 1;
		}
	}

	if x_array.len() >= 10 {
		let mut solution_check = solution.clone();
		solution_check.pop();
		let mut solution_check = solution_check.iter().fold(0, |acc, elem| acc * 10 + elem) as i32;
		if solution_check > 214748364 { //Approximation
			return 0;
		}
	}

	let mut solution = solution.iter().fold(0, |acc, elem| acc * 10 + elem) as i32;
	if negative == true {
		solution = solution * -1;
	}
	solution
}
