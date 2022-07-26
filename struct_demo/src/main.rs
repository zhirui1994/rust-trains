fn main() {
    let user1 = build_user(
        String::from("some_name"),
        String::from("example@email.com")
    );

    let user2 = User {
        username: String::from("user2"),
        email: String::from("user2@email.com"),
        ..user1
    };

    println!("{:?}\n{:?}", user1, user2);

    // 元祖结构体
    #[derive(Debug)]
    struct Color(u8, u8, u8);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}, {:?}", black, origin);

    let rect = Rect {
        width: 30,
        height: 50,
    };

    println!("The erea of rectangle is {} squre pixels.", rect.area());
    println!("Rectangle is {:?}", rect);

    let s1:String = "s".into();
    let s2 = "s".to_string();
    let mut s3 = String::new();
    s3.insert_str(0, "s");

    println!("{}", s1 == s2);
    println!("{}", s2 == s3);

}

#[derive(Debug)] // 实现了Debug trait才能通过debug模式打印
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
}

