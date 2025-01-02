This example sequence will focus on a `Point` example at various stages of
design/development.


# Example 1


This example focuses on the start of a Point library. The foci will include:

  - A simple Point struct.

    ```rust
    pub struct Point2D {
        x: f64,
        y: f64,
    }
    ```

  - Implementing the `From` trait.

    ```rust
    impl From<(f64, f64)> for Point2D {
        fn from(tuple: (f64, f64)) -> Self {
            Point2D::new(tuple.0, tuple.1)
        }
    }
    ```

  - Initial testing through unit tests *(including the hamcrest2 crate, one of
    my two favourite crates)*.

    ```rust
        // Tolerance within which two numbers are considered equal
        const TOL_F64: f64 = 1e-8;

        #[test]
        fn test_origin_f64() {
            let origin = Point2D::new(0_f64, 0_f64);

            assert_that!(origin.get_x(), close_to(0_f64, TOL_F64));
            assert_that!(origin.get_y(), close_to(0_f64, TOL_F64));

            let dims = origin.get_dims();
            assert_that!(dims.0, close_to(0_f64, TOL_F64));
            assert_that!(dims.1, close_to(0_f64, TOL_F64));
        }
    }
    ```

# Example 2

This example focuses on the refinement of a Point library. The foci will include:

  - Defining a set of common operations with a `PointOps<T>` trait.

    ```rust
    pub trait PointOps<T>
    where
        T: Float,
    {
        fn scale(self: &Self, s: T) -> Self;

        fn dot(self: &Self, rhs: &Self) -> T;
    }
    ```

  - Using the `num_traits` crate, generics, and `trait` bounds to allow
    `Point2D` to use any `Float` type.

    ```rust
    use num_traits::Float;

    struct Point2D<T: Float> {
        x: T,
        y: T,
    }
    ```

  - Implementing the `Add` and `Sub` traits.

    ```rust
    impl<T> Add for Point2D<T>
    where
        T: Float + Add,
    {
        type Output = Point2D<T>;

        fn add(self, rhs: Self) -> Self {
            // Unit test caught copy and paste error (- vs +)
            Point2D::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    ```


# Example 3

This example focuses on the refinement of a Point library. The focus be
organizing the single `lib.rs` file into multiple files.


# Example 4

This example focuses on setting up a *Workspace* with the existing Point
library and a new (separate) binary crate.
