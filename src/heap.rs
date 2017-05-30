use print_sorted;

/* Heapsort as implemented in CLRS Chapter 6 */

fn left(i: i32) -> i32 {
	2*i + 1
}

fn right(i: i32) -> i32 {
	2*i + 2
}

fn max_heapify(a: &mut [i32], i: i32, heap_size: i32) {
	let mut largest: i32 = 0;
	let l = left(i);
	let r = right(i);
	let mut temp: i32;

	if l < heap_size && a[l as usize] > a[i as usize] {
			largest = l;
	}
	else {
		largest = i;
	}
	if r < heap_size && a[r as usize] > a[largest as usize] {
		largest = r;
	}
	if largest != i {
		temp = a[i as usize];
		a[i as usize] = a[largest as usize];
		a[largest as usize] = temp;
		max_heapify(a, largest, heap_size);
	}
}

fn build_max_heap(a: &mut [i32], heap_size: i32) {
	for i in (0..heap_size/2+1).rev() {
		max_heapify(a, i, heap_size);
	}
}

pub fn heap_sort(a: &mut [i32]) {

	print_sorted::print_array(&a);
	let mut heap_size = a.len() as i32;
	build_max_heap(a, heap_size);

	let mut temp: i32;

	for i in (1..heap_size as usize).rev() {
		temp = a[0];
		a[0] = a[i];
		a[i] = temp;
		heap_size = heap_size - 1;
		max_heapify(a, 0, heap_size);
	}	
	print_sorted::print_array(&a);
}

