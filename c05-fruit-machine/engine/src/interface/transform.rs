use ndarray::Array2;
use ndarray::arr2;

///A struct representing a position, rotation, and scale in 3d space
#[derive(Debug, Clone)]
pub struct Transform {
    pub matrix: Array2<f64>,
}

impl Transform {

    pub fn origin() -> Self {
        Transform::from_pos(0.0, 0.0, 0.0)
    }

    pub fn from_pos(x: f64, y: f64, z: f64) -> Self {
        Transform { matrix:  arr2(
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0]
            ])}
    }

    pub fn from_scale(x: f64, y: f64, z: f64) -> Self {
        Transform { matrix: arr2(
            &[
                [x, 0.0, 0.0, 0.0],
                [0.0, y, 0.0, 0.0],
                [0.0, 0.0, z, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        
        )}
    }

    pub fn from_euler(x: f64, y: f64, z: f64) -> Self {
        Transform { matrix: arr2(
            &[
                [1.0, 0.0, 0.0, 0.0],
                [0.0, x.cos(), -x.sin(), 0.0],
                [0.0, x.sin(), x.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]).dot(&arr2(
                &[
                    [y.cos(), 0.0, y.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-y.sin(), 0.0, y.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ]
            )).dot(&arr2(
                &[
                    [z.cos(), z.sin(), 0.0, 0.0],
                    [-z.sin(), z.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ]
            ))
        }
    }

    pub fn to_array(&self) -> [[f32; 4]; 4] {
        let slice = self.matrix.as_slice().unwrap();
        [
            [slice[0] as f32, slice[1] as f32, slice[2] as f32, slice[3] as f32],
            [slice[4] as f32, slice[5] as f32, slice[6] as f32, slice[7] as f32],
            [slice[8] as f32, slice[9] as f32, slice[10] as f32, slice[11] as f32],
            [slice[12] as f32, slice[13] as f32, slice[14] as f32, slice[15] as f32],
        ]
    }

    pub fn get_pos(&self) -> (f64, f64, f64) {
        (self.matrix.column(0)[3], self.matrix.column(1)[3], self.matrix.column(2)[3])
    }
}

impl std::ops::Mul for Transform {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Transform { matrix: self.matrix.dot(&rhs.matrix) }
    }
}