trait Shape {
   fn area(&self)  -> f32;
}

#[derive(Debug)]
struct Circle {
    radius: f32
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius 
    }
}

#[derive(Debug)]
struct Triangle {
    x: f32,
    y: f32,
    z: f32
}

impl Shape for Triangle {
    fn area(&self)  -> f32 {
        let s:f32 = (self.x + self.y + self.z) / 2.0;
        (s * (s-self.x) * (s-self.y) * (s-self.z)).sqrt()
    }

}

#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32
}

impl Shape for Rectangle {
    fn area(&self)  -> f32 {
        self.x * self.y
    }
    
}


#[cfg(test)]
fn print_area<T>(shape: T) where T:Shape+std::fmt::Debug {
    println!("area of {:?} is: {}", shape, shape.area());
}


#[cfg(test)]
#[test]
fn shape_tests() {

    let circle = Circle {radius: 2.0};
    print_area(circle);

    let triangle1 = Triangle { x:3.0, y:5.0, z:4.0 };
    print_area(triangle1);

    let triangle2 = Triangle {x:6.0, y:6.0, z: 6.0};
    print_area(triangle2);

    let rectangle = Rectangle {x: 4.5, y: 4.0};
    print_area(rectangle);
}
