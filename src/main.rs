pub mod lib {
    pub mod array;
}

use lib::array::Array;

fn main() {
    let mut my_arr = Array::new();

    println!("Array is: {:?}", my_arr);

    my_arr.insert_at_end(0);
    my_arr.insert_at_start(2);
    my_arr.insert_by_index(1, 1);
    my_arr.insert_at_end(3);

    println!("Array is: {:?}", my_arr);

    my_arr.delete_by_index(0);
    my_arr.delete_at_end();
    my_arr.delete_at_start();

    println!("Array is: {:?}", my_arr);
}
