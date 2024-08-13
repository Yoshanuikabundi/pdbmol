use std::{
    fmt::Display,
    iter::Peekable,
    num::{ParseFloatError, ParseIntError},
    ops::RangeInclusive,
    str::{FromStr, Lines},
};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct AtomRecord<S = String> {
    serial: i32,
    name: S,
    alt_loc: char,
    res_name: S,
    chain_id: char,
    res_seq: i32,
    i_code: char,
    x: f32,
    y: f32,
    z: f32,
    occupancy: f32,
    temp_factor: f32,
    element: S,
    charge: i8,
}

impl<S: Display> Display for AtomRecord<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let AtomRecord {
            serial,
            name,
            alt_loc,
            res_name,
            chain_id,
            res_seq,
            i_code,
            x,
            y,
            z,
            occupancy,
            temp_factor,
            element,
            charge,
        } = self;
        let charge: String = match charge.cmp(&0) {
            std::cmp::Ordering::Less => format!("{}-", -charge),
            std::cmp::Ordering::Equal => "  ".to_string(),
            std::cmp::Ordering::Greater => format!("{charge}+"),
        };
        let name = format!("{name: <3}");
        write!(f, "{serial: >5} {name: >4}{alt_loc}{res_name: >3} ")?;
        write!(f, "{chain_id}{res_seq: >4}{i_code}   ")?;
        write!(f, "{x: >8.3}{y: >8.3}{z: >8.3}")?;
        write!(f, "{occupancy: >6.2}{temp_factor: >6.2}          ")?;
        write!(f, "{element: >2}{charge: >2}")
    }
}

impl AtomRecord<&str> {
    pub fn to_owned(&self) -> AtomRecord<String> {
        let AtomRecord {
            serial,
            name,
            alt_loc,
            res_name,
            chain_id,
            res_seq,
            i_code,
            x,
            y,
            z,
            occupancy,
            temp_factor,
            element,
            charge,
        } = *self;

        AtomRecord {
            serial,
            name: name.to_owned(),
            alt_loc,
            res_name: res_name.to_owned(),
            chain_id: chain_id.to_owned(),
            res_seq,
            i_code,
            x,
            y,
            z,
            occupancy,
            temp_factor,
            element: element.to_owned(),
            charge,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PdbRecord<S = String> {
    /// First line of the entry including entry-wide metadata.
    ///
    /// Contains PDB ID code, classification, and date of deposition.
    ///
    /// Mandatory in deposited files.
    Header,
    /// List of ID codes that have obsoleted this entry.
    ///
    /// OBSLTE records indicate that the PDB entry has been removed from
    /// distribution and provide a list of the ID code(s) which replaced it.
    ///
    /// Optional in deposited files, mandatory in entries that have been
    /// replaced by a newer entry.
    Obslte,
    /// Description of the experiment represented in the entry.
    ///
    /// Mandatory in deposited files.
    Title,
    /// List of PDB entries that compose a larger macromolecular complexes.
    ///
    /// Optional in deposited files, mandatory when large macromolecular
    /// complexes are split into multiple PDB entries.
    Split,
    /// Severe error indicator.
    ///
    /// Optional in deposited files, mandatory when there are outstanding errors
    /// such as chirality.
    Caveat,
    /// Description of macromolecular contents of the entry.
    ///
    /// Mandatory in deposited files.
    Compnd,
    /// Biological source of macromolecules in the entry.
    ///
    /// Mandatory in deposited files.
    Source,
    /// List of keywords describing the macromolecule.
    ///
    /// Mandatory in deposited files.
    Keywds,
    /// Experimental technique used for the structure determination.
    ///
    /// Mandatory in deposited files.
    ExpDta,
    /// Number of models.
    ///
    /// Optional in deposited files, mandatory for NMR ensemble entries.
    NumMdl,
    /// Describes which coordinates are included in the entry's model.
    ///
    /// Optional in deposited files, mandatory for NMR minimized average
    /// structures or when the entire polymer chain contains C alpha or P atoms
    /// only.
    MdlTyp,
    /// List of contributors.
    ///
    /// Mandatory in deposited files.
    Author,
    /// Revision date and related information.
    ///
    /// Mandatory in deposited files.
    RevDat,
    /// List of entries obsoleted from public release and superseded by current
    /// entry.
    ///
    /// Optional in deposited files, mandatory for a replacement entry.
    Sprsde,
    /// Literature citation that defines the coordinate set.
    ///
    /// Optional in deposited files, mandatory for a publication describes the
    /// experiment.
    Jrnl,
    /// General remarks; they can be structured or free form.
    ///
    /// Optional in deposited files.
    Remark { remark_num: i16, remark: S },
    /// Reference to the entry in the sequence database(s).
    ///
    /// Split into DBREF1 and DBREF2 when accession IDs don't fit on one line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef,
    /// Identification of conflicts between PDB and the named sequence database.
    ///
    /// Optional in deposited files, mandatory if sequence conflict exists.
    SeqAdv,
    /// Primary sequence of backbone residues.
    ///
    /// Mandatory in deposited files, Mandatory if ATOM records exist.
    SeqRes { chain_id: char, res_names: Vec<S> },
    /// Identification of modifications to standard residues.
    ///
    /// Optional in deposited files, mandatory if modified group exists in the
    /// coordinates.
    ModRes,
    /// Identification of non-standard groups heterogens).
    ///
    /// Optional in deposited files, mandatory if a non-standard group other
    /// than water appears in the coordinates.
    Het,
    /// Compound name of the heterogens.
    ///
    /// Optional in deposited files, mandatory if a non-standard group other
    /// than water appears in the coordinates.
    HetNam,
    /// Synonymous compound names for heterogens.
    ///
    /// Optional in deposited files.
    HetSyn,
    /// Chemical formula of non-standard groups.
    ///
    /// Optional in deposited files, mandatory if a non-standard group or water
    /// appears in the coordinates.
    Formul,
    /// Identification of helical substructures.
    ///
    /// Optional in deposited files.
    Helix,
    /// Identification of sheet substructures.
    ///
    /// Optional in deposited files.
    Sheet,
    /// Identification of disulfide bonds.
    ///
    /// Optional in deposited files, mandatory if a disulfide bond is present.
    SsBond {
        serial_number: i16,
        res_name1: S,
        chain_id1: char,
        res_seq1: i32,
        i_code1: char,
        res_name2: S,
        chain_id2: char,
        res_seq2: i32,
        i_code2: char,
        symmetry_op1: S,
        symmetry_op2: S,
        length: f32,
    },
    /// Identification of inter-residue bonds.
    ///
    /// Optional in deposited files, mandatory if non-standard residues appear
    /// in a polymer
    Link,
    /// Identification of peptide residues in cis conformation.
    ///
    /// Optional in deposited files.
    CisPep,
    /// Identification of groups comprising important entity sites.
    ///
    /// Optional in deposited files.
    Site,
    /// Unit cell parameters, space group, and Z.
    ///
    /// Mandatory in deposited files.
    Cryst1 {
        a: f32,
        b: f32,
        c: f32,
        alpha: f32,
        beta: f32,
        gamma: f32,
        space_group: S,
        z: i16,
    },
    /// Transformation from orthogonal coordinates to the submitted coordinates
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// Mandatory in deposited files.
    OrigXN,
    /// Transformation from orthogonal coordinates to fractional crystallographic coordinates
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// Mandatory in deposited files.
    ScaleN,
    /// Transformations expressing non-crystallographic symmetry.
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// There may be multiple sets of these records.
    ///
    /// Optional in deposited files, mandatory if the complete asymmetric unit
    /// must be generated from the given coordinates using non-crystallographic
    /// symmetry.
    MtrixN,
    /// Specification of model number for multiple structures in a single coordinate entry.
    ///
    /// Optional in deposited files, mandatory if more than one model is present
    /// in the entry.
    Model(usize),
    /// Atomic coordinate records for standard groups.
    ///
    /// Optional in deposited files, mandatory if standard residues exist.
    Atom(AtomRecord<S>),
    /// Anisotropic temperature factors.
    ///
    /// Optional in deposited files.
    AnisoU,
    /// Chain terminator.
    ///
    /// Optional in deposited files, mandatory if ATOM records exist.
    Ter {
        serial: i32,
        res_name: S,
        chain_id: char,
        res_seq: i32,
        i_code: char,
    },
    /// Atomic coordinate records for heterogens.
    ///
    /// Optional in deposited files, mandatory if non-standard group exists.
    HetAtm(AtomRecord<S>),
    /// End-of-model record for multiple structures in a single coordinate entry.
    ///
    /// Optional in deposited files, mandatory if MODEL appears.
    EndMdl,
    /// Connectivity records.
    ///
    /// Optional in deposited files, mandatory if non-standard group appears and
    /// if LINK or SSBOND records exist.
    Conect { parent: S, bonds: Vec<S> },
    /// Control record for bookkeeping.
    ///
    /// Mandatory in deposited files.
    Master,
    /// Last record in the file.
    ///
    /// Mandatory in deposited files.
    End,
}

impl<S: Display + Default + Clone> Display for PdbRecord<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdbRecord::Header
            | PdbRecord::Obslte
            | PdbRecord::Title
            | PdbRecord::Split
            | PdbRecord::Caveat
            | PdbRecord::Compnd
            | PdbRecord::Source
            | PdbRecord::Keywds
            | PdbRecord::ExpDta
            | PdbRecord::NumMdl
            | PdbRecord::MdlTyp
            | PdbRecord::Author
            | PdbRecord::RevDat
            | PdbRecord::Sprsde
            | PdbRecord::Jrnl
            | PdbRecord::DbRef
            | PdbRecord::SeqAdv
            | PdbRecord::ModRes
            | PdbRecord::Het
            | PdbRecord::HetNam
            | PdbRecord::HetSyn
            | PdbRecord::Formul
            | PdbRecord::Helix
            | PdbRecord::Sheet
            | PdbRecord::Link
            | PdbRecord::CisPep
            | PdbRecord::Site
            | PdbRecord::OrigXN
            | PdbRecord::ScaleN
            | PdbRecord::MtrixN
            | PdbRecord::AnisoU
            | PdbRecord::Master
            | PdbRecord::End => Ok(()),
            PdbRecord::Remark { remark_num, remark } => {
                writeln!(f, "REMARK {remark_num: >3} {remark}")
            }
            PdbRecord::SeqRes {
                chain_id,
                res_names,
            } => {
                let num_res = res_names.len();
                for (i, chunk) in res_names.chunks(13).enumerate() {
                    write!(f, "SEQRES {: >3} {chain_id} {num_res: >4} ", i + 1)?;
                    for res_name in chunk {
                        write!(f, " {res_name: >3}")?;
                    }
                    writeln!(f, "")?;
                }
                Ok(())
            }
            PdbRecord::SsBond {
                serial_number,
                res_name1,
                chain_id1,
                res_seq1,
                i_code1,
                res_name2,
                chain_id2,
                res_seq2,
                i_code2,
                symmetry_op1,
                symmetry_op2,
                length,
            } => {
                write!(f, "SSBOND {serial_number: >3} {res_name1: >3}")?;
                write!(f, " {chain_id1} {res_seq1: >4}{i_code1}   ")?;
                write!(f, "{res_name2: >3} {chain_id2} {res_seq2: >4}")?;
                write!(f, "{i_code2}                       ")?;
                writeln!(f, "{symmetry_op1} {symmetry_op2} {length: >5.2}")
            }
            PdbRecord::Cryst1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                space_group,
                z,
            } => {
                write!(f, "CRYST1{a: >9.3}{b: >9.3}{c: >9.3}")?;
                write!(f, "{alpha: >7.2}{beta: >7.2}{gamma: >7.2} ")?;
                writeln!(f, "{space_group}{z: >4}")
            }
            PdbRecord::Model(i) => writeln!(f, "MODEL     {i: >4}"),
            PdbRecord::Atom(record) => writeln!(f, "ATOM  {record}"),
            PdbRecord::HetAtm(record) => writeln!(f, "HETATM{record}"),
            PdbRecord::Ter {
                serial,
                res_name,
                chain_id,
                res_seq,
                i_code,
            } => {
                writeln!(
                    f,
                    "TER   {serial: >5}      {res_name: >3} {chain_id}{res_seq: >4}{i_code}"
                )
            }
            PdbRecord::EndMdl => writeln!(f, "ENDMDL"),
            PdbRecord::Conect { parent, bonds } => {
                let bond1 = bonds.get(0).cloned().unwrap_or_default();
                let bond2 = bonds.get(1).cloned().unwrap_or_default();
                let bond3 = bonds.get(2).cloned().unwrap_or_default();
                let bond4 = bonds.get(3).cloned().unwrap_or_default();
                writeln!(
                    f,
                    "CONECT{parent: >5}{bond1: >5}{bond2: >5}{bond3: >5}{bond4: >5}"
                )
            }
        }
    }
}

impl Into<PdbRecord<String>> for PdbRecord<&str> {
    fn into(self) -> PdbRecord<String> {
        match self {
            PdbRecord::Header => PdbRecord::Header,
            PdbRecord::Obslte => PdbRecord::Obslte,
            PdbRecord::Title => PdbRecord::Title,
            PdbRecord::Split => PdbRecord::Split,
            PdbRecord::Caveat => PdbRecord::Caveat,
            PdbRecord::Compnd => PdbRecord::Compnd,
            PdbRecord::Source => PdbRecord::Source,
            PdbRecord::Keywds => PdbRecord::Keywds,
            PdbRecord::ExpDta => PdbRecord::ExpDta,
            PdbRecord::NumMdl => PdbRecord::NumMdl,
            PdbRecord::MdlTyp => PdbRecord::MdlTyp,
            PdbRecord::Author => PdbRecord::Author,
            PdbRecord::RevDat => PdbRecord::RevDat,
            PdbRecord::Sprsde => PdbRecord::Sprsde,
            PdbRecord::Jrnl => PdbRecord::Jrnl,
            PdbRecord::Remark { remark_num, remark } => PdbRecord::Remark {
                remark_num,
                remark: remark.to_owned(),
            },
            PdbRecord::DbRef => PdbRecord::DbRef,
            PdbRecord::SeqAdv => PdbRecord::SeqAdv,
            PdbRecord::SeqRes {
                chain_id,
                res_names,
            } => PdbRecord::SeqRes {
                chain_id: chain_id.to_owned(),
                res_names: res_names.into_iter().map(str::to_owned).collect(),
            },
            PdbRecord::ModRes => PdbRecord::ModRes,
            PdbRecord::Het => PdbRecord::Het,
            PdbRecord::HetNam => PdbRecord::HetNam,
            PdbRecord::HetSyn => PdbRecord::HetSyn,
            PdbRecord::Formul => PdbRecord::Formul,
            PdbRecord::Helix => PdbRecord::Helix,
            PdbRecord::Sheet => PdbRecord::Sheet,
            PdbRecord::SsBond {
                serial_number,
                res_name1,
                chain_id1,
                res_seq1,
                i_code1,
                res_name2,
                chain_id2,
                res_seq2,
                i_code2,
                symmetry_op1,
                symmetry_op2,
                length,
            } => PdbRecord::SsBond {
                serial_number,
                res_name1: res_name1.to_owned(),
                chain_id1,
                res_seq1,
                i_code1,
                res_name2: res_name2.to_owned(),
                chain_id2,
                res_seq2,
                i_code2,
                symmetry_op1: symmetry_op1.to_owned(),
                symmetry_op2: symmetry_op2.to_owned(),
                length,
            },
            PdbRecord::Link => PdbRecord::Link,
            PdbRecord::CisPep => PdbRecord::CisPep,
            PdbRecord::Site => PdbRecord::Site,
            PdbRecord::Cryst1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                space_group,
                z,
            } => PdbRecord::Cryst1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                space_group: space_group.to_owned(),
                z,
            },
            PdbRecord::OrigXN => PdbRecord::OrigXN,
            PdbRecord::ScaleN => PdbRecord::ScaleN,
            PdbRecord::MtrixN => PdbRecord::MtrixN,
            PdbRecord::Model(i) => PdbRecord::Model(i),
            PdbRecord::Atom(record) => PdbRecord::Atom(record.to_owned()),
            PdbRecord::AnisoU => PdbRecord::AnisoU,
            PdbRecord::Ter {
                serial,
                res_name,
                chain_id,
                res_seq,
                i_code,
            } => PdbRecord::Ter {
                serial,
                res_name: res_name.to_owned(),
                chain_id,
                res_seq,
                i_code,
            },
            PdbRecord::HetAtm(record) => PdbRecord::HetAtm(record.to_owned()),
            PdbRecord::EndMdl => PdbRecord::EndMdl,
            PdbRecord::Conect { parent, bonds } => PdbRecord::Conect {
                parent: parent.to_owned(),
                bonds: bonds.into_iter().map(str::to_owned).collect(),
            },
            PdbRecord::Master => PdbRecord::Master,
            PdbRecord::End => PdbRecord::End,
        }
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PdbParseErr {
    #[error("record type {0} is unknown")]
    UnknownRecordType(String),
    #[error("expected a {expected} record, found {found}")]
    UnexpectedRecord {
        expected: &'static str,
        found: String,
    },
    #[error("SEQRES record declared {expected} residues, but contained {found}")]
    SeqresResnameCountMismatch { expected: usize, found: usize },
    #[error("encountered unexpected end of file")]
    UnexpectedEof,
    #[error("couldn't parse int: {0}")]
    CouldNotParseInt(#[from] ParseIntError),
    #[error("couldn't parse float: {0}")]
    CouldNotParseFloat(#[from] ParseFloatError),
    #[error("couldn't parse charge: {0}")]
    CouldNotParseCharge(String),
    #[error("Line {0:?} too short to include essential data")]
    LineTooShort(String),
}

type Result<T, E = PdbParseErr> = std::result::Result<T, E>;

pub struct PdbRecordParser<'t> {
    lines: Peekable<Lines<'t>>,
    current_line: Option<&'t str>,
}

impl<'t> PdbRecordParser<'t> {
    pub fn from_str(s: &'t str) -> Self {
        Self {
            lines: s.lines().peekable(),
            current_line: None,
        }
    }

    fn get_continuation(&mut self, prefix: &str) -> Option<&'t str> {
        let &continuation_line = self.lines.peek()?;
        if continuation_line.starts_with(prefix) {
            self.lines.next()
        } else {
            None
        }
    }

    fn try_continuation(&mut self, record_name: &'static str) -> Result<&'t str> {
        if record_name.len() > 6 {
            panic!("record_name must be 6 characters or less")
        }
        let continuation_line = self.lines.next().ok_or(PdbParseErr::UnexpectedEof)?;
        if continuation_line.starts_with(record_name) {
            Ok(continuation_line)
        } else {
            Err(PdbParseErr::UnexpectedRecord {
                expected: record_name,
                found: continuation_line[..6].to_owned(),
            })
        }
    }

    fn get_current_line(&self) -> &'t str {
        self.current_line
            .expect("get_current_line called with no current_line set")
    }

    /// Get nonempty fields from a line
    ///
    /// Intended for fields representing repeated data types, so they can be
    /// collected
    fn split_line(
        line: &'t str,
        fields: impl IntoIterator<Item = RangeInclusive<usize>>,
    ) -> impl Iterator<Item = &'t str> {
        let line = line;
        fields
            .into_iter()
            .map_while(|range| line.get(range).map(str::trim).take_if(|s| !s.is_empty()))
    }

    fn split_current_line(
        &self,
        fields: impl IntoIterator<Item = RangeInclusive<usize>>,
    ) -> impl Iterator<Item = &'t str> {
        PdbRecordParser::split_line(self.get_current_line(), fields)
    }

    /// Get a field from the current line if the field exists
    fn try_field(&self, range: RangeInclusive<usize>) -> Result<&'t str> {
        let line = self.get_current_line();
        line.get(range)
            .ok_or(PdbParseErr::LineTooShort(line.to_owned()))
    }

    /// Get a field from the current line if the field exists and is not empty
    fn get_field(&self, range: RangeInclusive<usize>) -> Option<&'t str> {
        match self.try_field(range).map(str::trim) {
            Ok("") => None,
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }

    fn try_parsed_field<F>(&self, range: RangeInclusive<usize>) -> Result<F, PdbParseErr>
    where
        F: FromStr,
        F::Err: Into<PdbParseErr>,
        PdbParseErr: From<F::Err>,
    {
        Ok(self.try_field(range)?.trim().parse()?)
    }

    fn get_parsed_field<F>(&self, range: RangeInclusive<usize>) -> Result<Option<F>, PdbParseErr>
    where
        F: FromStr,
        F::Err: Into<PdbParseErr>,
        PdbParseErr: From<F::Err>,
    {
        Ok(self.get_field(range).map(str::parse).transpose()?)
    }

    fn try_char_field(&self, index: usize) -> Result<char, PdbParseErr> {
        Ok(self.try_field(index..=index)?.chars().next().unwrap())
    }

    fn get_char_field(&self, index: usize) -> Option<char> {
        self.get_field(index..=index)
            .map(|s| s.chars().next().unwrap())
    }

    fn parse_seqres(&mut self) -> Result<PdbRecord<&'t str>> {
        let ser_num: i16 = self.try_parsed_field(7..=9)?;
        let chain_id = self.try_char_field(11)?;
        let num_res: usize = self.try_parsed_field(13..=16)?;

        let res_names: Vec<&'t str> = [self.get_current_line()]
            .into_iter()
            .chain((ser_num + 1..).into_iter().map_while(|i| {
                self.get_continuation(&format!("SEQRES {i: >3} {chain_id} {num_res: >4}"))
            }))
            .map(|line| {
                PdbRecordParser::split_line(
                    line,
                    [
                        19..=21,
                        23..=25,
                        27..=29,
                        31..=33,
                        35..=37,
                        39..=41,
                        43..=45,
                        47..=49,
                        51..=53,
                        55..=57,
                        59..=61,
                        63..=65,
                        67..=69,
                    ],
                )
            })
            .flatten()
            .collect();

        if res_names.len() != num_res {
            Err(PdbParseErr::SeqresResnameCountMismatch {
                expected: num_res,
                found: res_names.len(),
            })
        } else {
            Ok(PdbRecord::SeqRes {
                chain_id,
                res_names,
            })
        }
    }

    fn parse_atomrecord(&mut self) -> Result<AtomRecord<&'t str>> {
        let atom = AtomRecord {
            serial: self.try_parsed_field(6..=10)?,
            name: self.try_field(12..=15)?.trim(),
            alt_loc: self.try_char_field(16)?,
            res_name: self.try_field(17..=19)?.trim(),
            chain_id: self.try_char_field(21)?,
            res_seq: self.try_parsed_field(22..=25)?,
            i_code: self.try_char_field(26)?,
            x: self.try_parsed_field(30..=37)?,
            y: self.try_parsed_field(38..=45)?,
            z: self.try_parsed_field(47..=53)?,
            occupancy: self.try_parsed_field(54..=59)?,
            temp_factor: self.try_parsed_field(60..=65)?,
            element: self.try_field(76..=77)?.trim(),
            charge: {
                match self.try_field(78..=79) {
                    Ok(s) if &s[1..] == "+" => s[..1].parse()?,
                    Ok(s) if &s[1..] == "-" => -s[..1].parse()?,
                    Ok(s) if s.trim() == "" => 0,
                    Ok(s) => Err(PdbParseErr::CouldNotParseCharge(s.to_owned()))?,
                    Err(PdbParseErr::LineTooShort(_)) => 0,
                    Err(e) => Err(e)?,
                }
            },
        };

        Ok(atom)
    }

    fn get_record(&mut self) -> Result<PdbRecord<&'t str>> {
        let line = self.get_current_line();
        match &line[..6] {
            "HEADER" => Ok(PdbRecord::Header),
            "OBSLTE" => Ok(PdbRecord::Obslte),
            "TITLE " => Ok(PdbRecord::Title),
            "SPLIT " => Ok(PdbRecord::Split),
            "CAVEAT" => Ok(PdbRecord::Caveat),
            "COMPND" => Ok(PdbRecord::Compnd),
            "SOURCE" => Ok(PdbRecord::Source),
            "KEYWDS" => Ok(PdbRecord::Keywds),
            "EXPDTA" => Ok(PdbRecord::ExpDta),
            "NUMMDL" => Ok(PdbRecord::NumMdl),
            "MDLTYP" => Ok(PdbRecord::MdlTyp),
            "AUTHOR" => Ok(PdbRecord::Author),
            "REVDAT" => Ok(PdbRecord::RevDat),
            "SPRSDE" => Ok(PdbRecord::Sprsde),
            "JRNL  " => Ok(PdbRecord::Jrnl),
            "REMARK" => Ok(PdbRecord::Remark {
                remark_num: self.try_field(7..=9)?.trim().parse()?,
                remark: self.try_field(11..=78)?,
            }),
            "DBREF " => Ok(PdbRecord::DbRef),
            "DBREF1" => {
                let _dbref1: &'t str = line;
                let _dbref2: &'t str = self.try_continuation("DBREF2")?;
                Ok(PdbRecord::DbRef)
            }
            "SEQADV" => Ok(PdbRecord::SeqAdv),
            "SEQRES" => self.parse_seqres(),
            "MODRES" => Ok(PdbRecord::ModRes),
            "HET   " => Ok(PdbRecord::Het),
            "HETNAM" => Ok(PdbRecord::HetNam),
            "HETSYN" => Ok(PdbRecord::HetSyn),
            "FORMUL" => Ok(PdbRecord::Formul),
            "HELIX " => Ok(PdbRecord::Helix),
            "SHEET " => Ok(PdbRecord::Sheet),
            "SSBOND" => Ok(PdbRecord::SsBond {
                serial_number: self.try_parsed_field(7..=9)?,
                res_name1: self.try_field(11..=13)?.trim(),
                chain_id1: self.try_char_field(15)?,
                res_seq1: self.try_parsed_field(17..=20)?,
                i_code1: self.try_char_field(21)?,
                res_name2: self.try_field(25..=27)?.trim(),
                chain_id2: self.try_char_field(29)?,
                res_seq2: self.try_parsed_field(31..=34)?,
                i_code2: self.try_char_field(35)?,
                symmetry_op1: self.try_field(59..=64)?,
                symmetry_op2: self.try_field(66..=71)?,
                length: self.try_parsed_field(73..=77)?,
            }),
            "LINK  " => Ok(PdbRecord::Link),
            "CISPEP" => Ok(PdbRecord::CisPep),
            "SITE  " => Ok(PdbRecord::Site),
            "CRYST1" => Ok(PdbRecord::Cryst1 {
                a: self.try_parsed_field(6..=14)?,
                b: self.try_parsed_field(15..=23)?,
                c: self.try_parsed_field(24..=32)?,
                alpha: self.try_parsed_field(33..=39)?,
                beta: self.try_parsed_field(40..=46)?,
                gamma: self.try_parsed_field(47..=53)?,
                space_group: self.try_field(55..=65)?,
                z: self.try_parsed_field(66..=69)?,
            }),
            "ORIGX1" => Ok(PdbRecord::OrigXN),
            "ORIGX2" => Ok(PdbRecord::OrigXN),
            "ORIGX3" => Ok(PdbRecord::OrigXN),
            "SCALE1" => Ok(PdbRecord::ScaleN),
            "SCALE2" => Ok(PdbRecord::ScaleN),
            "SCALE3" => Ok(PdbRecord::ScaleN),
            "MTRIX1" => Ok(PdbRecord::MtrixN),
            "MTRIX2" => Ok(PdbRecord::MtrixN),
            "MTRIX3" => Ok(PdbRecord::MtrixN),
            "MODEL " => Ok(PdbRecord::Model(self.try_parsed_field(10..=13)?)),
            "ATOM  " => Ok(PdbRecord::Atom(self.parse_atomrecord()?)),
            "ANISOU" => Ok(PdbRecord::AnisoU),
            "TER   " => Ok(PdbRecord::Ter {
                serial: self.try_parsed_field(6..=10)?,
                res_name: self.try_field(17..=19)?.trim(),
                chain_id: self.try_char_field(21)?,
                res_seq: self.try_parsed_field(22..=25)?,
                i_code: self.try_char_field(26)?,
            }),
            "HETATM" => Ok(PdbRecord::HetAtm(self.parse_atomrecord()?)),
            "ENDMDL" => Ok(PdbRecord::EndMdl),
            "CONECT" => Ok(PdbRecord::Conect {
                parent: self.try_field(6..=10)?.trim(),
                bonds: self
                    .split_current_line([11..=15, 16..=20, 21..=25, 26..=30])
                    .collect(),
            }),
            "MASTER" => Ok(PdbRecord::Master),
            "END   " => Ok(PdbRecord::End),
            s => Err(PdbParseErr::UnknownRecordType(s.to_owned())),
        }
    }
}

impl<'t> Iterator for PdbRecordParser<'t> {
    type Item = Result<PdbRecord<&'t str>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_line = self.lines.next();
        self.current_line?;
        Some(self.get_record())
    }
}
