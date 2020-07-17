fn main() {
    // 控制流: if, while, loop, for

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is : {}", number);

    let mut counter = 0;

    // loop 表达式
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The rsult is : {}", result);

    let mut number = 3;

    while number != 0 {
        println!("number is : {}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("The element is: {}", element);
    }

    for number in (0..4).rev() {
        println!("number is: {}", number);
    }
}
