fn main() {
    // 加减运算
    println!("1 + 2 = {}", 1i32 + 2);
    println!("1 - 2 = {}", 1i32  - 2);

    // 逻辑运算
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 位运算
    println!("0011 AND 0101 is {:04b}", 0b_0011_u32 & 0b_0101);
    println!("0011 OR 0101 is {:04b}", 0b_0011_u32 | 0b_0101);
    println!("0011 XOR 0101 is {:04b}", 0b_0011_u32 ^ 0b_0101);
    println!("1 << 5 is {}", 1_u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x_80_u32 >> 2);

    println!("一百万可以写成{}", 100_0000_u32);
}