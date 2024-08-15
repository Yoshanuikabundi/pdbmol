use std::collections::BTreeSet;

/// Stores bonds as pairs of atom identifiers
pub struct BondSet<T>(BTreeSet<(T, T)>);

impl<T> BondSet<T> {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord + Copy> BondSet<T> {
    /// Iterate over all bonds as pairs with the lesser serial first.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (T, T)> + 'a {
        self.0.iter().filter(|(a, b)| a < b).copied()
    }

    /// Iterate over all bonds as pairs with the lesser serial first.
    pub fn into_iter(self) -> impl Iterator<Item = (T, T)> {
        self.0.into_iter().filter(|(a, b)| a < b)
    }

    pub fn insert(&mut self, a: T, b: T) {
        self.0.insert((a, b));
        self.0.insert((b, a));
    }
}

impl<T> Default for BondSet<T> {
    fn default() -> Self {
        Self(BTreeSet::default())
    }
}

macro_rules! impl_bondset {
    ($($t:ty),+) => {
        $(impl BondSet<$t> {
            /// Iterate over atom serial numbers bonded to the given serial number.
            pub fn bonded_to<'a>(
                &'a self,
                serial: $t
            ) -> impl Iterator<Item = $t> + 'a {
                self.0
                    .range((serial, <$t>::MIN)..=(serial, <$t>::MAX))
                    .map(|(_, b)| *b)
            }
        })+
    }
}
impl_bondset!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);
