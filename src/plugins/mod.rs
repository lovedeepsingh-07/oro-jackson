use crate::{context, error, oj_file};
use color_eyre::eyre;

pub mod emitters;
pub mod transformers;

pub type Transformer = for<'a> fn(
    ctx: &'a context::Context,
    content_files: &'a mut Vec<oj_file::OjFile>,
) -> eyre::Result<&'a mut Vec<oj_file::OjFile>, error::Error>;

pub type Emitter = fn(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error>;
