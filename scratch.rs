## RUST
// Basic printing with newline (equivalent to Python's print())
println!("Hello, world!");

// Print with formatting (similar to Python's f-strings or .format())
let name = "Rust";
println!("Hello, {}!", name);

// Print without adding a newline at the end
print!("This doesn't add a newline");

// Print to standard error instead of standard output
eprintln!("This is an error message");

// Create an empty vector
let mut empty_vec: Vec<i32> = Vec::new();

// Create a vector with initial values
let mut numbers = vec![1, 2, 3, 4, 5];

// Adding elements
numbers.push(6);
numbers.push(7);

// Insert at specific position
numbers.insert(0, 0);  // Insert 0 at index 0

// Access elements
let third = numbers[2];  // Zero-indexed like Python
let maybe_value = numbers.get(10);  // Safe access that returns Option<&T>

// Remove elements
let last = numbers.pop();  // Removes and returns the last element
numbers.remove(0);  // Remove element at index 0

// Iterate over elements
for num in &numbers {
    println!("{}", num);
}

// Vector length
let length = numbers.len();

// Check if empty
if numbers.is_empty() {
    println!("Vector is empty");
}

// Clear all elements
// numbers.clear();

// Key differences from Python lists:

// Rust vectors are homogeneous (all elements must be the same type)
// You need to specify the type (Vec<T> where T is the element type)
// To modify a vector, it must be declared as mutable with mut
// Rust has both unchecked access (vec[i]) which will panic if out of bounds, and safe access (vec.get(i)) which returns an Option
// The vector operations tend to be more explicit than Python's list operations

// Vectors in Rust provide similar functionality to Python lists but with Rust's added safety guarantees around memory management and type safety.


In Rust, the equivalent of Python dictionaries are HashMap and BTreeMap from the standard library. HashMap is more similar to Python dictionaries in terms of performance characteristics (average O(1) lookups), while BTreeMap keeps keys sorted.
Here's how to create and use a HashMap in Rust:


use std::collections::HashMap;

fn main() {
    // Create an empty HashMap (key type: String, value type: i32)
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    // Insert key-value pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    // Create from tuples with collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 25];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    // Access values
    let blue_score = scores.get("Blue");  // Returns Option<&i32>
    
    // Access with default value if key doesn't exist
    let green_score = scores.get("Green").copied().unwrap_or(0);
    
    // Check if key exists
    if scores.contains_key("Yellow") {
        println!("Yellow team exists!");
    }
    
    // Update a value
    scores.insert(String::from("Blue"), 25);  // Overwrites existing value
    
    // Update only if key doesn't exist
    scores.entry(String::from("Green")).or_insert(50);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    // Iterate over key-value pairs
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Remove a key-value pair
    scores.remove("Blue");
    
    // Get the number of elements
    let size = scores.len();
    
    // Check if empty
    if scores.is_empty() {
        println!("HashMap is empty");
    }
}

Key differences from Python dictionaries:

You need to import HashMap with use std::collections::HashMap
You must specify both key and value types when creating a HashMap
String keys are typically String type rather than string literals (&str)
get() returns an Option<&V> rather than the value directly
Iteration patterns and mutability rules follow Rust's ownership system
The .entry() API provides more explicit control over insertion logic

If you need ordered keys (like Python's OrderedDict), use BTreeMap instead, which has a similar API but maintains keys in sorted order.

The main advantages of BTreeMap over HashMap:

Keys are always sorted (lexicographically for strings)
Supports ordered operations like first_key_value(), last_key_value(), and range()
Provides consistent iteration order
Offers logarithmic time complexity (O(log n)) for operations, which is slower than HashMap's average O(1) but has more predictable worst-case performance

Use BTreeMap when you need ordered keys or when you want to perform range queries on your map. Use HashMap when you prioritize raw lookup and insertion speed and don't care about key ordering.

use std::collections::BTreeMap;

fn main() {
    // Create an empty BTreeMap (key type: String, value type: i32)
    let mut scores: BTreeMap<String, i32> = BTreeMap::new();
    
    // Insert key-value pairs (will be stored in sorted key order)
    scores.insert(String::from("Zebra"), 10);
    scores.insert(String::from("Apple"), 50);
    scores.insert(String::from("Banana"), 25);
    
    // Create from tuples with collect
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 25];
    let mut ordered_scores: BTreeMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    // Access values
    let apple_score = scores.get("Apple");  // Returns Option<&i32>
    
    // Access with default value if key doesn't exist
    let missing_score = scores.get("Missing").copied().unwrap_or(0);
    
    // Check if key exists
    if scores.contains_key("Banana") {
        println!("Banana team exists!");
    }
    
    // Update a value
    scores.insert(String::from("Apple"), 55);  // Overwrites existing value
    
    // Update only if key doesn't exist
    scores.entry(String::from("Cherry")).or_insert(30);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut word_count = BTreeMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    // Iterate over key-value pairs (will be in sorted key order)
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Get first and last entries (not available in HashMap)
    if let Some((first_key, first_value)) = scores.first_key_value() {
        println!("First entry: {}: {}", first_key, first_value);
    }
    
    if let Some((last_key, last_value)) = scores.last_key_value() {
        println!("Last entry: {}: {}", last_key, last_value);
    }
    
    // Range operations (not available in HashMap)
    for (key, value) in scores.range("Apple".."Zebra") {
        println!("Range value: {}: {}", key, value);
    }
    
    // Remove a key-value pair
    scores.remove("Banana");
    
    // Get the number of elements
    let size = scores.len();
    
    // Check if empty
    if scores.is_empty() {
        println!("BTreeMap is empty");
    }
}

