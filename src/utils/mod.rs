use crate::{context, error, oj_file};
use color_eyre::eyre;
use pathdiff;
use serde_yaml;
use std::{fs, path};

#[cfg(test)]
pub mod tests;

#[bon::builder]
pub fn prepare_content(ctx: &context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    if ctx.is_rebuild == true {
        let curr_build_path_string = ctx.build_path.clone();
        let curr_build_path = path::Path::new(&curr_build_path_string);
        if curr_build_path.is_file() {
            let binary_dir = std::env::current_dir()?;

            let difference_of_paths = pathdiff::diff_paths(&curr_build_path_string, &binary_dir)
                .ok_or_else(|| error::Error::PathdiffError)?;

            let curr_build_rel_path = difference_of_paths.to_string_lossy().to_string();
            let output_path_slug = curr_build_rel_path.replace(".md", ".html").replace(
                &ctx.build_args.content.to_string_lossy().to_string(),
                &ctx.build_args.output.to_string_lossy().to_string(),
            );
            return Ok(vec![oj_file::OjFile {
                frontmatter: oj_file::OjFrontmatter::Yaml(serde_yaml::Value::Null),
                abs_input_path: path::PathBuf::from(curr_build_path_string.clone()),
                input_path: path::PathBuf::from(curr_build_rel_path),
                output_path: path::PathBuf::from(output_path_slug),
                content: fs::read_to_string(curr_build_path_string.clone())?,
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
    ctx: &context::Context,
) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    let mut res: Vec<oj_file::OjFile> = Vec::new();
    let content_entries = walkdir::WalkDir::new(ctx.build_path.clone())
        .into_iter()
        .filter_entry(|e| !is_hidden_file().file_path(e.path().into()).call());
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
        let entry_path = entry.path();
        let canon_entry_path = fs::canonicalize(&entry_path)?;
        let abs_entry_path = canon_entry_path.to_string_lossy().to_string();

        if is_markdown_file().file_path(entry_path.into()).call() {
            let file_content = fs::read_to_string(&entry_path)?;
            let entry_output_path_slug = entry_path
                .to_string_lossy()
                .to_string()
                .replace(".md", ".html")
                .replace(
                    &ctx.build_args.content.to_string_lossy().to_string(),
                    &ctx.build_args.output.to_string_lossy().to_string(),
                );

            res.push(oj_file::OjFile {
                frontmatter: oj_file::OjFrontmatter::Yaml(serde_yaml::Value::Null),
                input_path: path::PathBuf::from(entry_path),
                abs_input_path: path::PathBuf::from(abs_entry_path),
                output_path: path::PathBuf::from(entry_output_path_slug),
                content: file_content,
            })
        }
    }
    return Ok(res);
}

#[bon::builder]
pub fn is_hidden_file(file_path: path::PathBuf) -> bool {
    // if any problem occurs in this process, we consider that the file is hidden in order to
    // prevent any faulty or badly named file to go into the content vector
    return match file_path.file_name() {
        Some(value) => value,
        None => {
            return true;
        }
    }
    .to_string_lossy()
    .to_string()
    .starts_with(".");
}

#[bon::builder]
pub fn is_markdown_file(file_path: path::PathBuf) -> bool {
    return file_path.to_string_lossy().to_string().ends_with(".md");
}
