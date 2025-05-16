use crate::{context, error, oj_file};
use color_eyre::eyre;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FolderPageEmitterOptions {
    pub enable: bool,
    pub show_folder_page_children: bool,
}

pub fn folder_page_emitter(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    let _ = content_files;
    let _ = ctx;
    // let entries = walkdir::WalkDir::new(&ctx.build_args.output)
    //     .into_iter()
    //     .filter_entry(|e| !utils::is_hidden_file().entry(e).call());
    // for entry in entries {
    //     let entry = match entry {
    //         Ok(entry) => entry,
    //         Err(e) => {
    //             tracing::warn!(
    //                 "unable to access file from content folder, error: {:#?}",
    //                 e.to_string()
    //             );
    //             continue;
    //         }
    //     };
    //     if !entry.path().is_dir() {
    //         continue;
    //     }
    //     let entry_path_string = entry.path().display().to_string();
    //     // build_curr_folder_index_file()
    //     //     .output_folder_string(ctx.build_args.output.clone())
    //     //     .input_folder_string(entry_path_string.clone())
    //     //     .call()?;
    // }
    return Ok(());
}
//
// #[bon::builder]
// pub fn build_curr_folder_index_file(
//     output_folder_string: String,
//     input_folder_string: String,
// ) -> eyre::Result<(), error::Error> {
//     if input_folder_string == output_folder_string {
//         return Ok(());
//     }
//
//     let input_folder_path = path::Path::new(&input_folder_string);
//
//     if !input_folder_path.is_dir() {
//         return Err(error::Error::InvalidInput(format!(
//             "provided input path is not a valid file or a directory, input: {}",
//             input_folder_string
//         )))?;
//     }
//     let ignores = Vec::from([".git", ".obsidian", "index"]);
//     let mut curr_folder_subfiles: Vec<FolderPageChildLink> = Vec::new();
//
//     let folder_entries = fs::read_dir(input_folder_path)?;
//
//     for folder_entry in folder_entries {
//         let folder_entry = match folder_entry {
//             Ok(folder_entry) => folder_entry,
//             Err(e) => {
//                 tracing::warn!(
//                     "unable to access file from content folder, error: {:#?}",
//                     e.to_string()
//                 );
//                 continue;
//             }
//         };
//         let entry_path = folder_entry.path();
//         let entry_name = folder_entry.file_name().to_string_lossy().to_string();
//
//         if ignores.contains(&entry_name.as_str()) {
//             continue;
//         }
//
//         let href = entry_path
//             .to_string_lossy()
//             .to_string()
//             .replace(&output_folder_string, "")
//             .replace(".html", "");
//
//         if entry_path.is_dir() || entry_path.is_file() {
//             curr_folder_subfiles.push(
//                 FolderPageChildLink::builder()
//                     .name(entry_name.replace(".html", ""))
//                     .href(href)
//                     .build(),
//             );
//         } else {
//             return Err(error::Error::InvalidInput(format!(
//                 "provided folder entry is neither a file nor a directory, input: {}",
//                 input_folder_string
//             )))?;
//         }
//     }
//
//     let index_html = generate_html_for_folder_page()
//         .subfiles(curr_folder_subfiles)
//         .call()?;
//
//     let index_file_location = format!(
//         "{}/index.html",
//         input_folder_path.to_string_lossy().to_string()
//     );
//
//     fs::write(index_file_location, index_html)?;
//
//     return Ok(());
// }
