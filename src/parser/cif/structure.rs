use std::collections::HashMap;

use super::charsets::nonblank1;
use super::reserved;
use super::values::{noteol_value, tag, whitespace_value};
use super::Value;
use super::{comments, whitespace};
use winnow::combinator::{alt, opt, preceded, repeat, separated};
use winnow::error::StrContext;
use winnow::prelude::*;
use winnow::stream::Stream;

fn loop_header<'s>(input: &mut &'s str) -> PResult<Vec<&'s str>> {
    preceded(reserved::loop_, repeat(1.., preceded(whitespace, tag)))
        .context(StrContext::Label("loop header"))
        .parse_next(input)
}

fn data_items_loop<'s>(input: &mut &'s str) -> PResult<HashMap<&'s str, Vec<Value<'s>>>> {
    let tags = loop_header.parse_next(input)?;
    repeat::<_, _, Vec<_>, _, _>(
        0..,
        repeat::<_, _, Vec<_>, _, _>(tags.len(), whitespace_value),
    )
    .map(|rows| {
        let mut values: Vec<_> = tags
            .iter()
            .map(|_| Vec::with_capacity(rows.len()))
            .collect();
        for row in rows {
            for (i, value) in row.into_iter().enumerate() {
                values[i].push(value)
            }
        }
        HashMap::from_iter(tags.iter().cloned().zip(values))
    })
    .parse_next(input)
}

fn data_items<'s>(input: &mut &'s str) -> PResult<HashMap<&'s str, Value<'s>>> {
    separated::<_, _, Vec<_>, _, _, _, _>(1.., (tag, whitespace_value), whitespace)
        .map(HashMap::from_iter)
        .context(StrContext::Label("data items"))
        .parse_next(input)
}

// fn save_frame_heading<'s>(input: &mut &'s str) -> PResult<&'s str> {
//     preceded(reserved::save_, nonblank1).parse_next(input)
// }

// fn save_frame<'s>(input: &mut &'s str) -> PResult<(&'s str, HashMap<&'s str, Value<'s>>)> {
//     (
//         save_frame_heading,
//         repeat(1.., preceded(whitespace, data_items)).fold(HashMap::new, |mut acc, data| {
//             acc.extend(data);
//             acc
//         }),
//         whitespace,
//         reserved::save_,
//     )
//         .map(|(head, map, _, _)| (head, map))
//         .context(StrContext::Label("save frame"))
//         .parse_next(input)
// }

fn datablock_heading<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(reserved::data_, nonblank1).parse_next(input)
}

#[derive(Debug, Clone)]
pub enum DataBlockItem<'s> {
    DataItems(HashMap<&'s str, Value<'s>>),
    Table(HashMap<&'s str, Vec<Value<'s>>>),
    SaveFrame((&'s str, HashMap<&'s str, Value<'s>>)),
}

fn datablock<'s>(input: &mut &'s str) -> PResult<(&'s str, Vec<DataBlockItem<'s>>)> {
    (
        datablock_heading,
        repeat(
            0..,
            preceded(
                whitespace,
                alt((
                    data_items.map(DataBlockItem::DataItems),
                    data_items_loop.map(DataBlockItem::Table),
                    // save_frame.map(DataBlockItem::SaveFrame),
                )),
            ),
        ),
    )
        .context(StrContext::Label("data block"))
        .parse_next(input)
}

pub fn cif<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, Vec<DataBlockItem<'s>>)>> {
    (
        opt(comments).context(StrContext::Label("initial comments")),
        opt(whitespace),
        separated(0.., datablock, whitespace).context(StrContext::Label("data blocks")),
        opt(whitespace),
    )
        .map(|(_, _, datablocks, _)| datablocks)
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_header() {
        let mut stream = r##"loop_
        _pdbx_chem_comp_audit.comp_id
        _pdbx_chem_comp_audit.action_type
        _pdbx_chem_comp_audit.date
        _pdbx_chem_comp_audit.processing_site
        "##;
        let output = loop_header.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(vec![
                "pdbx_chem_comp_audit.comp_id",
                "pdbx_chem_comp_audit.action_type",
                "pdbx_chem_comp_audit.date",
                "pdbx_chem_comp_audit.processing_site"
            ])
        );
    }

    #[test]
    fn test_loop() {
        let mut stream = r##"loop_
        _pdbx_chem_comp_audit.comp_id
        _pdbx_chem_comp_audit.action_type
        _pdbx_chem_comp_audit.date
        _pdbx_chem_comp_audit.processing_site
        PHE "Create component"  1999-07-08 EBI
        PHE "Modify descriptor" 2011-06-04 RCSB
        PHE "Modify backbone"   2023-11-03 PDBE
        #
        "##;

        let output = data_items_loop.parse_next(&mut stream);
        assert_eq!(
            output,
            Ok(HashMap::from_iter(
                [
                    (
                        "pdbx_chem_comp_audit.comp_id",
                        vec![
                            Value::String("PHE"),
                            Value::String("PHE"),
                            Value::String("PHE")
                        ]
                    ),
                    (
                        "pdbx_chem_comp_audit.action_type",
                        vec![
                            Value::String("Create component"),
                            Value::String("Modify descriptor"),
                            Value::String("Modify backbone")
                        ]
                    ),
                    (
                        "pdbx_chem_comp_audit.date",
                        vec![
                            Value::String("1999-07-08"),
                            Value::String("2011-06-04"),
                            Value::String("2023-11-03")
                        ]
                    ),
                    (
                        "pdbx_chem_comp_audit.processing_site",
                        vec![
                            Value::String("EBI"),
                            Value::String("RCSB"),
                            Value::String("PDBE")
                        ]
                    )
                ]
                .into_iter()
            ))
        );
    }
}
