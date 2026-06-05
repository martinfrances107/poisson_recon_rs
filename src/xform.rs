use std::cmp::max;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use nalgebra::Const;
use nalgebra::Point;
use num_traits::Zero;

trait XformTrait<REAL> {
    fn identity() -> Self;
    fn coords(&self, i: usize, j: usize) -> REAL;
    fn coords_set(&mut self, i: usize, j: usize, value: REAL);
    fn determinant<const DIMMINUSONE: usize>(&self) -> REAL;
    fn inverse<const DIMMINUSONE: usize>(&self);
    fn sub_determinant<const DIMMINUSONE: usize>(&self, i: usize, j: usize) -> REAL;
}

#[derive(Debug)]
pub struct XForm<REAL, const DIM: usize>
where
    REAL: 'static + std::marker::Copy + std::fmt::Debug + PartialEq + num_traits::Zero,
{
    coords: [[REAL; DIM]; DIM],
    min: Point<REAL, DIM>,
    max: Point<REAL, DIM>,
}

impl<REAL, const DIM: usize> Default for XForm<REAL, DIM>
where
    REAL: 'static + Copy + std::default::Default + std::fmt::Debug + PartialEq + num_traits::Zero,
{
    fn default() -> Self {
        Self {
            coords: [[REAL::zero(); DIM]; DIM],
            min: Point::default(),
            // min: [REAL::default(); DIM],
            max: Point::default(),
        }
    }
}

impl<REAL, const DIM: usize> XformTrait<REAL> for XForm<REAL, DIM>
where
    REAL: std::fmt::Debug
        + Default
        + Copy
        + Add<REAL, Output = REAL>
        + AddAssign<REAL>
        + Div<REAL, Output = REAL>
        + From<i32>
        + Ord
        + PartialEq
        + Neg<Output = REAL>
        + Mul<REAL, Output = REAL>
        + MulAssign<REAL>
        + Sub<REAL, Output = REAL>
        + SubAssign<REAL>
        + Zero,
{
    fn identity() -> Self {
        let mut xform = Self::default();
        for d in 0..DIM {
            xform.coords_set(d, d, REAL::from(1));
        }
        xform
    }

    fn coords(&self, i: usize, j: usize) -> REAL {
        self.coords[i][j]
    }

    fn coords_set(&mut self, i: usize, j: usize, value: REAL) {
        self.coords[i][j] = value;
    }

    fn determinant<const DIMMINUSONE: usize>(&self) -> REAL {
        let mut det = REAL::zero();
        for d in 0..DIM {
            if d & 1 == 0 {
                det -= self.coords[d][0] * self.sub_determinant::<DIMMINUSONE>(d, 0);
            } else {
                det += self.coords[d][0] * self.sub_determinant::<DIMMINUSONE>(d, 0);
            }
        }
        det
    }
    fn inverse<const DIMMINUSONE: usize>(&self) {
        let mut xform = XForm::<REAL, DIM>::default();
        let d: REAL = self.determinant::<DIMMINUSONE>();
        for i in 0..DIM {
            for j in 0..DIM {
                if (i + j) % 2 == 0 {
                    xform.coords[j][i] = Self::determinant::<DIMMINUSONE>(&self) / d;
                } else {
                    xform.coords[j][i] = -Self::determinant::<DIMMINUSONE>(&self) / d;
                }
            }
        }
    }

    fn sub_determinant<const DIMMINUSONE: usize>(&self, i: usize, mut j: usize) -> REAL {
        let mut xform = XForm::<REAL, DIMMINUSONE>::default();
        let mut ii = [0usize; DIMMINUSONE];
        let mut jj = [0usize; DIMMINUSONE];
        let mut i = 0usize;
        let mut j = 0usize;
        for a in 0..DIM {
            if a != i {
                i += 1;
                ii[i] = a;
            }
            if (a != j) {
                j += 1;
                jj[j] = a;
            }
        }
        for i in 0..DIM - 1 {
            for j in 0..DIM - 1 {
                xform.coords_set(i, j, self.coords[ii[i]][jj[j]]);
            }
        }

        return xform.determinant::<DIMMINUSONE>();
    }
}
