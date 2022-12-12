#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // // 1.
    // // let width1 = 30;
    // // let height1 = 50;

    // // 2.
    // // let rect1 = (30, 50);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // // println!("rect1 is {:#?}", rect1);

    // // println!(
    // //     "The area of the rectangle is {} square pixels",
    // //     // 1.
    // //     // area(width1, height1)

    // //     // 2.
    // //     // area(rect1)
    // //     area(&rect1)
    // // );

    // // 3.
    // // let scale = 2;
    // // let rect1 = Rectangle {
    // //     width: dbg!(30 * scale),
    // //     height: 50,
    // // };

    // // dbg!(&rect1);
    // println!(
    //     "The area of the rectangle is {} square pixels",
    //     rect1.area()
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);
    dbg!(&sq);
}

// 1.
// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }

// 2.
// fn area(dimensions: (i32, i32)) -> i32 {
//     dimensions.0 * dimensions.1
// }

// 3.
// fn area(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }
