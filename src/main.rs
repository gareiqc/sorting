mod print_sorted;
//mod bubble;
//mod insertion;
//mod heap;
mod quick;

fn main() {
        let a: [i32; 6] = [5, 17, 8, 2, 62, 13];
        //println!("Calling Bubble Sort");
        let mut sortme: [i32; 6] = [0; 6];
        for i in 0..a.len() {
                sortme[i] = a[i];
        }
        //bubble::bubble_sort(&mut sortme);
	//insertion::insertion_sort(&mut sortme);	
	//heap::heap_sort(&mut sortme);
	quick::quick_sort(&mut sortme, 0, (a.len() as i32));
}
