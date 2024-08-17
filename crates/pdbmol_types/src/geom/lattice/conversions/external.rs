use super::super::representations::*;

impl From<[[f32; 3]; 3]> for OrientedTriclinicUnitCell {
    /// Unit cell vectors are an unambiguous representation of a general triclinic
    /// cell
    fn from(value: [[f32; 3]; 3]) -> Self {
        OrientedTriclinicUnitCell(value)
    }
}

impl From<OrientedTriclinicUnitCell> for [[f32; 3]; 3] {
    /// Unit cell vectors are an unambiguous representation of a general triclinic
    /// cell
    fn from(value: OrientedTriclinicUnitCell) -> Self {
        value.0
    }
}

mod composite {
    use super::*;

    impl TryFrom<[[f32; 3]; 3]> for TriclinicUnitCell {
        type Error = <OrientedTriclinicUnitCell as TryInto<TriclinicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            OrientedTriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<[[f32; 3]; 3]> for CrystallographicUnitCell {
        type Error = <OrientedTriclinicUnitCell as TryInto<CrystallographicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            OrientedTriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<[[f32; 3]; 3]> for OrthorhombicUnitCell {
        type Error = <OrientedTriclinicUnitCell as TryInto<OrthorhombicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            OrientedTriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<[[f32; 3]; 3]> for CubicUnitCell {
        type Error = <OrientedTriclinicUnitCell as TryInto<CubicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            OrientedTriclinicUnitCell::from(value).try_into()
        }
    }

    // Conversions to unit cell vectors
    // Unit cell vectors are an unambiguous representation of a general triclinic
    // cell

    impl From<TriclinicUnitCell> for [[f32; 3]; 3] {
        fn from(value: TriclinicUnitCell) -> Self {
            OrientedTriclinicUnitCell::from(value).into()
        }
    }

    impl From<CrystallographicUnitCell> for [[f32; 3]; 3] {
        fn from(value: CrystallographicUnitCell) -> Self {
            OrientedTriclinicUnitCell::from(value).into()
        }
    }

    impl From<OrthorhombicUnitCell> for [[f32; 3]; 3] {
        fn from(value: OrthorhombicUnitCell) -> Self {
            OrientedTriclinicUnitCell::from(value).into()
        }
    }

    impl From<CubicUnitCell> for [[f32; 3]; 3] {
        fn from(value: CubicUnitCell) -> Self {
            OrientedTriclinicUnitCell::from(value).into()
        }
    }
}
