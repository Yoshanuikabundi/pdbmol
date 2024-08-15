#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_variables, unreachable_code,)
)]

use crate::bondset::BondSet;

use super::{CubicUnitCell, OrthogonalUnitCell, UnitCell};

impl OrthogonalUnitCell {
    pub fn tile_past(&self, target: impl UnitCell, points: &[[f32; 3]]) -> Vec<[f32; 3]> {
        if target.has_orientation() {
            todo!("get rotation from triclinic -> restricted triclinic");
        };

        let brick = target.to_brick();
        let [a, b, c] = self.to_vectors();
        let [a_target, b_target, c_target] = brick.to_vectors();

        use crate::geom::math_utils::norm;
        let tiled = self.tile_points(
            points,
            [
                (norm(a_target) / norm(a)).floor() as usize,
                (norm(b_target) / norm(b)).floor() as usize,
                (norm(c_target) / norm(c)).floor() as usize,
            ],
        );

        if target.has_orientation() {
            todo!("rotate the tiled points by inverse of rotation");
        };

        tiled
    }

    pub fn tile_to(
        &self,
        target: impl UnitCell,
        points: &[[f32; 3]],
        _bonds: BondSet<usize>,
    ) -> Vec<[f32; 3]> {
        if target.has_orientation() {
            todo!("get rotation from triclinic -> restricted triclinic");
        };

        let brick = target.to_brick();
        let _tiled = self.tile_past(brick, points);
        let _bonds = todo!("extend bonds according to tiling");

        todo!("trim points outside brick, and points bonded to them");

        if target.has_orientation() {
            todo!("rotate the tiled points by inverse of orientation");
        };

        _tiled
    }
}

impl CubicUnitCell {
    pub fn tile_past(&self, points: &[[f32; 3]], target: impl UnitCell) -> Vec<[f32; 3]> {
        OrthogonalUnitCell::from(*self).tile_past(target, points)
    }

    pub fn tile_to(
        &self,
        points: &[[f32; 3]],
        bonds: BondSet<usize>,
        target: impl UnitCell,
    ) -> Vec<[f32; 3]> {
        OrthogonalUnitCell::from(*self).tile_to(target, points, bonds)
    }
}
