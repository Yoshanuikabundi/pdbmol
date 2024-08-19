#![allow(clippy::eq_op)]

use pdbmol_types::Element;
use std::borrow::Cow;

use super::{
    types::{PrecisionFloat, VerbatimStr},
    ConectBonds,
};

#[macro_use]
mod machinery;

pdb_records! {
    /// First line of the entry including entry-wide metadata.
    ///
    /// Contains PDB ID code, classification, and date of deposition.
    ///
    /// Mandatory in deposited files.
    Header => { s: VerbatimStr<'s> = line[6..=80] },
    /// List of ID codes that have obsoleted this entry.
    ///
    /// OBSLTE records indicate that the PDB entry has been removed from
    /// distribution and provide a list of the ID code(s) which replaced it.
    ///
    /// Optional in deposited files, mandatory in entries that have been
    /// replaced by a newer entry.
    Obslte => { s: VerbatimStr<'s> = line[6..=80] },
    /// Description of the experiment represented in the entry.
    ///
    /// Mandatory in deposited files.
    Title => { s: VerbatimStr<'s> = line[6..=80] },
    /// List of PDB entries that compose a larger macromolecular complexes.
    ///
    /// Optional in deposited files, mandatory when large macromolecular
    /// complexes are split into multiple PDB entries.
    Split => { s: VerbatimStr<'s> = line[6..=80] },
    /// Severe error indicator.
    ///
    /// Optional in deposited files, mandatory when there are outstanding errors
    /// such as chirality.
    Caveat => { s: VerbatimStr<'s> = line[6..=80] },
    /// Description of macromolecular contents of the entry.
    ///
    /// Mandatory in deposited files.
    Compnd => { s: VerbatimStr<'s> = line[6..=80] },
    /// Biological source of macromolecules in the entry.
    ///
    /// Mandatory in deposited files.
    Source => { s: VerbatimStr<'s> = line[6..=80] },
    /// List of keywords describing the macromolecule.
    ///
    /// Mandatory in deposited files.
    Keywds => { s: VerbatimStr<'s> = line[6..=80] },
    /// Experimental technique used for the structure determination.
    ///
    /// Mandatory in deposited files.
    ExpDta => { s: VerbatimStr<'s> = line[6..=80] },
    /// Number of models.
    ///
    /// Optional in deposited files, mandatory for NMR ensemble entries.
    NumMdl => { s: VerbatimStr<'s> = line[6..=80] },
    /// Describes which coordinates are included in the entry's model.
    ///
    /// Optional in deposited files, mandatory for NMR minimized average
    /// structures or when the entire polymer chain contains C alpha or P atoms
    /// only.
    MdlTyp => { s: VerbatimStr<'s> = line[6..=80] },
    /// List of contributors.
    ///
    /// Mandatory in deposited files.
    Author => { s: VerbatimStr<'s> = line[6..=80] },
    /// Revision date and related information.
    ///
    /// Mandatory in deposited files.
    RevDat => { s: VerbatimStr<'s> = line[6..=80] },
    /// List of entries obsoleted from public release and superseded by current
    /// entry.
    ///
    /// Optional in deposited files, mandatory for a replacement entry.
    Sprsde => { s: VerbatimStr<'s> = line[6..=80] },
    /// Literature citation that defines the coordinate set.
    ///
    /// Optional in deposited files, mandatory for a publication describes the
    /// experiment.
    Jrnl => { s: VerbatimStr<'s> = line[6..=80] },
    /// General remarks; they can be structured or free form.
    ///
    /// Optional in deposited files.
    Remark => {
        remark_num: i32 = line[7..=9],
        remark: VerbatimStr<'s> = line[11..=78]
    },
    /// Reference to the entry in the sequence database(s).
    ///
    /// Split into DBREF1 and DBREF2 when accession IDs don't fit on one line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef => { s: VerbatimStr<'s> = line[6..=80] },
    /// Reference to the entry in the sequence database(s).
    ///
    /// With DBREF2, alternative to DBREF when accession IDs don't fit on one
    /// line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef1 => { s: VerbatimStr<'s> = line[6..=80] },
    /// Reference to the entry in the sequence database(s).
    ///
    /// With DBREF1, alternative to DBREF when accession IDs don't fit on one
    /// line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef2 => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of conflicts between PDB and the named sequence database.
    ///
    /// Optional in deposited files, mandatory if sequence conflict exists.
    SeqAdv => { s: VerbatimStr<'s> = line[6..=80] },
    /// Primary sequence of backbone residues.
    ///
    /// Mandatory in deposited files, Mandatory if ATOM records exist.
    SeqRes => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of modifications to standard residues.
    ///
    /// Optional in deposited files, mandatory if modified group exists in the
    /// coordinates.
    ModRes => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of non-standard groups heterogens).
    ///
    /// Optional in deposited files, mandatory if a non-standard group other
    /// than water appears in the coordinates.
    Het => { s: VerbatimStr<'s> = line[6..=80] },
    /// Compound name of the heterogens.
    ///
    /// Optional in deposited files, mandatory if a non-standard group other
    /// than water appears in the coordinates.
    HetNam => { s: VerbatimStr<'s> = line[6..=80] },
    /// Synonymous compound names for heterogens.
    ///
    /// Optional in deposited files.
    HetSyn => { s: VerbatimStr<'s> = line[6..=80] },
    /// Chemical formula of non-standard groups.
    ///
    /// Optional in deposited files, mandatory if a non-standard group or water
    /// appears in the coordinates.
    Formul => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of helical substructures.
    ///
    /// Optional in deposited files.
    Helix => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of sheet substructures.
    ///
    /// Optional in deposited files.
    Sheet => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of disulfide bonds.
    ///
    /// Optional in deposited files, mandatory if a disulfide bond is present.
    SsBond => {
        serial_number: i32 = line[7..=9],
        res_name1: Cow<'s, str> = line[11..=13],
        chain_id1: Option<char> = line[15..=15],
        res_seq1: i32 = line[17..=20],
        i_code1: Option<char> = line[21..=21],
        res_name2: Cow<'s, str> = line[25..=27],
        chain_id2: Option<char> = line[29..=29],
        res_seq2: i32 = line[31..=34],
        i_code2: Option<char> = line[35..=35],
        symmetry_op1: Cow<'s, str> = line[59..=64],
        symmetry_op2: Cow<'s, str> = line[66..=71],
        length: Cow<'s, str> = line[73..=77]
    },
    /// Identification of inter-residue bonds.
    ///
    /// Optional in deposited files, mandatory if non-standard residues appear
    /// in a polymer
    Link => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of peptide residues in cis conformation.
    ///
    /// Optional in deposited files.
    CisPep => { s: VerbatimStr<'s> = line[6..=80] },
    /// Identification of groups comprising important entity sites.
    ///
    /// Optional in deposited files.
    Site => { s: VerbatimStr<'s> = line[6..=80] },
    /// Unit cell parameters, space group, and Z.
    ///
    /// Mandatory in deposited files.
    Cryst1 => {
        a: PrecisionFloat<3> = line[6..=14],
        b: PrecisionFloat<3> = line[15..=23],
        c: PrecisionFloat<3> = line[24..=32],
        alpha: PrecisionFloat<2> = line[33..=39],
        beta: PrecisionFloat<2> = line[40..=46],
        gamma: PrecisionFloat<2> = line[47..=53],
        space_group: VerbatimStr<'s> = line[55..=65],
        z: Cow<'s, str> = line[66..=69]
    },
    /// Transformation from orthogonal coordinates to the submitted coordinates
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// Mandatory in deposited files.
    OrigX1 | OrigX2 | OrigX3 => { s: VerbatimStr<'s> = line[6..=80] },
    /// Transformation from orthogonal coordinates to fractional crystallographic coordinates
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// Mandatory in deposited files.
    Scale1 | Scale2 | Scale3 => { s: VerbatimStr<'s> = line[6..=80] },
    /// Transformations expressing non-crystallographic symmetry.
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// There may be multiple sets of these records.
    ///
    /// Optional in deposited files, mandatory if the complete asymmetric unit
    /// must be generated from the given coordinates using non-crystallographic
    /// symmetry.
    Mtrix1 | Mtrix2 | Mtrix3 => { s: VerbatimStr<'s> = line[6..=80] },
    /// Specification of model number for multiple structures in a single coordinate entry.
    ///
    /// Optional in deposited files, mandatory if more than one model is present
    /// in the entry.
    Model => { model_serial: i32 = line[10..=13] },
    /// Atomic coordinate records for standard groups.
    ///
    /// Optional in deposited files, mandatory if standard residues exist.
    Atom |
        /// Atomic coordinate records for heterogens.
        ///
        /// Optional in deposited files, mandatory if non-standard group exists.
        HetAtm =>
    {
        serial: i32 = line[6..=10],
        name: Cow<'s, str> = line[12..=15],
        alt_loc: Option<char> = line[16..=16],
        res_name: Cow<'s, str> = line[17..=19],
        chain_id: Option<char> = line[21..=21],
        res_seq: i32 = line[22..=25],
        i_code: Option<char> = line[26..=26],
        x: PrecisionFloat<3> = line[30..=37],
        y: PrecisionFloat<3> = line[38..=45],
        z: PrecisionFloat<3> = line[47..=53],
        occupancy: Cow<'s, str> = line[54..=59],
        temp_factor: Cow<'s, str> = line[60..=65],
        element: Element = line[76..=77],
        charge: Cow<'s, str> = line[78..=79]
    },
    /// Anisotropic temperature factors.
    ///
    /// Optional in deposited files.
    AnisoU => { s: VerbatimStr<'s> = line[6..=80] },
    /// Chain terminator.
    ///
    /// Optional in deposited files, mandatory if ATOM records exist.
    Ter => {
        serial: i32 = line[6..=10],
        res_name: Cow<'s, str> = line[17..=19],
        chain_id: Option<char> = line[21..=21],
        res_seq: i32 = line[22..=25],
        i_code: Option<char> = line[26..=26]
    },
    /// End-of-model record for multiple structures in a single coordinate entry.
    ///
    /// Optional in deposited files, mandatory if MODEL appears.
    EndMdl => { s: VerbatimStr<'s> = line[6..=80] },
    /// Connectivity records.
    ///
    /// Optional in deposited files, mandatory if non-standard group appears and
    /// if LINK or SSBOND records exist.
    Conect => {
        parent: Cow<'s, str> = line[6..=10],
        bonds: ConectBonds = line[11..=80]
    },
    /// Control record for bookkeeping.
    ///
    /// Mandatory in deposited files.
    Master => { s: VerbatimStr<'s> = line[6..=80] },
    /// Last record in the file.
    ///
    /// Mandatory in deposited files.
    End => { s: VerbatimStr<'s> = line[6..=80] },
}
