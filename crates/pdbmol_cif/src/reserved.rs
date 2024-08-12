use winnow::{ascii::Caseless, error::StrContext, prelude::*};

pub fn data_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("DATA_")
        .context(StrContext::Label("DATA_"))
        .parse_next(input)
}

pub fn loop_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("LOOP_")
        .context(StrContext::Label("LOOP_"))
        .parse_next(input)
}

pub fn global_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("GLOBAL_")
        .context(StrContext::Label("GLOBAL_"))
        .parse_next(input)
}

pub fn save_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("SAVE_")
        .context(StrContext::Label("SAVE_"))
        .parse_next(input)
}

pub fn stop_<'s>(input: &mut &'s str) -> PResult<&'s str> {
    Caseless("STOP_")
        .context(StrContext::Label("STOP_"))
        .parse_next(input)
}
