use crate::{context, error, oj_file};
use color_eyre::eyre;
use pathdiff;
use std::{fs, path};

#[bon::builder]
pub fn prepare_content(
    ctx: &mut context::Context,
) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    if ctx.is_rebuild == true {
        let curr_build_path_string = ctx.build_path.clone();
        let curr_build_path = path::Path::new(&curr_build_path_string);
        if curr_build_path.is_file() {
            let binary_dir = std::env::current_dir()?;
            let curr_build_rel_path = pathdiff::diff_paths(&curr_build_path_string, &binary_dir)
                .unwrap()
                .to_string_lossy()
                .to_string();
            let output_path_slug = curr_build_rel_path
                .replace(".md", ".html")
                .replace(&ctx.build_args.content, &ctx.build_args.output);
            return Ok(vec![oj_file::OjFile {
                abs_input_path: curr_build_path_string.clone(),
                input_path: curr_build_rel_path,
                content: fs::read_to_string(curr_build_path_string.clone())?,
                output_path: output_path_slug,
            }]);
        } else if curr_build_path.is_dir() {
            return Ok(prepare_folder_content().ctx(ctx).call()?);
        }
        return Ok(Vec::new());
    } else {
        return Ok(prepare_folder_content().ctx(ctx).call()?);
    }
}

#[bon::builder]
pub fn prepare_folder_content(
    ctx: &mut context::Context,
) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    let mut res: Vec<oj_file::OjFile> = Vec::new();
    let content_entries = walkdir::WalkDir::new(ctx.build_path.clone())
        .into_iter()
        .filter_entry(|e| {
            !is_hidden_file()
                .entry_file_name(e.file_name().to_string_lossy().to_string())
                .call()
        });
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

        if is_markdown_file().file_path(&entry_path).call() {
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
    return Ok(res);
}

#[bon::builder]
pub fn is_hidden_file(entry_file_name: String) -> bool {
    return entry_file_name.starts_with(".");
}

#[bon::builder]
pub fn is_markdown_file(file_path: &str) -> bool {
    return file_path.ends_with(".md");
}
