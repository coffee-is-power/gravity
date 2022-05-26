#[derive(Copy, Clone, Debug)]
pub struct Vector2(pub f64, pub f64);
impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl std::ops::AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl Vector2 {
    pub fn normalize(&self) -> Self {
        let len = self.len();
        Vector2(self.0 / len, self.1 / len)
    }
    pub fn len(&self) -> f64 {
        (self.0.powf(2.0) + self.1.powf(2.0)).sqrt()
    }
}
