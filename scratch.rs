/* In Rust, there are several ways to convert a string to an integer. Here are the main approaches:
Using the parse() method The most common way is to use the parse() method which is available on string types: */

let my_string = "42";
let my_int: i32 = my_string.parse().unwrap();
// Or with type annotation in parse
let my_int = my_string.parse::<i32>().unwrap();

let my_string = "42";
match my_string.parse::<i32>() {
    Ok(num) => println!("Parsed number: {}", num),
    Err(_) => println!("Failed to parse number"),
}

// Or using Result's methods
let my_int = my_string.parse::<i32>().unwrap_or(0); // Default to 0 on error
let my_int = my_string.parse::<i32>().expect("Not a valid number"); // Panic with message

let my_string = "42";
let my_int = i32::from_str(my_string).unwrap();

// For decimal (base 10)
let my_int = i32::from_str_radix(my_string, 10).unwrap();

// For other bases (e.g., hexadecimal - base 16)
let hex_string = "2A";
let hex_value = i32::from_str_radix(hex_string, 16).unwrap(); // Gives 42






















// RUST
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
numbers.clear();

// Key differences from Python lists:
// Rust vectors are homogeneous (all elements must be the same type)
// You need to specify the type (Vec<T> where T is the element type)
// To modify a vector, it must be declared as mutable with mut
// Rust has both unchecked access (vec[i]) which will panic if out of bounds, and safe access (vec.get(i)) which returns an Option
// The vector operations tend to be more explicit than Python's list operations
// Vectors in Rust provide similar functionality to Python lists but with Rust's added safety guarantees around memory management and type safety.

// TUPLES - FIXED COLLECTIONS OF DIFFERENT TYPES
fn main() {
    
    let mixed_tuple = (42, 3.14, "hello", true);
    
    println!("Integer: {}", mixed_tuple.0);
    println!("Float: {}", mixed_tuple.1);
    println!("String: {}", mixed_tuple.2);
    println!("Boolean: {}", mixed_tuple.3);
}

// ENUM
enum Mixed {
    Integer(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
}

fn main() {
    let mixed_array = vec![
        Mixed::Integer(42),
        Mixed::Float(3.14),
        Mixed::Text(String::from("hello")),
        Mixed::Boolean(true),
    ];
    
    for item in &mixed_array {
        match item {
            Mixed::Integer(i) => println!("Integer: {}", i),
            Mixed::Float(f) => println!("Float: {}", f),
            Mixed::Text(s) => println!("Text: {}", s),
            Mixed::Boolean(b) => println!("Boolean: {}", b),
        }
    }
}

// RUST EQUIVALENT OF PYTHON type()
// 1. Using std::any::type_name (for debugging/development)
fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

fn main() {
    let x = 42;
    let y = "hello";
    let z = vec![1, 2, 3];
    
    println!("Type of x: {}", type_of(&x));  // Type of x: i32
    println!("Type of y: {}", type_of(&y));  // Type of y: &str
    println!("Type of z: {}", type_of(&z));  // Type of z: alloc::vec::Vec<i32>
}

// 2. Using std::any::Any trait (for runtime type checking)
use std::any::Any;

fn main() {
    let num = 42;
    
    // Check if num is an i32
    println!("Is i32: {}", (&num as &dyn Any).type_id() == std::any::TypeId::of::<i32>());
    
    // Check if num is a String
    println!("Is String: {}", (&num as &dyn Any).type_id() == std::any::TypeId::of::<String>());
}

// 3. Using pattern matching
fn what_type(value: &dyn Any) -> &'static str {
    if value.is::<i32>() {
        "i32"
    } else if value.is::<f64>() {
        "f64"
    } else if value.is::<String>() {
        "String"
    } else {
        "unknown type"
    }
}

fn main() {
    let a: Box<dyn Any> = Box::new(42);
    let b: Box<dyn Any> = Box::new(3.14);
    let c: Box<dyn Any> = Box::new(String::from("hello"));
    
    println!("a is {}", what_type(&*a));  // a is i32
    println!("b is {}", what_type(&*b));  // b is f64
    println!("c is {}", what_type(&*c));  // c is String
}

// RUST EQUIVALENT OF PYTHON len()
// For strings
let s = "hello";
let length = s.len(); // 5 bytes (note: this counts bytes, not characters)

// For string characters (grapheme clusters)
// Need to use an external crate like unicode-segmentation for true character count
let char_count = s.chars().count(); // 5 characters

// For vectors/arrays
let vec = vec![1, 2, 3, 4, 5];
let length = vec.len(); // 5

let array = [1, 2, 3, 4];
let length = array.len(); // 4

// For collections like HashMap and BTreeMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);
let length = map.len(); // 2

// For slices
let slice = &[1, 2, 3][..];
let length = slice.len(); // 3

// WHILE LOOPS
// PYTHON
count = 0
while count < 5:
    print(count)
    count += 1

// RUST
let mut count = 0;
while count < 5 {
    println!("{}", count);
    count += 1;
}

// PYTHON
while True:
    print("Infinite loop")
    break  # Exit the loop

// RUST
while true {
    println!("Infinite loop");
    break;  // Exit the loop
}

// RUST ONLY 
loop {
    println!("Infinite loop");
    break;  // Exit the loop
}

// PYTHON
count = 0
while count < 10:
    count += 1
    if count % 2 == 0:
        continue  # Skip even numbers
    if count > 7:
        break  # Exit when count > 7
    print(count)
    
// RUST
let mut count = 0;
while count < 10 {
    count += 1;
    if count % 2 == 0 {
        continue;  // Skip even numbers
    }
    if count > 7 {
        break;  // Exit when count > 7
    }
    println!("{}", count);
}    




// HASHMAPS
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

