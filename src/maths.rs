#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct vec2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl vec2 {
    // construct a new vec2
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    // calculate the length of a vec2
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // calculate the squared length of a vec2
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    // normalized vec2
    pub fn normalized(&self) -> vec2 {
        let length = self.length();
        vec2::new(self.x / length, self.y / length)
    }

    // dot product
    pub fn dot(a: vec2, b: vec2) -> f64 {
        a.x * b.x + a.y * b.y
    }
}

// implement the Add trait
impl std::ops::Add for vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
// implement the Add trait for scalar addition
impl std::ops::Add<f64> for vec2 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.x + other, self.y + other)
    }
}
// implement the Sub trait
impl std::ops::Sub for vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
// implement the Sub trait for scalar subtraction
impl std::ops::Sub<f64> for vec2 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self::new(self.x - other, self.y - other)
    }
}
// implemlement the Mul trait
impl std::ops::Mul for vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}
// implement the Mul trait for scalar multiplication
impl std::ops::Mul<f64> for vec2 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.x * other, self.y * other)
    }
}
// implement the Div trait
impl std::ops::Div for vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}
// implement the Div trait for scalar division
impl std::ops::Div<f64> for vec2 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.x / other, self.y / other)
    }
}

impl vec3 {
    // construct a new vec3
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    // calculate the cross product of two vec3s
    pub fn cross(&self, other: vec3) -> vec3 {
        vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // calculate the length of a vec3
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // calculate the squared length of a vec3
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // normalized vec3
    pub fn normalized(&self) -> vec3 {
        let length = self.length();
        vec3::new(self.x / length, self.y / length, self.z / length)
    }

    // dot product
    pub fn dot(a: vec3, b: vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn lerp(a: vec3, b: vec3, t: f64) -> vec3 {
        a * (1.0 - t) + b * t
    }
}

// implement the Add trait
impl std::ops::Add for vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
// implement the Add trait for scalar addition
impl std::ops::Add<f64> for vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}
// implement the Sub trait
impl std::ops::Sub for vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
// implement the Sub trait for scalar subtraction
impl std::ops::Sub<f64> for vec3 {
    type Output = Self;
    fn sub(self, other: f64) -> Self {
        Self::new(self.x - other, self.y - other, self.z - other)
    }
}
// implemlement the Mul trait
impl std::ops::Mul for vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}
// implement the Mul trait for scalar multiplication
impl std::ops::Mul<f64> for vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}
// implement the Div trait
impl std::ops::Div for vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}
// implement the Div trait for scalar division
impl std::ops::Div<f64> for vec3 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_new_instance() {
        let vec = vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x, 1.0);
        assert_eq!(vec.y, 2.0);
        assert_eq!(vec.z, 3.0);
    }

    #[test]
    fn it_adds_two_vectors() {
        let vec_1 = vec3::new(1.0, 2.0, 3.0);
        let vec_2 = vec3::new(4.0, 5.0, 6.0);
        let result = vec_1 + vec_2;
        assert_eq!(result, vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn it_subtracts_two_vectors() {
        let vec_1 = vec3::new(4.0, 5.0, 6.0);
        let vec_2 = vec3::new(1.0, 2.0, 3.0);
        let result = vec_1 - vec_2;
        assert_eq!(result, vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn it_multiplies_two_vectors() {
        let vec_1 = vec3::new(1.0, 2.0, 3.0);
        let vec_2 = vec3::new(4.0, 5.0, 6.0);
        let result = vec_1 * vec_2;
        assert_eq!(result, vec3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn it_divides_two_vectors() {
        let vec_1 = vec3::new(4.0, 10.0, 18.0);
        let vec_2 = vec3::new(4.0, 5.0, 6.0);
        let result = vec_1 / vec_2;
        assert_eq!(result, vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn it_adds_a_scalar_to_a_vector() {
        let vec = vec3::new(1.0, 2.0, 3.0);
        let result = vec + 1.0;
        assert_eq!(result, vec3::new(2.0, 3.0, 4.0));
    }

    #[test]
    fn it_subtracts_a_scalar_from_a_vector() {
        let vec = vec3::new(4.0, 5.0, 6.0);
        let result = vec - 1.0;
        assert_eq!(result, vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn it_multiplies_a_vector_by_a_scalar() {
        let vec = vec3::new(1.0, 2.0, 3.0);
        let result = vec * 2.0;
        assert_eq!(result, vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn it_divides_a_vector_by_a_scalar() {
        let vec = vec3::new(4.0, 10.0, 18.0);
        let result = vec / 2.0;
        assert_eq!(result, vec3::new(2.0, 5.0, 9.0));
    }

    #[test]
    fn it_calculates_the_cross_product() {
        let vec_1 = vec3::new(1.0, 0.0, 0.0);
        let vec_2 = vec3::new(0.0, 1.0, 0.0);
        let result = vec_1.cross(vec_2);
        assert_eq!(result, vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn it_calculates_the_length() {
        let vec = vec3::new(1.0, 2.0, 3.0);
        let result = vec.length();
        assert_eq!(result, 14.0_f64.sqrt());
    }

    #[test]
    fn it_calculates_the_squared_length() {
        let vec = vec3::new(1.0, 2.0, 3.0);
        let result = vec.length_squared();
        assert_eq!(result, 14.0);
    }

    #[test]
    fn it_normalizes_a_vector() {
        let vec = vec3::new(1.0, 2.0, 3.0);
        let result = vec.normalized();
        assert_eq!(
            result,
            vec3::new(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn it_creates_a_zero_vector() {
        let vec = vec3::zero();
        assert_eq!(vec, vec3::new(0.0, 0.0, 0.0));
    }
}
