#![allow(clippy::eq_op)]

use pdbmol_types::Element;
use std::{borrow::Cow, fmt::Debug};

use super::ConectBonds;

#[macro_use]
mod machinery;

fn right_justify(
    value: impl std::fmt::Display,
    width: usize,
) -> String {
    format!("{value: >width$}")
}

fn left_justify(
    value: impl std::fmt::Display,
    width: usize,
) -> String {
    format!("{value: <width$}")
}

fn format_atomname(
    value: impl AsRef<str>,
    width: usize,
) -> String {
    let name = format!("{: <3}", value.as_ref());
    format!("{name: >width$}")
}

fn float_precision<const P: usize>(
    value: &f32,
    width: usize,
) -> String {
    format!("{value: >width$.P$}")
}

fn or_space(
    value: &Option<char>,
    width: usize,
) -> String {
    assert_eq!(width, 1);
    value.unwrap_or(' ').to_string()
}

fn element_symbol(
    value: &Element,
    width: usize,
) -> String {
    format!("{: >width$}", value.symbol().to_uppercase())
}

pdb_records! {
    /// First line of the entry including entry-wide metadata.
    ///
    /// Contains PDB ID code, classification, and date of deposition.
    ///
    /// Mandatory in deposited files.
    Header => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// List of ID codes that have obsoleted this entry.
    ///
    /// OBSLTE records indicate that the PDB entry has been removed from
    /// distribution and provide a list of the ID code(s) which replaced it.
    ///
    /// Optional in deposited files, mandatory in entries that have been
    /// replaced by a newer entry.
    Obslte => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Description of the experiment represented in the entry.
    ///
    /// Mandatory in deposited files.
    Title => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// List of PDB entries that compose a larger macromolecular complexes.
    ///
    /// Optional in deposited files, mandatory when large macromolecular
    /// complexes are split into multiple PDB entries.
    Split => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Severe error indicator.
    ///
    /// Optional in deposited files, mandatory when there are outstanding errors
    /// such as chirality.
    Caveat => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Description of macromolecular contents of the entry.
    ///
    /// Mandatory in deposited files.
    Compnd => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Biological source of macromolecules in the entry.
    ///
    /// Mandatory in deposited files.
    Source => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// List of keywords describing the macromolecule.
    ///
    /// Mandatory in deposited files.
    Keywds => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Experimental technique used for the structure determination.
    ///
    /// Mandatory in deposited files.
    ExpDta => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Number of models.
    ///
    /// Optional in deposited files, mandatory for NMR ensemble entries.
    NumMdl => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Describes which coordinates are included in the entry's model.
    ///
    /// Optional in deposited files, mandatory for NMR minimized average
    /// structures or when the entire polymer chain contains C alpha or P atoms
    /// only.
    MdlTyp => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// List of contributors.
    ///
    /// Mandatory in deposited files.
    Author => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Revision date and related information.
    ///
    /// Mandatory in deposited files.
    RevDat => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// List of entries obsoleted from public release and superseded by current
    /// entry.
    ///
    /// Optional in deposited files, mandatory for a replacement entry.
    Sprsde => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Literature citation that defines the coordinate set.
    ///
    /// Optional in deposited files, mandatory for a publication describes the
    /// experiment.
    Jrnl => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// General remarks; they can be structured or free form.
    ///
    /// Optional in deposited files.
    Remark => {
        remark_num: i32 = line[7..=9] <=> right_justify,
        remark: Cow<'s, str> = line[11..=78] <=> left_justify
    },
    /// Reference to the entry in the sequence database(s).
    ///
    /// Split into DBREF1 and DBREF2 when accession IDs don't fit on one line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Reference to the entry in the sequence database(s).
    ///
    /// With DBREF2, alternative to DBREF when accession IDs don't fit on one
    /// line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef1 => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Reference to the entry in the sequence database(s).
    ///
    /// With DBREF1, alternative to DBREF when accession IDs don't fit on one
    /// line.
    ///
    /// Optional in deposited files, mandatory for all polymers.
    DbRef2 => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of conflicts between PDB and the named sequence database.
    ///
    /// Optional in deposited files, mandatory if sequence conflict exists.
    SeqAdv => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Primary sequence of backbone residues.
    ///
    /// Mandatory in deposited files, Mandatory if ATOM records exist.
    SeqRes => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of modifications to standard residues.
    ///
    /// Optional in deposited files, mandatory if modified group exists in the
    /// coordinates.
    ModRes => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of non-standard groups heterogens).
    ///
    /// Optional in deposited files, mandatory if a non-standard group other
    /// than water appears in the coordinates.
    Het => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Compound name of the heterogens.
    ///
    /// Optional in deposited files, mandatory if a non-standard group other
    /// than water appears in the coordinates.
    HetNam => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Synonymous compound names for heterogens.
    ///
    /// Optional in deposited files.
    HetSyn => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Chemical formula of non-standard groups.
    ///
    /// Optional in deposited files, mandatory if a non-standard group or water
    /// appears in the coordinates.
    Formul => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of helical substructures.
    ///
    /// Optional in deposited files.
    Helix => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of sheet substructures.
    ///
    /// Optional in deposited files.
    Sheet => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of disulfide bonds.
    ///
    /// Optional in deposited files, mandatory if a disulfide bond is present.
    SsBond => {
        serial_number: i32 = line[7..=9] <=> right_justify,
        res_name1: Cow<'s, str> = line[11..=13].trim() <=> right_justify,
        chain_id1: Option<char> = line[15..=15] <=> or_space,
        res_seq1: i32 = line[17..=20] <=> right_justify,
        i_code1: Option<char> = line[21..=21] <=> or_space,
        res_name2: Cow<'s, str> = line[25..=27].trim() <=> right_justify,
        chain_id2: Option<char> = line[29..=29] <=> or_space,
        res_seq2: i32 = line[31..=34] <=> right_justify,
        i_code2: Option<char> = line[35..=35] <=> or_space,
        symmetry_op1: Cow<'s, str> = line[59..=64] <=> right_justify,
        symmetry_op2: Cow<'s, str> = line[66..=71] <=> right_justify,
        length: Cow<'s, str> = line[73..=77] <=> right_justify
    },
    /// Identification of inter-residue bonds.
    ///
    /// Optional in deposited files, mandatory if non-standard residues appear
    /// in a polymer
    Link => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of peptide residues in cis conformation.
    ///
    /// Optional in deposited files.
    CisPep => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Identification of groups comprising important entity sites.
    ///
    /// Optional in deposited files.
    Site => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Unit cell parameters, space group, and Z.
    ///
    /// Mandatory in deposited files.
    Cryst1 => {
        a: f32 = line[6..=14] <=> float_precision::<3>,
        b: f32 = line[15..=23] <=> float_precision::<3>,
        c: f32 = line[24..=32] <=> float_precision::<3>,
        alpha: f32 = line[33..=39] <=> float_precision::<2>,
        beta: f32 = line[40..=46] <=> float_precision::<2>,
        gamma: f32 = line[47..=53] <=> float_precision::<2>,
        space_group: Cow<'s, str> = line[55..=65] <=> right_justify,
        z: Cow<'s, str> = line[66..=69] <=> right_justify
    },
    /// Transformation from orthogonal coordinates to the submitted coordinates
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// Mandatory in deposited files.
    OrigX1 | OrigX2 | OrigX3 => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Transformation from orthogonal coordinates to fractional crystallographic coordinates
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// Mandatory in deposited files.
    Scale1 | Scale2 | Scale3 => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Transformations expressing non-crystallographic symmetry.
    ///
    /// N may be 1, 2, or 3; all three records are required.
    ///
    /// There may be multiple sets of these records.
    ///
    /// Optional in deposited files, mandatory if the complete asymmetric unit
    /// must be generated from the given coordinates using non-crystallographic
    /// symmetry.
    Mtrix1 | Mtrix2 | Mtrix3 => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Specification of model number for multiple structures in a single coordinate entry.
    ///
    /// Optional in deposited files, mandatory if more than one model is present
    /// in the entry.
    Model => { model_serial: i32 = line[10..=13] <=> right_justify },
    /// Atomic coordinate records for standard groups.
    ///
    /// Optional in deposited files, mandatory if standard residues exist.
    Atom |
        /// Atomic coordinate records for heterogens.
        ///
        /// Optional in deposited files, mandatory if non-standard group exists.
        HetAtm =>
    {
        serial: i32 = line[6..=10] <=> right_justify,
        name: Cow<'s, str> = line[12..=15].trim() <=> format_atomname,
        alt_loc: Option<char> = line[16..=16] <=> or_space,
        res_name: Cow<'s, str> = line[17..=19].trim() <=> right_justify,
        chain_id: Option<char> = line[21..=21] <=> or_space,
        res_seq: i32 = line[22..=25] <=> right_justify,
        i_code: Option<char> = line[26..=26] <=> or_space,
        x: f32 = line[30..=37] <=> float_precision::<3>,
        y: f32 = line[38..=45] <=> float_precision::<3>,
        z: f32 = line[47..=53] <=> float_precision::<3>,
        occupancy: Cow<'s, str> = line[54..=59].trim() <=> right_justify,
        temp_factor: Cow<'s, str> = line[60..=65].trim() <=> right_justify,
        element: Element = line[76..=77] <=> element_symbol,
        charge: Cow<'s, str> = line[78..=79] <=> right_justify
    },
    /// Anisotropic temperature factors.
    ///
    /// Optional in deposited files.
    AnisoU => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Chain terminator.
    ///
    /// Optional in deposited files, mandatory if ATOM records exist.
    Ter => {
        serial: i32 = line[6..=10] <=> right_justify,
        res_name: Cow<'s, str> = line[17..=19].trim() <=> right_justify,
        chain_id: Option<char> = line[21..=21] <=> or_space,
        res_seq: i32 = line[22..=25] <=> right_justify,
        i_code: Option<char> = line[26..=26] <=> or_space
    },
    /// End-of-model record for multiple structures in a single coordinate entry.
    ///
    /// Optional in deposited files, mandatory if MODEL appears.
    EndMdl => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Connectivity records.
    ///
    /// Optional in deposited files, mandatory if non-standard group appears and
    /// if LINK or SSBOND records exist.
    Conect => {
        parent: i32 = line[6..=10] <=> right_justify,
        bonds: ConectBonds = line[11..=80] <=> right_justify
    },
    /// Control record for bookkeeping.
    ///
    /// Mandatory in deposited files.
    Master => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
    /// Last record in the file.
    ///
    /// Mandatory in deposited files.
    End => { s: Cow<'s, str> = line[6..=80] <=> left_justify },
}
