#![feature(test)]

fn main() {
	let s1 = String::from("kelb");
	let s2 = String::from("kelb");
	println!("{}", fast_solution(s1, s2));
}

fn slow_solution(s1: String, s2: String) -> bool {
	let s1: Vec<char> = s1.chars().collect();
	let s2: Vec<char> = s2.chars().collect();
	if s1 == s2 {
		return true
	}
	let mut idx = 0;
	let mut errs: Vec<usize> = vec![];
	while idx < s1.len() {
		if s1[idx] != s2[idx] {
			errs.push(idx);
		}
		idx += 1;
	}
	if errs.len() == 2 && s1[errs[0]] == s2[errs[1]] && s1[errs[1]] == s2[errs[0]] {
		return true;
	}
	false
}

fn fast_solution(s1: String, s2: String) -> bool {
	let s1 = s1.as_bytes();
	let s2 = s2.as_bytes();
	if s1 == s2 {
		return true
	}
	let mut idx = 0;
	let mut errs: Vec<usize> = vec![];
	while idx < s1.len() {
		if s1[idx] != s2[idx] {
			errs.push(idx);
		}
		idx += 1;
	}
	if errs.len() == 2 && s1[errs[0]] == s2[errs[1]] && s1[errs[1]] == s2[errs[0]] {
		return true;
	}
	false
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn slow_bench(bench: &mut Bencher) {
    	bench.iter(|| {
	    	let s1 = String::from("adffjasdfjòòjasdfijjeawòlfjfjòaskldfjskfjò");
    		let s2 = String::from("adffjasdfjòòjasdfijjeawòlfjfjòaskldfjskfòj");
    		let _sol= slow_solution(s1, s2);
    	})
    }

    #[bench]
    fn fast_bench(bench: &mut Bencher) {
    	bench.iter(|| {
	    	let s1 = String::from("adffjasdfjòòjasdfijjeawòlfjfjòaskldfjskfjò");
    		let s2 = String::from("adffjasdfjòòjasdfijjeawòlfjfjòaskldfjskfòj");
    		let _sol = fast_solution(s1, s2);
    	})
    }
}