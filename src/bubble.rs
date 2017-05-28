pub fn bubble_sort(a: &mut [i32]){
	println!("Original array:");
	print_array(&a);
	let mut sorted: bool = true;
	let mut temp: i32;

	while sorted {
		sorted = false;
		//go through elements of array
		for i in 0..a.len() {
			if a.len() <= i+1 { break; }

			//compare current element to next
			if a[i] > a[i+1] {
				println!("{} is less than {}", a[i+1], a[i]);
				sorted = true;
				//swap elements that are not in sorted order
				temp = a[i+1];
				a[i+1] = a[i];
				a[i] = temp;
			} else {
				println!("{} is bigger than {}, so no swaps are necessary", a[i+1], a[i])
			}
			print_array(&a);
		}
	} 
}

fn print_array(a: & [i32]){
	for x in a.iter() {
		print!("{} ", x);
	}
	print!("\n");
}
