fn vec_enum_poly() {
    struct Circle;
    struct Triangle;
    struct Rectangle;

    trait Shape {
        fn area(&self) -> f64;
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            println!("circle");
            0.0
        }
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            println!("rectangle");
            0.0
        }
    }

    impl Shape for Triangle {
        fn area(&self) -> f64 {
            println!("triangle");
            0.0
        }
    }

    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

    let rectangle = Rectangle;
    let circle = Circle;
    let triangle = Triangle;

    shapes.push(Box::new(rectangle));
    shapes.push(Box::new(circle));
    shapes.push(Box::new(triangle));

    for s in shapes {
        s.area();
    }
}

fn vec_trait_object_poly() {
    struct Circle;
    struct Triangle;
    struct Rectangle;

    impl Circle {
        fn area() -> f64 {
            println!("circle");
            0.0
        }
    }

    impl Rectangle {
        fn area() -> f64 {
            println!("rectangle");
            0.0
        }
    }

    impl Triangle {
        fn area() -> f64 {
            println!("triangle");
            0.0
        }
    }

    enum Shape {
        Circle,
        Rectangle,
        Triangle,
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Triangle {} => Triangle::area(),
                Shape::Circle {} => Circle::area(),
                Shape::Rectangle {} => Rectangle::area(),
            }
        }
    }

    let mut shapes: Vec<&Shape> = Vec::new();

    let rectangle = Shape::Rectangle;
    let circle = Shape::Circle;
    let triangle = Shape::Triangle;

    shapes.push(&rectangle);
    shapes.push(&circle);
    shapes.push(&triangle);

    for x in shapes {
        x.area();
    }
}

fn main() {
    vec_enum_poly();
    vec_trait_object_poly();
}
