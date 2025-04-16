// 1. Basic integer types with type inference
let a = 10;  // i32 by default
let b = -5;  // i32 by default

// 2. Explicit type annotations
let a: i8 = 10;    // 8-bit signed integer (-128 to 127)
let b: i16 = 10;   // 16-bit signed integer (-32,768 to 32,767)
let c: i32 = 10;   // 32-bit signed integer (-2^31 to 2^31-1)
let d: i64 = 10;   // 64-bit signed integer (-2^63 to 2^63-1)
let e: i128 = 10;  // 128-bit signed integer (-2^127 to 2^127-1)
let f: isize = 10; // Pointer-sized signed integer

// 3. Unsigned integer types
let g: u8 = 10;    // 8-bit unsigned integer (0 to 255)
let h: u16 = 10;   // 16-bit unsigned integer (0 to 65,535)
let i: u32 = 10;   // 32-bit unsigned integer (0 to 2^32-1)
let j: u64 = 10;   // 64-bit unsigned integer (0 to 2^64-1)
let k: u128 = 10;  // 128-bit unsigned integer (0 to 2^128-1)
let l: usize = 10; // Pointer-sized unsigned integer

// 4. Integer literals with specific type suffixes
let m = 10i8;    // i8 type
let n = 10i16;   // i16 type
let o = 10i32;   // i32 type
let o = 10_i32;    // i32 type (underscore for readability)
let p = 10i64;   // i64 type
let q = 10i128;  // i128 type
let r = 10isize; // isize type
let s = 10u8;    // u8 type
let t = 10u16;   // u16 type
let u = 10u32;   // u32 type
let v = 10u64;   // u64 type
let p = 10_u64;    // u64 type (underscore for readability)
let w = 10u128;  // u128 type
let x = 10usize; // usize type

// 5. Different number bases
let decimal = 98_222;         // Decimal (base 10)
let hex = 0xff;               // Hexadecimal (base 16)
let octal = 0o77;             // Octal (base 8)
let binary = 0b1111_0000;     // Binary (base 2)
let byte = b'A';              // ASCII byte (u8 only)

// 6. Underscores for readability
let million = 1_000_000;      // Same as 1000000
let billion = 1_000_000_000;  // Underscores for readability

// 7. Mutable integers
let mut counter = 0;
counter += 1;

// 8. Constants
const MAX_POINTS: u32 = 100_000;

// 9. Static variables
static GLOBAL_COUNTER: i32 = 0;
static mut MUTABLE_COUNTER: i32 = 0;  // Requires unsafe to modify

// 10. Using arrays to store integers
let array_of_ints = [1, 2, 3, 4, 5];  // [i32; 5]
let explicit_array: [i32; 5] = [1, 2, 3, 4, 5];

// 11. Using tuples to store integers
let tuple_of_ints = (1, 2, 3);  // (i32, i32, i32)
let explicit_tuple: (i8, u16, i32) = (1, 2, 3);

// 12. Integer references
let num = 42;
let ref_to_num: &i32 = &num;

// 13. Option type
let some_int: Option<i32> = Some(42);
let no_int: Option<i32> = None;

// 14. Result type
let ok_result: Result<i32, &str> = Ok(42);
let err_result: Result<i32, &str> = Err("something went wrong");

// 15. Box for heap allocation
let boxed_int: Box<i32> = Box::new(42);

// 16. With generic types
struct Container<T> {
    value: T,
}
let int_container = Container { value: 42 };

// Mutable integers
let mut counter = 0;
counter += 1;

// Different Number Bases
let decimal = 98_222;       // Decimal (base 10)
let hex = 0xff;             // Hexadecimal (base 16)
let octal = 0o77;           // Octal (base 8)
let binary = 0b1111_0000;   // Binary (base 2)
let byte = b'A';            // ASCII byte (u8 only)

// Constants (compile-time evaluated)
const MAX_POINTS: u32 = 100_000;

// Static variables (program lifetime)
static GLOBAL_COUNTER: i32 = 0;

// Arrays of integers
let array = [1, 2, 3, 4, 5];  // [i32; 5]

// Tuples containing integers
let tuple = (1, 2, 3);  // (i32, i32, i32)

// Boxed integers (heap allocation)
let boxed = Box::new(42);  // Box<i32>

// Option type for optional integers
let some_int: Option<i32> = Some(42);
