macro_rules! pdb_records_inner {
    ($($(#[$($attrss:tt)*])* $variantname:ident => { $($columnname:ident : $columntype:ty = line[$columnstart:literal ..= $columnend:literal]),*}, )+ ) => {
        use paste::paste;
        paste!{
            use crate::parser::PdbRecordParseError;
            use crate::parser::types::ParseFromPdb;
            use crate::parser::types::WriteToPdb;
            use std::fmt::Display;
            use bounded_static::ToStatic;

            #[derive(Clone, Debug, PartialEq, ToStatic)]
            pub enum PdbRecord<'s> {
                $(
                    $(#[$($attrss)*])*
                    $variantname { $($columnname: $columntype),* },
                )+
            }

            impl PdbRecord<'_> {
                fn prefix(&self) -> &'static str {
                    match self {
                        $(
                            PdbRecord::$variantname {..} => paste!(stringify!([<$variantname:upper>])),
                        )*
                    }
                }
            }

            impl<'s> TryFrom<&'s str> for PdbRecord<'s> {
                type Error = PdbRecordParseError;

                fn try_from(line: &'s str) -> Result<Self, Self::Error> {
                    $(
                        if line.starts_with(stringify!([<$variantname:upper>]))
                            & line
                                .get(stringify!([<$variantname:upper>]).len()..6)
                                .map(|s| s.trim().is_empty())
                                .ok_or_else(|| PdbRecordParseError::LineTooShort(
                                    line.to_owned()
                                ))?
                        {
                            Ok(Self::$variantname {
                                $(
                                    $columnname: {
                                        let field_to_end = line
                                            .get($columnstart..)
                                            .ok_or_else(|| PdbRecordParseError::LineTooShort(
                                                line.to_owned()
                                            ))?;
                                        let field_str = field_to_end
                                            .get(..$columnend - $columnstart + 1)
                                            .unwrap_or(field_to_end);
                                        ParseFromPdb::parse_from_pdb(field_str)?
                                    }
                                ),*
                            })
                        } else
                    )+ {
                        Err(
                            PdbRecordParseError::UnknownRecordType(
                                line[..=6].trim().to_owned()
                            )
                        )
                    }
                }
            }

            impl Display for PdbRecord<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut line = format!("{: <100}", self.prefix());
                    match self {$(
                        Self::$variantname {$($columnname),*} => {$(
                            let width = $columnend - $columnstart + 1;
                            let s = WriteToPdb::write_to_pdb($columnname, width);
                            line.replace_range($columnstart..=$columnend, &s);
                        )*}
                    )+}
                    writeln!(f, "{}", line.trim())
                }
            }
        }
    };
}

macro_rules! pdb_records {
    ($($($(#[$($attrss:tt)*])* $variantname:ident)|+ => $variantdefinition:tt, )+ ) => {
        pdb_records_inner! {
            $(
                $(
                    $(#[$($attrss)*])*
                    $variantname => $variantdefinition ,
                )+
            )+
        }
    };
}
