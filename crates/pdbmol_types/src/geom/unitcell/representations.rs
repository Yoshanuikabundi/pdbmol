// 9 parameter representations - 3 sizes, 3 tilts, 3 orientations
#[derive(Debug, Clone, Copy)]
pub struct TriclinicUnitCell(pub [[f32; 3]; 3]);

// 6 parameter representations - 3 sizes, 3 tilts
// A is aligned to x axis, B is in the xy plane, By and Cz are positive.
#[derive(Debug, Clone, Copy)]
pub struct CrystallographicUnitCell {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct RestrictedTriclinicUnitCell {
    pub size_parameters: [f32; 3],
    pub tilt_parameters: [f32; 3],
}

// 3 parameter representations - 3 sizes
//
// Unit cell is axis aligned and orthogonal.
#[derive(Debug, Clone, Copy)]
pub struct OrthogonalUnitCell {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// 1 parameter representations - 1 size
//
// Unit cell is axis aligned, orthogonal, and regular.
#[derive(Debug, Clone, Copy)]
pub struct CubicUnitCell(pub f32);
