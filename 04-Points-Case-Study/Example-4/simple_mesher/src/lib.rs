use rust_points::prelude::*;

#[derive(Debug)]
pub struct Triangle {
    pub vertex_0: Point2D<f64>,
    pub vertex_1: Point2D<f64>,
    pub vertex_2: Point2D<f64>,
}

impl Triangle {
    pub fn centroid(&self) -> Point2D<f64> {
        let pt_sum = (self.vertex_0 + self.vertex_1 + self.vertex_2);
        let centroid = pt_sum.scale(1_f64 / 3_f64);

        centroid
    }
}

pub fn square_to_triangles(square: &[Point2D<f64>; 4]) -> (Triangle, Triangle) {
    let tri_1 = Triangle {
        vertex_0: square[0],
        vertex_1: square[1],
        vertex_2: square[2],
    };

    let tri_2 = Triangle {
        vertex_0: square[2],
        vertex_1: square[3],
        vertex_2: square[0],
    };

    (tri_1, tri_2)
}
