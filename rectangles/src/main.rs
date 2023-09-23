struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    // &self is alias for self: @Self
    // @Self is an alias for the type impl is for.

    fn new_square(size: u32) -> Self {
        Self {
            width : size,
            height : size
        }
    }

    fn area_mutable(&mut self, new_width : u32) -> u32 {
        self.width = new_width;
        self.width * self.height
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 80,
        height: 30
    };

    let square1 = Rectangle::new_square(30);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area_mutable(50)
    );
    println!(
        "The area of the square is {} square pixels.",
        square1.area()
    );
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );
}

// fn area(w: u32, h:u32) -> u32{
//     w * h
// }
