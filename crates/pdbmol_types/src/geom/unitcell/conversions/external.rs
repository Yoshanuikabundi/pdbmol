use super::*;

impl From<[[f32; 3]; 3]> for TriclinicUnitCell {
    /// Unit cell vectors are an unambiguous representation of a general triclinic
    /// cell
    fn from(value: [[f32; 3]; 3]) -> Self {
        TriclinicUnitCell(value)
    }
}

impl From<TriclinicUnitCell> for [[f32; 3]; 3] {
    /// Unit cell vectors are an unambiguous representation of a general triclinic
    /// cell
    fn from(value: TriclinicUnitCell) -> Self {
        value.0
    }
}

mod composite {
    use super::*;

    impl TryFrom<[[f32; 3]; 3]> for RestrictedTriclinicUnitCell {
        type Error = <TriclinicUnitCell as TryInto<RestrictedTriclinicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            TriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<[[f32; 3]; 3]> for CrystallographicUnitCell {
        type Error = <TriclinicUnitCell as TryInto<CrystallographicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            TriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<[[f32; 3]; 3]> for OrthogonalUnitCell {
        type Error = <TriclinicUnitCell as TryInto<OrthogonalUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            TriclinicUnitCell::from(value).try_into()
        }
    }

    impl TryFrom<[[f32; 3]; 3]> for CubicUnitCell {
        type Error = <TriclinicUnitCell as TryInto<CubicUnitCell>>::Error;

        fn try_from(value: [[f32; 3]; 3]) -> Result<Self, Self::Error> {
            TriclinicUnitCell::from(value).try_into()
        }
    }

    // Conversions to unit cell vectors
    // Unit cell vectors are an unambiguous representation of a general triclinic
    // cell

    impl From<RestrictedTriclinicUnitCell> for [[f32; 3]; 3] {
        fn from(value: RestrictedTriclinicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }

    impl From<CrystallographicUnitCell> for [[f32; 3]; 3] {
        fn from(value: CrystallographicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }

    impl From<OrthogonalUnitCell> for [[f32; 3]; 3] {
        fn from(value: OrthogonalUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }

    impl From<CubicUnitCell> for [[f32; 3]; 3] {
        fn from(value: CubicUnitCell) -> Self {
            TriclinicUnitCell::from(value).into()
        }
    }
}
