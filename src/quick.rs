use print_sorted;

/* Quicksort as implemented in CLRS Chapter 7 */

fn partition(a: &mut [i32], p: i32, r: i32) -> i32 {
	let mut x = a[(r-1) as usize];
	let mut i = p - 1;
	let mut temp: i32;
	for j in p..r-1 {
		if a[j as usize] <= x {
			i = i + 1;
			temp = a[i as usize];
			a[i as usize] = a[j as usize];
			a[j as usize] = temp;
		}
	}
	temp = a[(i+1) as usize];
	a[(i+1) as usize] = a[(r-1) as usize];
	a[(r-1) as usize] = temp;
	return i + 1;
}

pub fn quick_sort(a: &mut [i32], p: i32, r: i32) {
	print_sorted::print_array(&a);
	let mut q: i32;
	if p < r {
		q = partition(a, p, r);
		quick_sort(a, p, q);
		quick_sort(a, q + 1, r);
	}
	print_sorted::print_array(&a);
}
