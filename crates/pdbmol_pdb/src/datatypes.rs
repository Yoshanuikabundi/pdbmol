use itertools::Itertools;
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
        write!(f, "{serial: >5} {name: >4}{alt_loc}{res_name: >3} {chain_id}{res_seq: >4}{i_code}   {x: >8.3}{y: >8.3}{z: >8.3}{occupancy: >6.2}{temp_factor: >6.2}          {element: >2}{charge: >2}")
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
    /// First line of the entry, contains PDB ID code, classification, and date of deposition.
    ///
    /// Mandatory.
    HEADER,
    /// Statement that the entry has been removed from distribution and list of the ID code(s) which replaced it.
    ///
    /// Optional, mandatory in entries that have been replaced by a newer entry.
    OBSLTE,
    /// Description of the experiment represented in the entry.
    ///
    /// Mandatory.
    TITLE,
    /// List of PDB entries that compose a larger macromolecular complexes.
    ///
    /// Optional, mandatory when large macromolecular complexes are split into multiple PDB entries.
    SPLIT,
    /// Severe error indicator.
    ///
    /// Optional, mandatory when there are outstanding errors such as chirality.
    CAVEAT,
    /// Description of macromolecular contents of the entry.
    ///
    /// Mandatory.
    COMPND,
    /// Biological source of macromolecules in the entry.
    ///
    /// Mandatory.
    SOURCE,
    /// List of keywords describing the macromolecule.
    ///
    /// Mandatory.
    KEYWDS,
    /// Experimental technique used for the structure determination.
    ///
    /// Mandatory.
    EXPDTA,
    /// Number of models.
    ///
    /// Optional, mandatory for NMR ensemble entries.
    NUMMDL,
    /// Contains additional annotation pertinent to the coordinates presented in the entry.
    ///
    /// Optional, mandatory for NMR minimized average Structures or when the entire polymer chain contains C alpha or P atoms only.
    MDLTYP,
    /// List of contributors.
    ///
    /// Mandatory.
    AUTHOR,
    /// Revision date and related information.
    ///
    /// Mandatory.
    REVDAT,
    /// List of entries obsoleted from public release and replaced by current entry.
    ///
    /// Optional, mandatory for a replacement entry.
    SPRSDE,
    /// Literature citation that defines the coordinate set.
    ///
    /// Optional, mandatory for a publication describes the experiment.
    JRNL,
    /// General remarks; they can be structured or free form.
    ///
    /// Optional.
    REMARK { remark_num: i16, remark: S },
    /// Reference to the entry in the sequence database(s).
    ///
    /// Split into DBREF1 and DBREF2 when accession IDs don't fit on one line.
    ///
    /// Optional, mandatory for all polymers.
    DBREF,
    /// Identification of conflicts between PDB and the named sequence database.
    ///
    /// Optional, mandatory if sequence conflict exists.
    SEQADV,
    /// Primary sequence of backbone residues.
    ///
    /// Mandatory, Mandatory if ATOM records exist.
    SEQRES { chain_id: char, res_names: Vec<S> },
    /// Identification of modifications to standard residues.
    ///
    /// Optional, mandatory if modified group exists in the coordinates.
    MODRES,
    /// Identification of non-standard groups heterogens).
    ///
    /// Optional, mandatory if a non-standard group other than water appears in the coordinates.
    HET,
    /// Compound name of the heterogens.
    ///
    /// Optional, mandatory if a non-standard group other than water appears in the coordinates.
    HETNAM,
    /// Synonymous compound names for heterogens.
    ///
    /// Optional.
    HETSYN,
    /// Chemical formula of non-standard groups.
    ///
    /// Optional, mandatory if a non-standard group or water appears in the coordinates.
    FORMUL,
    /// Identification of helical substructures.
    ///
    /// Optional.
    HELIX,
    /// Identification of sheet substructures.
    ///
    /// Optional.
    SHEET,
    /// Identification of disulfide bonds.
    ///
    /// Optional, mandatory if a disulfide bond is present.
    SSBOND,
    /// Identification of inter-residue bonds.
    ///
    /// Optional, mandatory if non-standard residues appear in a polymer
    LINK,
    /// Identification of peptide residues in cis conformation.
    ///
    /// Optional.
    CISPEP,
    /// Identification of groups comprising important entity sites.
    ///
    /// Optional.
    SITE,
    /// Unit cell parameters, space group, and Z.
    ///
    /// Mandatory.
    CRYST1 {
        a: f32,
        b: f32,
        c: f32,
        alpha: f32,
        beta: f32,
        gamma: f32,
        space_group: S,
        z: i16,
    },
    /// Transformation from orthogonal coordinates to the submitted coordinates (n = 1, 2, or 3).
    ///
    /// Mandatory.
    ORIGXn,
    /// Transformation from orthogonal coordinates to fractional crystallographic coordinates (n = 1, 2, or 3).
    ///
    /// Mandatory.
    SCALEn,
    /// Transformations expressing non-crystallographic symmetry (n = 1, 2, or 3).
    ///
    /// There may be multiple sets of these records.
    ///
    /// Optional, mandatory if the complete asymmetric unit must be generated from the given coordinates using non-crystallographic symmetry.
    MTRIXn,
    /// Specification of model number for multiple structures in a single coordinate entry.
    ///
    /// Optional, mandatory if more than one model is present in the entry.
    MODEL(usize),
    /// Atomic coordinate records for standard groups.
    ///
    /// Optional, mandatory if standard residues exist.
    ATOM(AtomRecord<S>),
    /// Anisotropic temperature factors.
    ///
    /// Optional.
    ANISOU,
    /// Chain terminator.
    ///
    /// Optional, mandatory if ATOM records exist.
    TER {
        serial: i32,
        res_name: S,
        chain_id: char,
        res_seq: i32,
        i_code: char,
    },
    /// Atomic coordinate records for heterogens.
    ///
    /// Optional, mandatory if non-standard group exists.
    HETATM(AtomRecord<S>),
    /// End-of-model record for multiple structures in a single coordinate entry.
    ///
    /// Optional, mandatory if MODEL appears.
    ENDMDL,
    /// Connectivity records.
    ///
    /// Optional, mandatory if non-standard group appears and if LINK or SSBOND records exist.
    CONECT {
        parent: S,
        bond1: Option<S>,
        bond2: Option<S>,
        bond3: Option<S>,
        bond4: Option<S>,
    },
    /// Control record for bookkeeping.
    ///
    /// Mandatory.
    MASTER,
    /// Last record in the file.
    ///
    /// Mandatory.
    END,
}

impl<S: Display + Default + Clone> Display for PdbRecord<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdbRecord::HEADER
            | PdbRecord::OBSLTE
            | PdbRecord::TITLE
            | PdbRecord::SPLIT
            | PdbRecord::CAVEAT
            | PdbRecord::COMPND
            | PdbRecord::SOURCE
            | PdbRecord::KEYWDS
            | PdbRecord::EXPDTA
            | PdbRecord::NUMMDL
            | PdbRecord::MDLTYP
            | PdbRecord::AUTHOR
            | PdbRecord::REVDAT
            | PdbRecord::SPRSDE
            | PdbRecord::JRNL
            | PdbRecord::DBREF
            | PdbRecord::SEQADV
            | PdbRecord::MODRES
            | PdbRecord::HET
            | PdbRecord::HETNAM
            | PdbRecord::HETSYN
            | PdbRecord::FORMUL
            | PdbRecord::HELIX
            | PdbRecord::SHEET
            | PdbRecord::SSBOND
            | PdbRecord::LINK
            | PdbRecord::CISPEP
            | PdbRecord::SITE
            | PdbRecord::ORIGXn
            | PdbRecord::SCALEn
            | PdbRecord::MTRIXn
            | PdbRecord::ANISOU
            | PdbRecord::MASTER
            | PdbRecord::END => Ok(()),
            PdbRecord::REMARK { remark_num, remark } => {
                writeln!(f, "REMARK {remark_num: >3} {remark}")
            }
            PdbRecord::SEQRES {
                chain_id,
                res_names,
            } => {
                let num_res = res_names.len();
                for (i, chunk) in res_names.chunks(13).enumerate() {
                    write!(f, "SEQRES {: >3} {chain_id} {num_res: >4} ", i+1)?;
                    for res_name in chunk {
                        write!(f, " {res_name: >3}")?;
                    }
                    writeln!(f, "")?;
                };
                Ok(())
            },
            PdbRecord::CRYST1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                space_group,
                z,
            } => writeln!(f, "CRYST1{a: >9.3}{b: >9.3}{c: >9.3}{alpha: >7.2}{beta: >7.2}{gamma: >7.2} {space_group}{z: >4}"),
            PdbRecord::MODEL(i) => writeln!(f, "MODEL     {i: >4}"),
            PdbRecord::ATOM(record) => writeln!(f, "ATOM  {record}"),
            PdbRecord::HETATM(record) => writeln!(f, "HETATM{record}"),
            PdbRecord::TER { serial, res_name, chain_id, res_seq, i_code } => {
                writeln!(f, "TER   {serial: >5}      {res_name: >3} {chain_id}{res_seq: >4}{i_code}")
            },
            PdbRecord::ENDMDL => writeln!(f, "ENDMDL"),
            PdbRecord::CONECT {
                parent,
                bond1,
                bond2,
                bond3,
                bond4,
            } => {
                let bond1 = bond1.clone().unwrap_or_default();
                let bond2 = bond2.clone().unwrap_or_default();
                let bond3 = bond3.clone().unwrap_or_default();
                let bond4 = bond4.clone().unwrap_or_default();
                writeln!(f, "CONECT{parent: >5}{bond1: >5}{bond2: >5}{bond3: >5}{bond4: >5}")
            },
        }
    }
}

impl Into<PdbRecord<String>> for PdbRecord<&str> {
    fn into(self) -> PdbRecord<String> {
        match self {
            PdbRecord::HEADER => PdbRecord::HEADER,
            PdbRecord::OBSLTE => PdbRecord::OBSLTE,
            PdbRecord::TITLE => PdbRecord::TITLE,
            PdbRecord::SPLIT => PdbRecord::SPLIT,
            PdbRecord::CAVEAT => PdbRecord::CAVEAT,
            PdbRecord::COMPND => PdbRecord::COMPND,
            PdbRecord::SOURCE => PdbRecord::SOURCE,
            PdbRecord::KEYWDS => PdbRecord::KEYWDS,
            PdbRecord::EXPDTA => PdbRecord::EXPDTA,
            PdbRecord::NUMMDL => PdbRecord::NUMMDL,
            PdbRecord::MDLTYP => PdbRecord::MDLTYP,
            PdbRecord::AUTHOR => PdbRecord::AUTHOR,
            PdbRecord::REVDAT => PdbRecord::REVDAT,
            PdbRecord::SPRSDE => PdbRecord::SPRSDE,
            PdbRecord::JRNL => PdbRecord::JRNL,
            PdbRecord::REMARK { remark_num, remark } => PdbRecord::REMARK {
                remark_num,
                remark: remark.to_owned(),
            },
            PdbRecord::DBREF => PdbRecord::DBREF,
            PdbRecord::SEQADV => PdbRecord::SEQADV,
            PdbRecord::SEQRES {
                chain_id,
                res_names,
            } => PdbRecord::SEQRES {
                chain_id: chain_id.to_owned(),
                res_names: res_names.into_iter().map(|s| s.to_owned()).collect(),
            },
            PdbRecord::MODRES => PdbRecord::MODRES,
            PdbRecord::HET => PdbRecord::HET,
            PdbRecord::HETNAM => PdbRecord::HETNAM,
            PdbRecord::HETSYN => PdbRecord::HETSYN,
            PdbRecord::FORMUL => PdbRecord::FORMUL,
            PdbRecord::HELIX => PdbRecord::HELIX,
            PdbRecord::SHEET => PdbRecord::SHEET,
            PdbRecord::SSBOND => PdbRecord::SSBOND,
            PdbRecord::LINK => PdbRecord::LINK,
            PdbRecord::CISPEP => PdbRecord::CISPEP,
            PdbRecord::SITE => PdbRecord::SITE,
            PdbRecord::CRYST1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                space_group,
                z,
            } => PdbRecord::CRYST1 {
                a,
                b,
                c,
                alpha,
                beta,
                gamma,
                space_group: space_group.to_owned(),
                z,
            },
            PdbRecord::ORIGXn => PdbRecord::ORIGXn,
            PdbRecord::SCALEn => PdbRecord::SCALEn,
            PdbRecord::MTRIXn => PdbRecord::MTRIXn,
            PdbRecord::MODEL(i) => PdbRecord::MODEL(i),
            PdbRecord::ATOM(record) => PdbRecord::ATOM(record.to_owned()),
            PdbRecord::ANISOU => PdbRecord::ANISOU,
            PdbRecord::TER {
                serial,
                res_name,
                chain_id,
                res_seq,
                i_code,
            } => PdbRecord::TER {
                serial,
                res_name: res_name.to_owned(),
                chain_id,
                res_seq,
                i_code,
            },
            PdbRecord::HETATM(record) => PdbRecord::HETATM(record.to_owned()),
            PdbRecord::ENDMDL => PdbRecord::ENDMDL,
            PdbRecord::CONECT {
                parent,
                bond1,
                bond2,
                bond3,
                bond4,
            } => PdbRecord::CONECT {
                parent: parent.to_owned(),
                bond1: bond1.map(str::to_owned),
                bond2: bond2.map(str::to_owned),
                bond3: bond3.map(str::to_owned),
                bond4: bond4.map(str::to_owned),
            },
            PdbRecord::MASTER => PdbRecord::MASTER,
            PdbRecord::END => PdbRecord::END,
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
            Ok(PdbRecord::SEQRES {
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
            "HEADER" => Ok(PdbRecord::HEADER),
            "OBSLTE" => Ok(PdbRecord::OBSLTE),
            "TITLE " => Ok(PdbRecord::TITLE),
            "SPLIT " => Ok(PdbRecord::SPLIT),
            "CAVEAT" => Ok(PdbRecord::CAVEAT),
            "COMPND" => Ok(PdbRecord::COMPND),
            "SOURCE" => Ok(PdbRecord::SOURCE),
            "KEYWDS" => Ok(PdbRecord::KEYWDS),
            "EXPDTA" => Ok(PdbRecord::EXPDTA),
            "NUMMDL" => Ok(PdbRecord::NUMMDL),
            "MDLTYP" => Ok(PdbRecord::MDLTYP),
            "AUTHOR" => Ok(PdbRecord::AUTHOR),
            "REVDAT" => Ok(PdbRecord::REVDAT),
            "SPRSDE" => Ok(PdbRecord::SPRSDE),
            "JRNL  " => Ok(PdbRecord::JRNL),
            "REMARK" => Ok(PdbRecord::REMARK {
                remark_num: self.try_field(7..=9)?.trim().parse()?,
                remark: self.try_field(11..=78)?,
            }),
            "DBREF " => Ok(PdbRecord::DBREF),
            "DBREF1" => {
                let _dbref1: &'t str = line;
                let _dbref2: &'t str = self.try_continuation("DBREF2")?;
                Ok(PdbRecord::DBREF)
            }
            "SEQADV" => Ok(PdbRecord::SEQADV),
            "SEQRES" => self.parse_seqres(),
            "MODRES" => Ok(PdbRecord::MODRES),
            "HET   " => Ok(PdbRecord::HET),
            "HETNAM" => Ok(PdbRecord::HETNAM),
            "HETSYN" => Ok(PdbRecord::HETSYN),
            "FORMUL" => Ok(PdbRecord::FORMUL),
            "HELIX " => Ok(PdbRecord::HELIX),
            "SHEET " => Ok(PdbRecord::SHEET),
            "SSBOND" => Ok(PdbRecord::SSBOND),
            "LINK  " => Ok(PdbRecord::LINK),
            "CISPEP" => Ok(PdbRecord::CISPEP),
            "SITE  " => Ok(PdbRecord::SITE),
            "CRYST1" => Ok(PdbRecord::CRYST1 {
                a: self.try_parsed_field(6..=14)?,
                b: self.try_parsed_field(15..=23)?,
                c: self.try_parsed_field(24..=32)?,
                alpha: self.try_parsed_field(33..=39)?,
                beta: self.try_parsed_field(40..=46)?,
                gamma: self.try_parsed_field(47..=53)?,
                space_group: self.try_field(55..=65)?,
                z: self.try_parsed_field(66..=69)?,
            }),
            "ORIGX1" => Ok(PdbRecord::ORIGXn),
            "ORIGX2" => Ok(PdbRecord::ORIGXn),
            "ORIGX3" => Ok(PdbRecord::ORIGXn),
            "SCALE1" => Ok(PdbRecord::SCALEn),
            "SCALE2" => Ok(PdbRecord::SCALEn),
            "SCALE3" => Ok(PdbRecord::SCALEn),
            "MTRIX1" => Ok(PdbRecord::MTRIXn),
            "MTRIX2" => Ok(PdbRecord::MTRIXn),
            "MTRIX3" => Ok(PdbRecord::MTRIXn),
            "MODEL " => Ok(PdbRecord::MODEL(self.try_parsed_field(10..=13)?)),
            "ATOM  " => Ok(PdbRecord::ATOM(self.parse_atomrecord()?)),
            "ANISOU" => Ok(PdbRecord::ANISOU),
            "TER   " => Ok(PdbRecord::TER {
                serial: self.try_parsed_field(6..=10)?,
                res_name: self.try_field(17..=19)?.trim(),
                chain_id: self.try_char_field(21)?,
                res_seq: self.try_parsed_field(22..=25)?,
                i_code: self.try_char_field(26)?,
            }),
            "HETATM" => Ok(PdbRecord::HETATM(self.parse_atomrecord()?)),
            "ENDMDL" => Ok(PdbRecord::ENDMDL),
            "CONECT" => Ok(PdbRecord::CONECT {
                parent: self.try_field(6..=10)?.trim(),
                bond1: self.get_field(11..=15),
                bond2: self.get_field(16..=20),
                bond3: self.get_field(21..=25),
                bond4: self.get_field(26..=30),
            }),
            "MASTER" => Ok(PdbRecord::MASTER),
            "END   " => Ok(PdbRecord::END),
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
