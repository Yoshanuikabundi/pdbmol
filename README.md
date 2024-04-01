# PDBMol
*Josh Mitchell 2024-*

[![Crates.io](https://img.shields.io/crates/v/pdbmol.svg)](https://crates.io/crates/pdbmol)
[![Docs.rs](https://docs.rs/pdbmol/badge.svg)](https://docs.rs/pdbmol)
[![CI](https://github.com/Yoshanuikabundi/pdbmol/workflows/CI/badge.svg)](https://github.com/Yoshanuikabundi/pdbmol/actions)

PDB files are a ubiquitous format for sharing biomolecular structure information. They are used by both experimental and computational scientists for distribution, cross-software interchange, and storage. As the fixed-width limitations of PDBs make extending them increasingly problematic, PDBx/mmCIF is often recommended as a successor format, though it has not yet achieved the same popularity. Unfortunately, most software that parses or writes PDBs is hand-rolled and often produces subtly different and sometimes mutually incompatible files:

- Parsers typically only support a small selection of known residue types,
  rather than the entire [CCD].
- Writers often use custom or even arbitrarily generated atom names, breaking
  bond inference
- Parsers typically infer bonds from atomic positions rather than atom and
  residue names and `CONECT` records, resulting in incorrect bonding in
  strained conformations
- Parsers sometimes consider atom records with the same identifiers to be
  duplicates, and sometimes to be separate atoms 

The objective of PDBMol is to provide a spec-compliant PDB(x)/mmCIF reader/writer library that can be adopted by other projects to provide consistent PDB file handling. A secondary objective is to provide compatibility with common existing PDB dialects. To those ends, our specific feature goals are:

- [ ] To provide a [spec-compliant PDB parser]
- [ ] To provide a [spec-compliant PDBx/mmCIF parser]
- [ ] To interpret all standard residue and atom names according to the [CCD]
- [ ] To provide APIs to process PDB(x)/mmCIF files at the record level
- [ ] To provide APIs to process PDB(x)/mmCIF files at the molecular graph level
  (with coordinates and bonds)
- [ ] To provide clear, descriptive errors when a PDB(x)/mmCIF file does not comply
  with the spec
- [ ] To provide limited configuration support for alternate PDB dialects when
  requested 
- [ ] To provide Rust, Python, and C bindings for the above
- [ ] 🦀⚡🦀 Blazingly Fast 🦀⚡🦀 

Non-goals include:

- Automagic loading of noncompliant PDB files

[CCD]: https://www.wwpdb.org/data/ccd
[spec-compliant PDB parser]: https://www.wwpdb.org/documentation/file-format-content/format33/v3.3.html
[spec-compliant PDBx/mmCIF parser]: https://mmcif.wwpdb.org/

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install pdbmol`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
