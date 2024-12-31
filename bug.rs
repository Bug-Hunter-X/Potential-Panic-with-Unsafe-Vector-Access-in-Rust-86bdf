fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; // This is correct
    println!("Second element: {}", second_element);

    let mut vec2 = Vec::with_capacity(2);
    unsafe {
        vec2.set_len(2);
    }
    vec2[0] = 1;
    vec2[1] = 2; // This might panic!
    let second_element2 = vec2[1]; //This will panic if vec2 is not initialized
    println!("Second element: {}", second_element2);
}