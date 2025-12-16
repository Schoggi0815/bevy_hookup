pub trait Interpolate {
    fn interpolate(&self, other: &Self, percentage: f32) -> Self;
}
