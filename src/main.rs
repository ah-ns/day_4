fn main() {
    let eight_bit_signed: i8 = -128;
    println!("int8 signed: {}", eight_bit_signed);
    let eight_bit_unsigned: u8 = 255;
    println!("int8 unsigned (octal): {:o}", eight_bit_unsigned);

    let sixteen_bit_signed: i16 = -32_768;
    println!("int16 signed: {}", sixteen_bit_signed);
    let sixteen_bit_unsigned: u16 = 65_535;
    println!("int16 unsigned (binary): {:b}", sixteen_bit_unsigned);
    
    let thirty_two_bit_signed: i32 = -2_147_483_648;
    println!("int32 signed: {}", thirty_two_bit_signed);
    let thirty_two_bit_unsigned: u32 = 4_294_967_295;
    println!("int32 unsigned (hex): {:x}", thirty_two_bit_unsigned);
    
    let sixty_four_bit_signed: i64 = -9_223_372_036_854_775_808;
    println!("int64 signed: {}", sixty_four_bit_signed);
    let sixty_four_bit_unsigned: u64 = 18_446_744_073_709_551_615;
    println!("int64 unsigned: {}", sixty_four_bit_unsigned);

    let one_twenty_eight_bit_signed: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    println!("int128 signed: {}", one_twenty_eight_bit_signed);
    let one_twenty_eight_bit_unsigned: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    println!("int128 unsigned: {}", one_twenty_eight_bit_unsigned);

    let architecture_bit_signed: isize = -9_223_372_036_854_775_808;
    println!("int architecture signed: {}", architecture_bit_signed);
    let architecture_bit_unsigned: usize = 18_446_744_073_709_551_615;
    println!("int archiecture unsigned: {}", architecture_bit_unsigned);
    

    let thirty_two_bit_float: f32 = 2.82938208798789;
    println!("float32 {}", thirty_two_bit_float);

    let sixty_four_bit_float: f64 = 2.82938208798789;
    println!("float64 {}", sixty_four_bit_float);


    let boolean: bool = true;
    println!("boolean: {}", boolean);


    let character: char = 'η'; // Any unicode character
    println!("character: {}", character);


    let tuple: (i32, bool, char) = (23, false, 'A');
    let (a, b, c) = tuple;
    println!("tuple: {0}, {1}, {2}", a, tuple.1, c);


    let list: [i32; 5] = [0; 5];
    println!("list: {0} {1} {2}", list[0], list[1], list[2]);
}
