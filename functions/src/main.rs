fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 2 // 此处不能加分号;加分号就不赋值.
        // 加分号就是语句,语句不反回值,表达式,不加分号,有值返回.
    };
    another_function(add(x, y), y);
}

// 无返回值的函数
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// 有返回值的函数
fn add(a: i32, b: i32) -> i32 {
    a + b // 这里不能加分号;

    // return a + b;
    //如果用了return语句,则可以加分号;
}