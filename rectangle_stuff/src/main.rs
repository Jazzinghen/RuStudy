mod rect;

fn main() {
    let rect_1 = rect::Rectangle::new(30, 50);
    let rect_2 = rect::Rectangle::new(10, 40);
    let rect_3 = rect::Rectangle::new(60, 45);

    println!("Rectangle data: {}", rect_1);
    println!("Rectangle area: {}", rect_1.area());

    println!("Can 1 hold 2? {}", rect_1.can_hold(&rect_2));
    println!("Can 1 hold 3? {}", rect_1.can_hold(&rect_3));
}
