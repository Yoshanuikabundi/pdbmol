use bounded_static::ToStatic;
use strum::{EnumIter, EnumString, FromRepr, IntoStaticStr};

/// A chemical element.
///
/// All elements' variant names are the element's name, and the discriminant is
/// the atomic number.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, FromRepr, IntoStaticStr, EnumString, EnumIter, ToStatic,
)]
#[repr(u8)]
#[non_exhaustive]
pub enum Element {
    Hydrogen = 1,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Sodium,
    Magnesium,
    Aluminium,
    Silicon,
    Phosphorus,
    Sulfur,
    Chlorine,
    Argon,
    Potassium,
    Calcium,
    Scandium,
    Titanium,
    Vanadium,
    Chromium,
    Manganese,
    Iron,
    Cobalt,
    Nickel,
    Copper,
    Zinc,
    Gallium,
    Germanium,
    Arsenic,
    Selenium,
    Bromine,
    Krypton,
    Rubidium,
    Strontium,
    Yttrium,
    Zirconium,
    Niobium,
    Molybdenum,
    Technetium,
    Ruthenium,
    Rhodium,
    Palladium,
    Silver,
    Cadmium,
    Indium,
    Tin,
    Antimony,
    Tellurium,
    Iodine,
    Xenon,
    Caesium,
    Barium,
    Lanthanum,
    Cerium,
    Praseodymium,
    Neodymium,
    Promethium,
    Samarium,
    Europium,
    Gadolinium,
    Terbium,
    Dysprosium,
    Holmium,
    Erbium,
    Thulium,
    Ytterbium,
    Lutetium,
    Hafnium,
    Tantalum,
    Tungsten,
    Rhenium,
    Osmium,
    Iridium,
    Platinum,
    Gold,
    Mercury,
    Thallium,
    Lead,
    Bismuth,
    Polonium,
    Astatine,
    Radon,
    Francium,
    Radium,
    Actinium,
    Thorium,
    Protactinium,
    Uranium,
    Neptunium,
    Plutonium,
    Americium,
    Curium,
    Berkelium,
    Californium,
    Einsteinium,
    Fermium,
    Mendelevium,
    Nobelium,
    Lawrencium,
    Rutherfordium,
    Dubnium,
    Seaborgium,
    Bohrium,
    Hassium,
    Meitnerium,
    Darmstadtium,
    Roentgenium,
    Copernicium,
    Nihonium,
    Flerovium,
    Moscovium,
    Livermorium,
    Tennessine,
    Oganesson,
}

impl Element {
    const SYMBOLS: [&'static str; 118] = [
        "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S",
        "Cl", "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga",
        "Ge", "As", "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd",
        "Ag", "Cd", "In", "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm",
        "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os",
        "Ir", "Pt", "Au", "Hg", "Tl", "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa",
        "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr", "R", "Db", "Sg",
        "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh", "Fl", "Mc", "Lv", "Ts", "Og",
    ];

    /// Atomic masses of the elements in daltons (or unified atomic mass units)
    ///
    /// From <https://pubchem.ncbi.nlm.nih.gov/ptable/atomic-mass/>.
    const MASSES: [f32; 118] = [
        1.0080, 4.00260, 7.0, 9.012183, 10.81, 12.011, 14.007, 15.999, 18.998403, 20.180,
        22.989769, 24.305, 26.981538, 28.085, 30.973762, 32.07, 35.45, 39.9, 39.0983, 40.08,
        44.95591, 47.867, 50.9415, 51.996, 54.93804, 55.84, 58.93319, 58.693, 63.55, 65.4, 69.723,
        72.63, 74.92159, 78.97, 79.90, 83.80, 85.468, 87.62, 88.90584, 91.22, 92.90637, 95.95,
        96.90636, 101.1, 102.9055, 106.42, 107.868, 112.41, 114.818, 118.71, 121.760, 127.6,
        126.9045, 131.29, 132.90545, 137.33, 138.9055, 140.116, 140.90766, 144.24, 144.91276,
        150.4, 151.964, 157.2, 158.92535, 162.500, 164.93033, 167.26, 168.93422, 173.05, 174.9668,
        178.49, 180.9479, 183.84, 186.207, 190.2, 192.22, 195.08, 196.96657, 200.59, 204.383, 207.,
        208.9804, 208.98243, 209.98715, 222.01758, 223.01973, 226.0254, 227.02775, 232.038,
        231.03588, 238.0289, 237.04817, 244.0642, 243.06138, 247.07035, 247.07031, 251.07959,
        252.083, 257.09511, 258.09843, 259.101, 266.120, 267.122, 268.126, 269.128, 270.133,
        269.1336, 277.154, 282.166, 282.169, 286.179, 286.182, 290.192, 290.196, 293.205, 294.211,
        295.216,
    ];

    /// Van der Waals radii of the elements in angstroms.
    ///
    /// This array should not be indexed by atomic number, as the indices are
    /// off by one. Instead, use `Element::radius()`.
    ///
    /// Elements 1-60, 62-83, and 89-99 are taken from
    /// > "A cartography of the van der Waals territories"\
    /// > Santiago Alvarez\
    /// > Dalton Trans., 2013, 42, 8617-8636\
    /// > DOI: [10.1039/C3DT50599E](https://doi.org/10.1039/C3DT50599E)
    ///
    /// Elements 84-88 are taken from
    /// > "Consistent van der Waals Radii for the Whole Main Group"\
    /// > Manjeera Mantina, Adam C. Chamberlin, Rosendo Valero, Christopher J. Cramer, and Donald G. Truhlar\
    /// > J. Phys. Chem. A, 2009, 113 (19), 5806-5812\
    /// > DOI: [10.1021/jp8111556](https://doi.org/10.1021/jp8111556)
    ///
    /// Elements 61 and 100+ are estimated to 1 significant figure from surrounding trends.
    ///
    /// A sensible alternative for elements 1-96 would be
    /// the atomic radii from
    /// > "Atomic and Ionic Radii of Elements 1-96"\
    /// > Martin Rahm, Roald Hoffman, and N. W. Ashcroft\
    /// > Chemistry - A European Journal, 2016, 22 (41), 14625-14632\
    /// > DOI: [10.1002/chem.201602949](https://doi.org/10.1002/chem.201602949)
    const RADII: [f32; 118] = [
        1.20, 1.43, 2.12, 1.98, 1.91, 1.77, 1.66, 1.50, 1.46, 1.58, 2.50, 2.51, 2.25, 2.19, 1.90,
        1.89, 1.82, 1.83, 2.73, 2.62, 2.58, 2.46, 2.42, 2.45, 2.45, 2.44, 2.40, 2.40, 2.38, 2.39,
        2.32, 2.29, 1.88, 1.82, 1.86, 2.25, 3.21, 2.84, 2.75, 2.52, 2.56, 2.45, 2.44, 2.46, 2.44,
        2.15, 2.53, 2.49, 2.43, 2.42, 2.47, 1.99, 2.04, 2.06, 3.48, 3.03, 2.98, 2.88, 2.92, 2.95,
        3., 2.90, 2.87, 2.83, 2.79, 2.87, 2.81, 2.83, 2.79, 2.80, 2.74, 2.63, 2.53, 2.57, 2.49,
        2.48, 2.41, 2.29, 2.32, 2.45, 2.47, 2.60, 2.54, 1.97, 2.02, 2.20, 3.48, 2.83, 2.8, 2.93,
        2.88, 2.71, 2.82, 2.81, 2.83, 3.05, 3.4, 3.05, 2.7, 3., 3., 3., 3., 3., 3., 3., 3., 3., 3.,
        3., 3., 3., 3., 3., 3., 3., 3., 3.,
    ];

    /// Get the atomic number of the element.
    ///
    /// Note that the atomic number is equal to the discriminant:
    /// ```rust
    /// # use pdbmol_types::Element;
    /// assert_eq!(Element::Hydrogen as u8, Element::Hydrogen.atomic_number())
    /// ```
    pub fn atomic_number(&self) -> u8 {
        *self as u8
    }

    /// Get the element with the given atomic number.
    ///
    /// Note that the atomic number is equal to the discriminant.
    pub fn from_atomic_number(n: u8) -> Option<Self> {
        Self::from_repr(n)
    }

    /// Get the elemental symbol for the element.
    pub fn symbol(&self) -> &'static str {
        Self::SYMBOLS[*self as usize - 1]
    }

    /// Get the element with the given symbol.
    pub fn from_symbol(symbol: impl AsRef<[u8]>) -> Option<Self> {
        Self::SYMBOLS
            .into_iter()
            .position(|s| AsRef::<[u8]>::as_ref(s) == symbol.as_ref())
            .map(|i| i as u8 + 1)
            .and_then(Self::from_repr)
    }

    /// Get the element with the given symbol case insensitively.
    pub fn from_uncased_symbol(symbol: impl AsRef<[u8]>) -> Option<Self> {
        match symbol.as_ref() {
            [c] => Self::from_symbol([c.to_ascii_uppercase()]),
            [c1, c2] => Self::from_symbol([c1.to_ascii_uppercase(), c2.to_ascii_lowercase()]),
            _ => None,
        }
    }

    /// Get the name of the element.
    pub fn name(&self) -> &'static str {
        self.into()
    }

    /// Get the element with the given name.
    ///
    /// Note that the element can also be constructed directly:
    /// ```rust
    /// # use pdbmol_types::Element;
    /// assert_eq!(Element::from_name("Vanadium"), Some(Element::Vanadium))
    /// ```
    pub fn from_name(name: &str) -> Option<Self> {
        use std::str::FromStr;
        Self::from_str(name).ok()
    }

    /// Get the atomic mass of the element in daltons (or unified atomic mass units).
    pub fn mass(&self) -> f32 {
        Self::MASSES[*self as usize - 1]
    }

    /// Get the Van der Waals radius of the element in angstroms.
    ///
    /// Elements 1-60, 62-83, and 89-99 are taken from
    /// > "A cartography of the van der Waals territories"\
    /// > Santiago Alvarez\
    /// > Dalton Trans., 2013, 42, 8617-8636\
    /// > DOI: [10.1039/C3DT50599E](https://doi.org/10.1039/C3DT50599E)
    ///
    /// Elements 84-88 are taken from
    /// > "Consistent van der Waals Radii for the Whole Main Group"\
    /// > Manjeera Mantina, Adam C. Chamberlin, Rosendo Valero, Christopher J. Cramer, and Donald G. Truhlar\
    /// > J. Phys. Chem. A, 2009, 113 (19), 5806-5812\
    /// > DOI: [10.1021/jp8111556](https://doi.org/10.1021/jp8111556)
    ///
    /// Elements 61 and 100+ are estimated to 1 significant figure from surrounding trends.
    pub fn radius(&self) -> f32 {
        Self::RADII[*self as usize - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    /// Check that there are masses, symbols, radii etc for every element
    fn data_are_complete() {
        assert_eq!(Element::SYMBOLS.len(), Element::iter().count());
        assert_eq!(Element::RADII.len(), Element::iter().count());
        assert_eq!(Element::MASSES.len(), Element::iter().count());
    }

    #[test]
    fn test_atomic_numbers() {
        for element in Element::iter() {
            let n = element.atomic_number();
            assert_eq!(Element::from_atomic_number(n).unwrap(), element);
        }

        assert_eq!(Element::Hydrogen.atomic_number(), 1);
        assert_eq!(Element::Oganesson.atomic_number(), 118);

        assert_eq!(Element::Hydrogen, Element::from_atomic_number(1).unwrap());
        assert_eq!(
            Element::Oganesson,
            Element::from_atomic_number(118).unwrap()
        );
    }

    #[test]
    /// Symbol handling code assumes all symbols are ASCII; if this changes,
    /// they should be re-written
    fn element_symbols_are_ascii() {
        for symbol in Element::SYMBOLS {
            assert!(symbol.is_ascii());
        }
    }

    #[test]
    fn test_element_symbols() {
        for element in Element::iter() {
            let symbol = element.symbol();
            assert_eq!(Element::from_symbol(symbol).unwrap(), element);
        }

        assert_eq!(Element::Hydrogen.symbol(), "H");
        assert_eq!(Element::Vanadium.symbol(), "V");
        assert_eq!(Element::Sodium.symbol(), "Na");
        assert_eq!(Element::Manganese.symbol(), "Mn");
        assert_eq!(Element::Oganesson.symbol(), "Og");

        assert_eq!(Element::from_symbol("H"), Some(Element::Hydrogen));
        assert_eq!(Element::from_symbol("V"), Some(Element::Vanadium));
        assert_eq!(Element::from_symbol("Na"), Some(Element::Sodium));
        assert_eq!(Element::from_symbol("Mn"), Some(Element::Manganese));
        assert_eq!(Element::from_symbol("Og"), Some(Element::Oganesson));

        assert_eq!(Element::from_symbol("Zz"), None);
        assert_eq!(Element::from_symbol("h"), None);
        assert_eq!(Element::from_symbol("mn"), None);
        assert_eq!(Element::from_symbol("OG"), None);
        assert_eq!(Element::from_symbol("nA"), None);
        assert_eq!(Element::from_symbol("Hydrogen"), None);
        assert_eq!(Element::from_symbol("νάτριο"), None);
        assert_eq!(Element::from_symbol(""), None);
        assert_eq!(Element::from_symbol("\u{1053}"), None); //  Cyrillic Н
        assert_eq!(Element::from_symbol("\u{1085}"), None); //  Cyrillic н
    }

    #[test]
    /// Exhaustively test all casings of symbols with from_uncased_symbol
    fn test_uncased_element_symbols() {
        fn switch_case(s: &str) -> String {
            s.chars()
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect()
        }

        for element in Element::iter() {
            let (element, symbol) = (Some(element), element.symbol());

            for change_case_fn in [
                str::to_lowercase,
                str::to_uppercase,
                str::to_string, // Unchanged case
                switch_case,
            ] {
                assert_eq!(
                    Element::from_uncased_symbol(change_case_fn(symbol)),
                    element
                );
            }
        }

        assert_eq!(Element::from_uncased_symbol(""), None);
        assert_eq!(Element::from_uncased_symbol("Hydrogen"), None);
        assert_eq!(Element::from_uncased_symbol("Zz"), None);
        assert_eq!(Element::from_uncased_symbol("\u{1053}"), None); //  Cyrillic Н
        assert_eq!(Element::from_uncased_symbol("\u{1085}"), None); //  Cyrillic н
    }

    #[test]
    fn test_masses() {
        for (element, mass) in Element::iter().zip(Element::MASSES) {
            assert_eq!(element.mass(), mass)
        }

        for (element, mass) in [
            (Element::Hydrogen, 1.),
            (Element::Carbon, 12.),
            (Element::Uranium, 238.),
        ] {
            assert!((mass - element.mass()).abs() < 0.5);
        }
    }

    #[test]
    fn test_radii() {
        for (element, radius) in Element::iter().zip(Element::RADII) {
            assert_eq!(element.radius(), radius)
        }
    }

    #[test]
    fn test_element_names() {
        // Check `element.name()` never panics
        for element in Element::iter() {
            element.name();
        }

        assert_eq!(Element::Hydrogen.name(), "Hydrogen");
        assert_eq!(Element::Oganesson.name(), "Oganesson");

        assert_eq!(
            Element::from_atomic_number(1)
                .unwrap()
                .name(),
            "Hydrogen"
        );
        assert_eq!(
            Element::from_atomic_number(6)
                .unwrap()
                .name(),
            "Carbon"
        );
        assert_eq!(
            Element::from_atomic_number(29)
                .unwrap()
                .name(),
            "Copper"
        );
        assert_eq!(
            Element::from_atomic_number(92)
                .unwrap()
                .name(),
            "Uranium"
        );
        assert_eq!(
            Element::from_atomic_number(118)
                .unwrap()
                .name(),
            "Oganesson"
        );

        assert_eq!(Element::Hydrogen, Element::from_name("Hydrogen").unwrap());
        assert_eq!(Element::Oganesson, Element::from_name("Oganesson").unwrap());
    }
}
