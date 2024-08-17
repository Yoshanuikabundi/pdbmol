#[test]
fn reads_and_writes_standard_pdb() {
    for pdbfile in ["1po0.pdb", "2l7v.pdb"] {
        let path = format!("../../data/{pdbfile}");
        let pdb_str = std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("{path:?}"));
        let records = pdbmol_pdb::parse(&pdb_str);

        let errors: Vec<_> = records
            .filter_map(Result::err)
            .collect();
        assert!(
            errors.is_empty(),
            "errors encountered while parsing pdb records: {errors:#?}"
        );

        let mut lines_iter = pdb_str.lines();
        for written_line in records.into_iter().flat_map(|r| {
            r.unwrap()
                .to_string()
                .lines()
                .map(String::from)
                .collect::<Vec<_>>()
        }) {
            eprintln!("         {written_line:?}");
            loop {
                match lines_iter.next() {
                    Some(read_line) if read_line.trim_end() == written_line.trim_end() => break,
                    Some(read_line) => {
                        eprintln!("Skipping {read_line:?}");
                    }
                    None => {
                        panic!("Written line {written_line:?} does not match any line in {path:?}");
                    }
                }
            }
        }
        for read_line in lines_iter {
            eprintln!("Skipping {read_line:?}");
        }
    }
}
