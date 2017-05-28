mod bubble;

fn main() {
        let a: [i32; 6] = [5, 17, 8, 2, 62, 13];
        println!("Calling Bubble Sort");
        let mut sortme: [i32; 6] = [0; 6];
        for i in 0..a.len() {
                sortme[i] = a[i];
        }
        bubble::bubble_sort(&mut sortme);
}

