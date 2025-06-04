use crate::{context, error, oj_file};
use color_eyre::eyre;
use serde_yaml;

#[cfg(test)]
pub mod tests;

#[bon::builder]
pub fn prepare_content(ctx: &context::Context) -> eyre::Result<Vec<oj_file::OjFile>, error::Error> {
    // TODO: reimplement live-reloading using `vfs`
    if ctx.is_rebuild == true {
        tracing::warn!("WARN: not yet working");
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
    let content_entries = ctx
        .build_path
        .walk_dir()?
        .filter_map(Result::ok)
        .filter(|e| {
            return !is_hidden_file().file_path(e).call();
        });
    for entry in content_entries {
        if is_markdown_file().file_path(&entry).call() {
            let file_content = entry.read_to_string()?;
            let entry_output_path = ctx
                .build_args
                .output
                .join(entry.as_str().replace(".md", ".html"))?;

            res.push(oj_file::OjFile {
                frontmatter: oj_file::OjFrontmatter::Yaml(serde_yaml::Value::Null),
                input_path: entry,
                output_path: entry_output_path,
                content: file_content,
            })
        }
    }
    return Ok(res);
}

#[bon::builder]
pub fn is_hidden_file(file_path: &vfs::VfsPath) -> bool {
    return file_path.filename().starts_with(".");
}

#[bon::builder]
pub fn is_markdown_file(file_path: &vfs::VfsPath) -> bool {
    return match file_path.extension() {
        Some(value) => value,
        None => {
            return false;
        }
    } == "md";
}
