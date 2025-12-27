pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    pub fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    pub fn static_circle() {
        println!("static_circle");
    }
}
