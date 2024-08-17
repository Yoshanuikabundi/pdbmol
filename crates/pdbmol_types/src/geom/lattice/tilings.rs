#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_variables, unreachable_code,)
)]

use crate::bondset::BondSet;

use super::representations::{CubicUnitCell, OrthorhombicUnitCell};
use super::UnitCell;

impl OrthorhombicUnitCell {
    fn tile_past_brick(
        &self,
        brick: &[f32; 3],
        points: &[[f32; 3]],
    ) -> Vec<[f32; 3]> {
        let [x, y, z] = brick;
        let [a, b, c] = self.to_vectors();

        use crate::geom::math_utils::norm;
        self.tile_points(
            points,
            [
                (x / norm(a)).floor() as usize,
                (y / norm(b)).floor() as usize,
                (z / norm(c)).floor() as usize,
            ],
        )
    }

    /// Tile points in this unit cell to extend past the brick representation of the
    /// target lattice.
    pub fn tile_past(
        &self,
        target: impl UnitCell,
        points: &[[f32; 3]],
    ) -> Vec<[f32; 3]> {
        if target.has_orientation() {
            todo!("get rotation from triclinic -> restricted triclinic");
        };

        let tiled = self.tile_past_brick(&target.lattice().brick(), points);

        if target.has_orientation() {
            todo!("rotate the tiled points by inverse of rotation");
        };

        tiled
    }

    /// Tile points in this unit cell to exactly cover the brick representation
    /// of the target lattice.
    pub fn tile_to(
        &self,
        target: impl UnitCell,
        points: &[[f32; 3]],
        _bonds: BondSet<usize>,
    ) -> Vec<[f32; 3]> {
        if target.has_orientation() {
            todo!("get rotation from triclinic -> restricted triclinic");
        };

        let _tiled = self.tile_past_brick(&target.lattice().brick(), points);

        todo!("extend bonds according to tiling");
        todo!("trim points outside brick, and points bonded to them");

        if target.has_orientation() {
            todo!("rotate the tiled points by inverse of orientation");
        };

        _tiled
    }
}

impl CubicUnitCell {
    pub fn tile_past(
        &self,
        points: &[[f32; 3]],
        target: impl UnitCell,
    ) -> Vec<[f32; 3]> {
        OrthorhombicUnitCell::from(*self).tile_past(target, points)
    }

    pub fn tile_to(
        &self,
        points: &[[f32; 3]],
        bonds: BondSet<usize>,
        target: impl UnitCell,
    ) -> Vec<[f32; 3]> {
        OrthorhombicUnitCell::from(*self).tile_to(target, points, bonds)
    }
}
