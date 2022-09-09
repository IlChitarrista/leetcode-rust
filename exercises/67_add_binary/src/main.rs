#![feature(test)]

fn main() {
	let a = String::from("101101");
	let b = String::from("11001");
	println!("{}", slow_sol(a, b));
	let a = String::from("101101");
	let b = String::from("11001");
	println!("{}", add_binary(a, b));
}

fn slow_sol(a: String, b: String) -> String {
	let a: Vec<char> = a.chars().rev().collect();
	let b: Vec<char> = b.chars().rev().collect();
	let mut output: Vec<char> = Vec::new();
	let mut idx = 0;
	let mut surp: u8 = 0;
	// This while should be refactored into a separate function
	while a.get(idx) != None && b.get(idx) != None {
		let mut ones = 0;
		if *a.get(idx).unwrap() == '1' { ones += 1 }
		if *b.get(idx).unwrap() == '1' { ones += 1 }
		if surp == 1 { ones += 1 }
		match ones {
			1 => { output.push('1'); surp = 0; },
			2 => { output.push('0'); surp = 1; },
			3 => { output.push('1'); surp = 1; },
			_ => { output.push('0'); surp = 0; },
		}
		idx += 1;
	}
	if a.get(idx) != None {
		while a.get(idx) != None {
			let mut ones = 0;
			if *a.get(idx).unwrap() == '1' { ones += 1 }
			if surp == 1 { ones += 1 }
			match ones {
				1 => { output.push('1'); surp = 0; },
				2 => { output.push('0'); surp = 1; },
				_ => { output.push('0'); surp = 0; },
			}
			idx += 1;
		}
	} else {
		while b.get(idx) != None {
			let mut ones = 0;
			if *b.get(idx).unwrap() == '1' { ones += 1 }
			if surp == 1 { ones += 1 }
			match ones {
				1 => { output.push('1'); surp = 0; },
				2 => { output.push('0'); surp = 1; },
				_ => { output.push('0'); surp = 0; },
			}
			idx += 1;
		}
	  }
	if surp == 1 {
		output.push('1');
	}
	let output: String = output.into_iter().rev().collect();
	output
}

fn add_binary(a: String, b: String) -> String {
    let mut first: i128 = 0;
    let mut second: i128 = 0;
    let mut run: i128 = 1;

    for (i, c) in a.chars().rev().enumerate() {
        if i == 0 {
            run = 1;
        } else {
            run *= 2;
        }

        if c == '1' {
            first += run;
        }
    }

    for (i, c) in b.chars().rev().enumerate() {
        if i == 0 {
            run = 1;
        } else {
            run *= 2;
        }

        if c == '1' {
            second += run;
        }
    }
    format!("{:b}", first + second)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn slow_bench(bench: &mut Bencher) {
    	bench.iter(|| {
	    	let a = String::from("1");
    		let b = String::from("1");
    		let _sol= slow_sol(a, b);
    	})
    }

    #[bench]
    fn fast_bench(bench: &mut Bencher) {
    	bench.iter(|| {
	    	let a = String::from("1");
    		let b = String::from("1");
    		let _sol = add_binary(a, b);
    	})
    }
}