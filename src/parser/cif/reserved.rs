use winnow::{ascii::Caseless, prelude::*};

pub fn data_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("DATA_").parse_next(input)
}

pub fn loop_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("LOOP_").parse_next(input)
}

pub fn global_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("GLOBAL_").parse_next(input)
}

pub fn save_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("SAVE_").parse_next(input)
}

pub fn stop_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("STOP_").parse_next(input)
}
