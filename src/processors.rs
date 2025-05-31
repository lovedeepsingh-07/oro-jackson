use crate::{context, error, oj_file, utils};
use bon;
use color_eyre::eyre;

#[bon::builder]
pub fn parse(ctx: &context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    let mut res = utils::prepare_content().ctx(ctx).call()?;
    for tfmr in &ctx.transformer_plugins {
        res = tfmr(ctx, &mut res)?.to_vec();
    }
    return Ok(res);
}

#[bon::builder]
pub fn emit(
    ctx: &context::Context,
    parsed_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    for emtr in &ctx.emitter_plugins {
        emtr(ctx, parsed_files)?;
    }
    return Ok(());
}
