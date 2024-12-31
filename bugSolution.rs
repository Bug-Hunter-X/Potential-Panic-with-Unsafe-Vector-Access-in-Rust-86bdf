fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; // This is correct
    println!("Second element: {}", second_element);

    let mut vec2 = Vec::with_capacity(2);
    vec2.resize(2, 0); // Initialize all elements to 0
    vec2[0] = 1;
    vec2[1] = 2; // Now this is safe
    let second_element2 = vec2[1];
    println!("Second element: {}", second_element2);
} 