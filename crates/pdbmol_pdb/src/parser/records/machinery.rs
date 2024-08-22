macro_rules! pdb_records_inner {
    ($($(
        #[$($attrss:tt)*])*
        $variantname:ident =>
        { $(
            $columnname:ident : $columntype:ty
            = line[$columnstart:literal ..= $columnend:literal]$(.$parsepostproc:ident())*
            <=> $writer:expr
        ),*},
    )+ ) => {
        use paste::paste;
        paste!{
            use crate::parser::PdbRecordParseError;
            use crate::parser::types::ParseFromPdb;
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
                const fn prefix(&self) -> &'static str {
                    match self {
                        $(
                            PdbRecord::$variantname {..} => stringify!([<$variantname:upper>]),
                        )*
                    }
                }
            }

            impl<'s> TryFrom<&'s str> for PdbRecord<'s> {
                type Error = PdbRecordParseError;

                fn try_from(line: &'s str) -> Result<Self, Self::Error> {
                    match line.get(..6).unwrap_or(line).trim_end() {
                        $(
                            stringify!([<$variantname:upper>]) => {
                                Ok(Self::$variantname {$(
                                    $columnname: {
                                        let field_to_end = line
                                            .get($columnstart..)
                                            .ok_or_else(|| PdbRecordParseError::LineTooShort(
                                                line.to_string()
                                            ))?;
                                        let field_str = field_to_end
                                            .get(..=$columnend - $columnstart)
                                            .unwrap_or(field_to_end)
                                            $(.$parsepostproc())*;
                                        ParseFromPdb::parse_from_pdb(field_str)?
                                    }
                                ),*})
                            }
                        )+
                        "" => Err(PdbRecordParseError::EmptyLine),
                        s => Err(PdbRecordParseError::UnknownRecordType(s.to_string())),
                    }
                }
            }

            impl Display for PdbRecord<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut line = format!("{: <100}", self.prefix());
                    match self {$(
                        Self::$variantname {$($columnname),*} => {$(
                            line.replace_range(
                                $columnstart..=$columnend,
                                &$writer($columnname, $columnend - $columnstart + 1)
                            );
                        )+}
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
