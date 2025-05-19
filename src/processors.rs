use crate::{context, error, oj_file, utils};
use bon;
use color_eyre::eyre;
use std::fs;
use tracing;
use walkdir;

#[bon::builder]
pub fn parse(ctx: &mut context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    let mut res: Vec<oj_file::OjFile> = Vec::new();
    let content_entries = walkdir::WalkDir::new(ctx.build_args.content.clone())
        .into_iter()
        .filter_entry(|e| !utils::is_hidden_file().entry(e).call());
    for entry in content_entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                tracing::warn!(
                    "unable to access file from content folder, error: {:#?}",
                    e.to_string()
                );
                continue;
            }
        };
        let entry_path = entry.path().display().to_string();
        let abs_entry_path = fs::canonicalize(&entry_path)?.to_string_lossy().to_string();

        if utils::is_markdown_file().file_path(&entry_path).call() {
            let file_content = fs::read_to_string(&entry_path)?;
            let entry_output_path_slug = entry_path
                .replace(".md", ".html")
                .replace(&ctx.build_args.content, &ctx.build_args.output);

            res.push(oj_file::OjFile {
                input_path: entry_path,
                abs_input_path: abs_entry_path,
                output_path: entry_output_path_slug,
                content: file_content,
            })
        }
    }
    for tfmr in &ctx.transformer_plugins {
        res = tfmr(&mut res)?.to_vec();
    }
    return Ok(res);
}

#[bon::builder]
pub fn emit(
    ctx: &mut context::Context,
    parsed_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    for emtr in &ctx.emitter_plugins {
        emtr(ctx, parsed_files)?;
    }
    return Ok(());
}
