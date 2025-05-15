use crate::{context, error, oj_file};
use color_eyre::eyre;

pub mod emitters;
pub mod transformers;

pub type Transformer = fn(
    content_files: &mut Vec<oj_file::OjFile>,
) -> eyre::Result<&mut Vec<oj_file::OjFile>, error::Error>;

pub type Emitter = fn(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error>;
