    // Integer Types (Signed)
    const i8_example: i8 = -128;            // Range: -128 to 127
    const i16_example: i16 = 32767;         // Range: -32,768 to 32,767
    const i32_example: i32 = 2147483647;    // Range: -2,147,483,648 to 2,147,483,647
    const i64_example: i64 = 9223372036854775807;  // Range: -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807
    const i128_example: i128 = 170141183460469231731687303715884105727;  // Range: -(2^127) to 2^127-1
    const isize_example: isize = 42;        // Range: Depends on platform (32 or 64 bits)

    // Integer Types (Unsigned)
    const u8_example: u8 = 255;             // Range: 0 to 255
    const u16_example: u16 = 65535;         // Range: 0 to 65,535
    const u32_example: u32 = 4294967295;    // Range: 0 to 4,294,967,295
    const u64_example: u64 = 18446744073709551615;  // Range: 0 to 18,446,744,073,709,551,615
    const u128_example: u128 = 340282366920938463463374607431768211455;  // Range: 0 to 2^128-1
    const usize_example: usize = 42;        // Range: Depends on platform (32 or 64 bits)

    // Floating-Point Types
    const f32_example: f32 = 3.14;          // Range: ~-3.4E+38 to +3.4E+38
    const f64_example: f64 = 3.14159265359; // Range: ~-1.8E+308 to +1.8E+308

    // Boolean Type
    const bool_example: bool = true;        // Values: true or false

    // Character Type
    const char_example: char = 'A';         // Unicode scalar value (4 bytes) Range: 0x0000 to 0x10FFFF

    // Compound Types
    // Array (Fixed size, same type)
    const array_example: [i32; 5] = [1, 2, 3, 4, 5];

    // Tuple (Fixed size, can mix types)
    const tuple_example: (i32, f64, bool) = (42, 3.14, true);

    // Unit Type
    const unit_example: () = ();  

     // Printing examples to prevent unused variable warnings
     println!("Examples of Rust Data Types:");
     println!("i8: {}", i8_example);
     println!("i16: {}", i16_example);
     println!("i32: {}", i32_example);
     println!("i64: {}", i64_example);
     println!("i128: {}", i128_example);
     println!("isize: {}", isize_example);
     println!("u8: {}", u8_example);
     println!("u16: {}", u16_example);
     println!("u32: {}", u32_example);
     println!("u64: {}", u64_example);
     println!("u128: {}", u128_example);
     println!("usize: {}", usize_example);
     println!("f32: {}", f32_example);
     println!("f64: {}", f64_example);
     println!("bool: {}", bool_example);
     println!("char: {}", char_example);
     println!("array: {:?}", array_example);
     println!("tuple: {:?}", tuple_example);
     println!("unit: {:?}", unit_example);
 