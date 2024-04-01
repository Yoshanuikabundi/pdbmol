use std::collections::HashMap;

use super::charsets::nonblank1;
use super::reserved;
use super::values::{noteol_value, tag, whitespace_value};
use super::Value;
use super::{comments, whitespace};
use winnow::combinator::{alt, opt, preceded, repeat};
use winnow::prelude::*;

/// This parser must only be called immediately after a non-EOL character
/// There is no need for an equivalent parser for after an EOL
fn noteol_loop_body<'s>(input: &mut &'s str) -> PResult<Vec<Value<'s>>> {
    let init = noteol_value.parse_next(input)?;
    let mut values: Vec<Value<'s>> =
        repeat(0.., whitespace_value.map(|(_, value)| value)).parse_next(input)?;
    // TODO: Avoid shifting all elements by one
    values.insert(0, init);
    Ok(values)
}

fn loop_header<'s>(input: &mut &'s str) -> PResult<Vec<&'s str>> {
    preceded(reserved::loop_, repeat(1.., preceded(whitespace, tag))).parse_next(input)
}

fn data_items<'s>(input: &mut &'s str) -> PResult<HashMap<&'s str, Value<'s>>> {
    alt((
        (tag, whitespace_value).map(|(tag, (_, value))| HashMap::from([(tag, value)])),
        (loop_header, noteol_loop_body)
            .map(|(tags, values)| HashMap::from_iter(tags.into_iter().zip(values))),
    ))
    .parse_next(input)
}

fn save_frame_heading<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(reserved::save_, nonblank1).parse_next(input)
}

fn save_frame<'s>(input: &mut &'s str) -> PResult<(&'s str, HashMap<&'s str, Value<'s>>)> {
    (
        save_frame_heading,
        repeat(1.., preceded(whitespace, data_items)).fold(HashMap::new, |mut acc, data| {
            acc.extend(data);
            acc
        }),
        whitespace,
        reserved::save_,
    )
        .map(|(head, map, _, _)| (head, map))
        .parse_next(input)
}

fn datablock_heading<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(reserved::data_, nonblank1).parse_next(input)
}

pub enum DataBlockItem<'s> {
    DataItems(HashMap<&'s str, Value<'s>>),
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
                    save_frame.map(DataBlockItem::SaveFrame),
                )),
            ),
        ),
    )
        .parse_next(input)
}

pub fn cif<'s>(input: &mut &'s str) -> PResult<Vec<(&'s str, Vec<DataBlockItem<'s>>)>> {
    (preceded(
        (opt(comments), opt(whitespace)),
        opt((
            datablock,
            repeat(0.., preceded(whitespace, datablock)),
            opt(whitespace),
        ))
        .map(|v: Option<(_, Vec<_>, _)>| match v {
            Some((block, mut blocks, _)) => {
                // TODO: Avoid shifting all elements by one
                blocks.insert(0, block);
                blocks
            }
            None => Vec::new(),
        }),
    ))
    .parse_next(input)
}
