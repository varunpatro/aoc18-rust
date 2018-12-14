fn main() {
	let mut v = vec![3, 7];
	let mut i = 0;
	let mut j = 1;

	let puzzle_input = 637061;
	let pv = vec![6, 3, 7, 0, 6, 1];
	let mut reverse_pv = pv.clone();
	reverse_pv.reverse();

	fn check_last(v: &[usize], pv: &[usize]) -> bool {
		let l = pv.len();
		if v.len() < l {
			return false;
		} else if v.len() == l {
			return v.iter().rev().take(l).eq(pv.iter());
		} else {
			return v.iter().rev().skip(1).take(l).eq(pv.iter());
		}
	}

	while (puzzle_input + 10 > v.len())
			 || !check_last(&v, &reverse_pv) {
		let new_recipe = v[i] + v[j];
		if new_recipe < 10 {
			v.push(new_recipe);
		} else {
			v.push(new_recipe / 10);
			v.push(new_recipe % 10);
		}

		i = (v[i] + 1 + i) % v.len();
		j = (v[j] + 1 + j) % v.len();
	}

	let v_10 = &v[puzzle_input..puzzle_input+10];
	let v_10_s = v_10.iter().map(|i| i.to_string());
	let s = v_10_s.collect::<Vec<String>>().join("");
	let mut start = 0;
	loop {
		if v.iter().skip(start).take(pv.len()).eq(pv.iter()) {
			break;
		}

		start += 1;
	}

	println!("ten recipes: {:?}", s);
	println!("start: {}", start);

}