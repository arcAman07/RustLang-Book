struct Point<T,U> {
    x: T,
    y: U,
}

impl <X1,Y1> Point<X1,Y1> {
    pub fn x(&self) -> &X1 {
        &self.x
    }

    pub fn mixup<X2,Y2>(self, other: Point<X2,Y2>) -> Point<X1,Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    
}


pub fn create_generic() {

}

pub fn get_largest(list : &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// pub fn largest_generic<T>(list : &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     return largest;
// }