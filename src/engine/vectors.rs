/// Type for manipulating 2D vectors.
///
/// The type parameter T is the type of the coordinates.
///
/// - `Vector2<f32>` is [`Vector2f`]
/// - `Vector2<i32>` is [`Vector2i`]
///
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
}

/// [`Vector2`] with `f32` coordinates.
pub type Vector2f = Vector2<f32>;
/// [`Vector2`] with `i32` coordinates.
pub type Vector2i = Vector2<i32>;

impl<T> Vector2<T> {
    /// Creates a new vector from its coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: PartialOrd + Copy + Sized> Vector2<T> {
    /// Restrict the coordinates to a certain interval unless it is NaN.
    ///
    /// Returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise this returns `self`..
    ///
    /// # Panics
    ///
    /// Panics if `min > max`, `min` is NaN, or `max` is NaN.
    ///
    pub fn clamp(&self, x_min: T, x_max: T, y_min: T, y_max: T) -> Self {
        assert!(x_min <= x_max);
        assert!(y_min <= y_max);
        let mut x = self.x;
        let mut y = self.y;

        if x < x_min { x = x_min; }
        if x > x_max { x = x_max; }
        if y < y_min { y = y_min; }
        if y > y_max { y = y_max; }
        Self {
            x,
            y,
        }
    }
}

impl Vector2f {
    /// Rotate the vector with a given rotation.
    ///
    /// # Arguments
    /// * new_rotation - rotation to apply (Radian).
    ///
    pub fn rotate(&mut self, new_rotation: f32) {
        let old_x = self.x;

        self.x = self.x * new_rotation.cos() - self.y * new_rotation.sin();
        self.y = old_x * new_rotation.sin() + self.y * new_rotation.cos();
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f {
            x,
            y,
            z,
        }
    }
}