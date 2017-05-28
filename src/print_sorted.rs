pub fn print_array(a: & [i32]){
        for x in a.iter() {
                print!("{} ", x);
        }
        print!("\n");
}
