fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // String是可变类型，字符串字面量“”是不可变类型
    println!("{}", s);

    let x = "hello";
    let y = x; // 将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y
    println!("x : {}", x); // 栈上的字面量值不存在借用问题，直接拷贝
    println!("y : {}", y);

    let s1 = String::from("hello");
    println!("s1 : {}", s1);
    println!("&s1 : {:p}", &s1);
    let s2 = s1;
    // println!("s1 : {}", s1); s1值被借用了，没有所有权。无法使用。
    // println!("&s1 : {:p}", &s1); &s1值被借用了，没有所有权。无法使用。
    println!("s2 : {}", s2);
    println!("&s2 : {:p}", &s2);

    let s1 = String::from("world");
    let s2 = s1.clone(); // 堆数据的克隆，不涉及所有权转移
    println!("s1 : {}, s2 : {}", s1, s2);
    println!("&s1 : {:p}, &s2 : {:p}", &s1, &s2); // 指向了两个内存地址

    let s = String::from("Hello");
    takes_ownership(s);
    // println!("{}", s); s 值移动倒了函数里，这里的s没有所有权，无效。

    let x = 5;
    makes_copy(x);
    println!("{}", x); // i32是拷贝类型，这里可以继续使用x

    let s1 = gives_overship();
    let s2 = String::from("hello");
    let s2 = takes_and_gives_back(s2);
    println!("{}, {}", s1, s2);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer)
}

fn gives_overship() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}