use num_traits::Float;

/// Defines the mathematical vector operations dot product and scaler
/// multiplication.
pub trait PointOps<T>
where
    T: Float,
{
    /// Scales a point by multiplying each component by a scaler (s), e.g.,
    ///   * For a 2D point s(x, y) would yield (s*x, s*y)
    ///   * For a 3D point s(x, y) would yield (s*x, s*y, s*y)
    fn scale(self: &Self, s: T) -> Self;

    /// Computes the dot product using the mathematical definition, e.g.,
    ///
    ///  (x1, y1) * (x2, y2) = (x1 * x2) + (y1 *y2)
    fn dot(self: &Self, rhs: &Self) -> T;
}
