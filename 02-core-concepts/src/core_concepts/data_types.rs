fn print_basic_data_ypes() {
    // Integer Types (Signed)
    let i8_example: i8 = -128;            // Range: -128 to 127
    let i16_example: i16 = 32767;         // Range: -32,768 to 32,767
    let i32_example: i32 = 2147483647;    // Range: -2,147,483,648 to 2,147,483,647
    let i64_example: i64 = 9223372036854775807;  // Range: -9,223,372,036,854,775,807 to 9,223,372,036,854,775,807
    let i128_example: i128 = 170141183460469231731687303715884105727;  // Range: -(2^127) to 2^127-1
    let isize_example: isize = 42;        // Range: Depends on platform (32 or 64 bits)

    // Integer Types (Unsigned)
    let u8_example: u8 = 255;             // Range: 0 to 255
    let u16_example: u16 = 65535;         // Range: 0 to 65,535
    let u32_example: u32 = 4294967295;    // Range: 0 to 4,294,967,295
    let u64_example: u64 = 18446744073709551615;  // Range: 0 to 18,446,744,073,709,551,615
    let u128_example: u128 = 340282366920938463463374607431768211455;  // Range: 0 to 2^128-1
    let usize_example: usize = 42;        // Range: Depends on platform (32 or 64 bits)

    // Floating-Point Types
    let f32_example: f32 = 3.14;          // Range: ~-3.4E+38 to +3.4E+38
    let f64_example: f64 = 3.14159265359; // Range: ~-1.8E+308 to +1.8E+308

    // Boolean Type
    let bool_example: bool = true;        // Values: true or false

    // Character Type
    let char_example: char = 'A';         // Unicode scalar value (4 bytes)

    // Compound Types
    let array_example: [i32; 5] = [1, 2, 3, 4, 5];  // Fixed size array
    let tuple_example: (i32, f64, bool) = (42, 3.14, true);  // Tuple

    // String Types
    let string_example: String = String::from("Hello");  // Owned string
    let str_slice_example: &str = "Hello";               // String slice

    // Unit Type
    let unit_example: () = ();            // Empty tuple

    // Binary representations (prefix: 0b)
    let binary_num1 = 0b1111_0000;        // 240 in decimal
    let binary_num2 = 0b0000_1111;        // 15 in decimal
    let binary_num3 = 0b1010_1010;        // 170 in decimal

    // Hexadecimal representations (prefix: 0x)
    let hex_num1 = 0xFF;                  // 255 in decimal
    let hex_num2 = 0x1234_ABCD;          // 305,441,741 in decimal
    let hex_num3: i64 = 0xDEAD_BEEF;          // 3,735,928,559 in decimal

    // Octal representations (prefix: 0o)
    let octal_num1 = 0o77;               // 63 in decimal
    let octal_num2 = 0o777;              // 511 in decimal
    let octal_num3 = 0o7_777;            // 4,095 in decimal

    // Numbers with type suffixes
    let suffix_num1 = 42_i32;
    let suffix_num2 = 42_u8;
    let suffix_num3 = 3.14_f32;

    // Numbers with underscores for readability
    let readable_num1 = 1_000_000;        // One million
    let readable_num2 = 1_234_567.890_123;
    let readable_num3 = 1_234_567_890;    // One billion

    // Scientific notation
    let scientific_num1 = 1.23e2;         // 123.0
    let scientific_num2 = 1.23e-2;        // 0.0123
    let scientific_num3 = 1.23E+4;        // 12300.0


    println!("\nBasic Types Examples:");
    println!("Integers (Signed): i8={}, i16={}, i32={}", i8_example, i16_example, i32_example);
    println!("Integers (Unsigned): u8={}, u16={}, u32={}", u8_example, u16_example, u32_example);
    println!("Floating-point: f32={}, f64={}", f32_example, f64_example);
    println!("Boolean: {}", bool_example);
    println!("Character: {}", char_example);
    println!("Array: {:?}", array_example);
    println!("Tuple: {:?}", tuple_example);
    println!("String: {}", string_example);
    println!("Str slice: {}", str_slice_example);

    println!("\nNumber Representations:");
    println!("Binary (normal):    {}", binary_num1);
    println!("Binary (formatted): {:b}", binary_num1);
    println!("Binary (with 0b):   {:#b}", binary_num1);
    println!("Binary (padded):    {:08b}", binary_num2);

    println!("\nHexadecimal:");
    println!("Hex (lowercase):    {:x}", hex_num1);
    println!("Hex (uppercase):    {:X}", hex_num1);
    println!("Hex (with 0x):      {:#x}", hex_num1);
    println!("Hex (padded):       {:08X}", hex_num1);

    println!("\nOctal:");
    println!("Octal (normal):     {:o}", octal_num1);
    println!("Octal (with 0o):    {:#o}", octal_num1);

    println!("\nFormatting Options:");
    println!("Right-aligned:      {:>10}", readable_num1);
    println!("Left-aligned:       {:<10}", readable_num1);
    println!("Center-aligned:     {:^10}", readable_num1);
    println!("Zero-padded:        {:010}", readable_num1);
    println!("With sign:          {:+}", readable_num1);
    println!("Scientific (lower): {:e}", scientific_num1);
    println!("Scientific (upper): {:E}", scientific_num1);
    println!("Precision (3):      {:.3}", readable_num2);
    println!("With underscores:   {:?}", readable_num3);

    // Combined formatting
    println!("\nCombined Formatting Examples:");
    println!("Hex with padding:    {:#08x}", suffix_num1);
    println!("Binary with padding: {:#010b}", suffix_num1);
    println!("Right-align with precision: {:>10.3}", scientific_num1);

}