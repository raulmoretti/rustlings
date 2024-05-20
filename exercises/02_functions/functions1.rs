// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


use std::fmt::Debug as Debug;
use std::any::type_name as var_type;
 

fn main() {
    // Primitives
    let x: bool = true;
    call_me(x);
    let x: char = 'A';
    call_me(x);
    let x: f32 = 7.7;
    call_me(x);
    let arr: [char; 2] = ['H', 'I'];
    call_me(arr); 
    let slice: &[char] = &arr[..1];
    call_me(slice);
    let tuple: (i32, f32, &str) = (3, 3.0, "three");
    call_me(tuple);
    let function = whatever_function();
    call_me(function); 
    let unit: () = ();
    call_me(unit);

}

fn call_me<T: Debug>(generic: T) {
    println!("Variable of type {}, and value is {:?}", 
        var_type::<T>(), generic
    );
}

fn whatever_function() -> String { "This is a function!".to_string() }