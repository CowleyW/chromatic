use std::ops;

/// A struct for representing positions and directions. In essence, simply mathematical vectors
///
/// Each vector contains an x, y, and z value
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    /// Creates a new vector with the given x, y, and z values
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Returns the squared length of the vector
    ///
    /// The resulting value is equal to `x^2 + y^2 + z^2`
    pub fn len_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Returns the length of the vector
    ///
    /// The resulting value is equal to `sqrt(x^2 + y^2 + z^2)`. This is calculated by returning
    /// the square root of `self.len_squared`
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    /// Returns the dot product between this vector and the given vector
    ///
    /// This is the same as summing the product of the corresponding components from each vector
    pub fn dot(&self, rhs: Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Returns a normalized version of this vector
    ///
    /// A normalized vector has the same proportions and direction as the orignial vector, but has
    /// been scaled so that its length is equal to 1
    pub fn normalize(self) -> Vector3 {
        let len = self.len();

        let x = self.x / len;
        let y = self.y / len;
        let z = self.z / len;

        Vector3 { x, y, z }
    }
}

/// Overrides the addition operator for vectors
///
/// Returns a new vector of the same length by adding each of the corresponding components together
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        Vector3 { x, y, z }
    }
}

/// Overrides the subtraction operator for vectors
///
/// Returns a new vector of the same length by subtracting each of the right hand side components
/// from their corresponding left hand side component
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        Vector3 { x, y, z }
    }
}

/// Overrides the multiplication operator between a vector and a scalar
///
/// Returns a new vector where each component has been multiplied by the given scalar
impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Vector3 { x, y, z }
    }
}

/// Overrides the division operator between a vector and a scalar
///
/// Returns a new vector where each component has been divided by the given scalar
impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;

        Vector3 { x, y, z }
    }
}
