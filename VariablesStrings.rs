// Remember that Rust has two main string types:
// &str: A string slice (immutable, fixed size, borrowed reference)
// String: A growable, heap-allocated string (owned)
// The choice between them depends on whether you need ownership, mutability, or are just referencing string data.

// 1. String literals (string slices) - immutable, fixed-size
let string_slice = "Hello, world!";  // &str
let explicit_str: &str = "Hello, world!";

// 2. String objects - growable, heap-allocated
let string_object = String::new();  // Empty String
let string_object = String::from("Hello, world!");  // From a literal
let explicit_string: String = String::from("Hello, world!");

// 3. Creating Strings from string slices
let s = "Hello".to_string();
let s = "Hello".to_owned();

// 4. Creating Strings with capacity
let s = String::with_capacity(10);  // Preallocate space for 10 bytes

// 5. String from raw parts (unsafe)
let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

// 6. Converting other types to strings
let s = 42.to_string();  // From integer
let s = format!("The answer is {}", 42);  // Using format macro

// 7. String concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // Note: s1 is moved here, can't be used after

// 8. Char arrays to strings
let chars = ['H', 'e', 'l', 'l', 'o'];
let s: String = chars.iter().collect();

// 9. Mutable string variables
let mut mutable_str = String::from("Hello");
mutable_str.push_str(", world!");

// 10. Static strings (lifetime of the entire program)
static GLOBAL_STR: &str = "I'm a static string";

// 11. Byte strings
let byte_str = b"Hello";  // &[u8; 5]
let byte_string = String::from_utf8(vec![72, 101, 108, 108, 111]).unwrap();

// 12. Raw string literals (no escape sequences)
let raw_str = r"C:\Users\name";
let raw_str_with_quotes = r#"He said: "Hello""#;

// 13. Strings with constrained lifetimes
fn example<'a>(text: &'a str) {
    let borrowed_str: &'a str = text;
}

// 14. Box<str> - heap allocated string slice
let boxed_str: Box<str> = "Hello".into();

// 15. Using const for string literals
const GREETING: &str = "Hello, world!";

// 16. Cow (Clone-on-write) strings
use std::borrow::Cow;
let cow_str: Cow<'static, str> = Cow::Borrowed("Hello");
let cow_string: Cow<'static, str> = Cow::Owned(String::from("Hello"));
