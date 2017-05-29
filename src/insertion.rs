use print_sorted;

pub fn insertion_sort(a: &mut [i32]) {
	print_sorted::print_array(&a);
	let mut temp: i32;
	
	//go through elements
	for i in 0..a.len() {
		//don't go out of array bounds
		if a.len() <= i+1 { break; }
	
		//if current element is greater than next element
		if a[i+1] < a[i] {
			//grab it so you know what it is
			temp = a[i+1];
			println!("{} is less than {}", temp, a[i]);
			//start from the beginning and find appropriate place to insert
			for j in (1..i+1).rev() {
				if temp < a[j] {
					a[j+1] = a[j];
					a[j] = temp;
				}
			}
		} else {
			println!("{} is bigger than {} so no swaps are necessary", a[i+1], a[i]);
		}
		print_sorted::print_array(&a);
	}
	
}	
