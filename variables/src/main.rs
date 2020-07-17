use std::convert::TryInto;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 10_0000;
    
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let a = 5;
    let a = a + 1;
    let a = a * 2;

    println!("The value of a is: {}", a);

    let spaces = "        ";
    let spaces = spaces.len();

    println!("There is {} spaces!", spaces);

    // 变量遮蔽
    let x = 2.0; // 默认f64
    let x: f32 = 2.0; // f32
    let _x = 2; // 默认 i32
    let x: u8 = 2;
    let x: i8 = 2;
    let x: i16 = 0xff;
    let x: u16 = 0o77;
    let x: u32 = 98_222;
    let _x: u64 = 8888;
    let x: isize =  9223372036854775000;
    let mut x: usize = 18446744073709551000;

    x = _x.try_into().unwrap();

    println!("x: {}", x);

    // 加法
    let sum = 5 + 10;

    // 减法
    let sub: f64 = 95.5 - sum as f64;

    // 乘法
    let product = sub * 5 as f64;
    println!("poduct is: {}", product);

    // 除法
    let division: f64 = 1 as f64 / 3 as f64;

    println!("1 / 3 is: {}", division);

    // 取余
    let remainder = 45 % 5;

    // 布尔类型
    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{0} {1} {2}", c, z, heart_eyed_cat);


    /*复合类型 */
    // 元组: 圆括号声明, 成员类型可不同, 长度固定,可通过下脚标取值".0"
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let tup: (i32, f64, u8) = (x, y, z);

    println!("tup is : {:?}", tup); // 显示复合类型

    println!("x is : {:b}", tup.0); // binary显示模式

    // 数组: 方括号声明, 成员类型必须相同, 长度固定, 可通过下脚标取值"[0]
    let array = [1, 2, 3, 4, 5];
    let array: [i32; 5] =  [1, 2, 3, 4, 5];
    let months = [
        "January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"
    ];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let [x, y, z, w, e] = a;
    let x = months[0];
    println!("x is : {}", x);
}
