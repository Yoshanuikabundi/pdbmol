use std::{
    error::Error,
    fmt::{write, Display},
    iter::Peekable,
    num::ParseIntError,
    ops::{RangeBounds, RangeInclusive},
    str::Lines,
};
use thiserror::Error;

pub enum PdbRecord<'t> {
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
    REMARK,
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
    SEQRES {
        chain_id: char,
        res_names: Vec<&'t str>,
    },
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
    CRYST1,
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
    MODEL,
    /// Atomic coordinate records for standard groups.
    ///
    /// Optional, mandatory if standard residues exist.
    ATOM,
    /// Anisotropic temperature factors.
    ///
    /// Optional.
    ANISOU,
    /// Chain terminator.
    ///
    /// Optional, mandatory if ATOM records exist.
    TER,
    /// Atomic coordinate records for heterogens.
    ///
    /// Optional, mandatory if non-standard group exists.
    HETATM,
    /// End-of-model record for multiple structures in a single coordinate entry.
    ///
    /// Optional, mandatory if MODEL appears.
    ENDMDL,
    /// Connectivity records.
    ///
    /// Optional, mandatory if non-standard group appears and if LINK or SSBOND records exist.
    CONECT,
    /// Control record for bookkeeping.
    ///
    /// Mandatory.
    MASTER,
    /// Last record in the file.
    ///
    /// Mandatory.
    END,
}

#[derive(Error, Debug)]
enum PdbParseErr {
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
    #[error("Line {0:?} too short to include essential data")]
    LineTooShort(String),
}

type Result<T, E = PdbParseErr> = std::result::Result<T, E>;

struct PdbRecordParser<'t> {
    lines: Peekable<Lines<'t>>,
    current_line: Option<&'t str>,
}

impl<'t> PdbRecordParser<'t> {
    fn try_continuation(&mut self, prefix: &str) -> Option<&'t str> {
        let &continuation_line = self.lines.peek()?;
        if continuation_line.starts_with(prefix) {
            self.lines.next()
        } else {
            None
        }
    }

    fn get_continuation(&mut self, record_name: &'static str) -> Result<&'t str> {
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

    fn get_field(&self, range: RangeInclusive<usize>) -> Result<&'t str> {
        let line = self.get_current_line();
        line.get(range)
            .ok_or(PdbParseErr::LineTooShort(line.to_owned()))
    }

    fn parse_seqres(&mut self) -> Result<PdbRecord> {
        let ser_num: i16 = self.get_field(7..=9)?.parse()?;
        let chain_id = self.get_field(11..=11)?.chars().next().unwrap();
        let num_res: usize = self.get_field(13..=16)?.parse()?;

        let mut res_names: Vec<&str> = Vec::with_capacity(num_res);
        let seqres_resname_ranges = [
            19..=21,
            23..=25,
            27..=29,
            31..=33,
            35..=37,
            39..=41,
            43..=45,
            47..=49,
            51..=53,
            56..=58,
            59..=61,
            63..=65,
            67..=69,
        ];
        res_names.extend(self.split_current_line(seqres_resname_ranges.clone()));

        (ser_num + 1..)
            .into_iter()
            .map_while(|i| {
                self.try_continuation(&format!("SEQRES {i: >3} {chain_id} {num_res: >4}"))
            })
            .for_each(|line| {
                res_names.extend(PdbRecordParser::split_line(
                    line,
                    seqres_resname_ranges.clone(),
                ))
            });

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

    fn get_record(&mut self) -> Result<PdbRecord> {
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
            "REMARK" => Ok(PdbRecord::REMARK),
            "DBREF " => Ok(PdbRecord::DBREF),
            "DBREF1" => {
                let dbref1 = &line;
                let dbref2 = self.get_continuation("DBREF2");
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
            "CRYST1" => Ok(PdbRecord::CRYST1),
            "ORIGX1" => Ok(PdbRecord::ORIGXn),
            "ORIGX2" => Ok(PdbRecord::ORIGXn),
            "ORIGX3" => Ok(PdbRecord::ORIGXn),
            "SCALE1" => Ok(PdbRecord::SCALEn),
            "SCALE2" => Ok(PdbRecord::SCALEn),
            "SCALE3" => Ok(PdbRecord::SCALEn),
            "MTRIX1" => Ok(PdbRecord::MTRIXn),
            "MTRIX2" => Ok(PdbRecord::MTRIXn),
            "MTRIX3" => Ok(PdbRecord::MTRIXn),
            "MODEL " => Ok(PdbRecord::MODEL),
            "ATOM  " => Ok(PdbRecord::ATOM),
            "ANISOU" => Ok(PdbRecord::ANISOU),
            "TER   " => Ok(PdbRecord::TER),
            "HETATM" => Ok(PdbRecord::HETATM),
            "ENDMDL" => Ok(PdbRecord::ENDMDL),
            "CONECT" => Ok(PdbRecord::CONECT),
            "MASTER" => Ok(PdbRecord::MASTER),
            "END   " => Ok(PdbRecord::END),
            s => Err(PdbParseErr::UnknownRecordType(s.to_owned())),
        }
    }
}

impl<'t> Iterator for PdbRecordParser<'t> {
    type Item = Result<PdbRecord>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_line = self.lines.next();
        self.current_line?;
        Some(self.get_record())
    }
}
