#![allow(non_snake_case)]

use crate::traits::Summary;
mod area;
mod vector;
mod string;
mod errors;
mod generics;
mod enums;
mod traits;
#[derive(Debug)]
struct User {
    username: String,
    age: u64,
    active: bool,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let s:String = String::from("Hello");
    let mut user = User {
        username: s,
        age: 20,
        active: true,
    };
    let user2 : User = build_user(String::from("Aman"), 20);
    let user3 : User = User {
        username: String::from("Aman"),
        ..user2
    };
    // println!("User: {:#?}", user3);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    let rect = area::Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = area::Rectangle::create_rect(&rect, 20, 60);
    println!("Area: {}", rect.area());
    println!("r1 fits r2 {}", rect.fit(&rect2));
    // vector::create_vector()
    // string::create_string()
    // errors::create_error()
    // generics::create_generic()
    let num_list = vec![34, 50, 25, 100, 65];
    println!("The largest num in num_list is {}",generics::get_largest(&num_list));
    let tweet = traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    }

fn build_user(username:String, age:u64) -> User {
    User {
        username,
        age,
        active: true,
    }
}


