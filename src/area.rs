#![allow(non_snake_case)]

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn create_rect(&self, width:u32, height:u32) -> Rectangle {
        Rectangle {
            width : width,
            height : height,
        }
    }
    pub fn fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
}