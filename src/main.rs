mod dsts;
use dsts::{array::Array, queue::Queue, stack::Stack};

fn main() {
    let mut my_arr = Array::new();
    let mut my_stack = Stack::new();
    let mut my_queue = Queue::new();

    // ============ Testing array ============
    println!("\nCurrent Data Structure: Array");
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
    // =======================================

    // ============ Testing stack ============
    println!("\nCurrent Data Structure: Stack");
    println!("Stack is: {:?}", my_stack);

    my_stack.push(0);
    my_stack.push(1);
    my_stack.push(2);
    my_stack.push(3);

    println!("Stack is: {:?}", my_stack);

    my_stack.pop();
    my_stack.pop();

    println!("Stack is: {:?}", my_stack);

    println!("Stack is empty: {:?}", my_stack.is_empty());
    println!("Stack top element is: {:?}", my_stack.peek());

    my_stack.pop();
    my_stack.pop();

    println!("Stack is empty: {:?}", my_stack.is_empty());
    // =======================================

    // ============ Testing queue ============
    println!("\nCurrent Data Structure: Queue");
    println!("Queue is: {:?}", my_queue);

    my_queue.push(0);
    my_queue.push(1);
    my_queue.push(2);
    my_queue.push(3);
    my_queue.push(4);

    println!("Queue is: {:?}", my_queue);

    my_queue.pop();

    println!("Queue is: {:?}", my_queue);

    my_queue.pop();

    println!("Queue is: {:?}", my_queue);
    // =======================================
}
