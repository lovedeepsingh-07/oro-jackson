use crate::utils;
use rstest::*;
use std::path;

#[fixture]
fn ctx() -> u32 {
    return 42;
}

#[rstest]
fn test_prepare_content() {}

#[rstest]
fn test_prepare_folder_content() {}

#[rstest]
#[case(path::PathBuf::from(".hidden_file.md"), true)]
#[case(path::PathBuf::from(".this_is_also_a_hidden_file.txt"), true)]
#[case(path::PathBuf::from("/this-/is/also/a/hidden/.file.txt"), true)]
#[case(path::PathBuf::from("this/is/not/a/hidden/file.md"), false)]
#[case(path::PathBuf::from("this.is.not.a.hidden.file.md"), false)]
fn test_is_hidden_file(#[case] input: path::PathBuf, #[case] expected: bool) {
    assert_eq!(expected, utils::is_hidden_file().file_path(input).call());
}

#[rstest]
#[case(path::PathBuf::from("/some_user_name/main.md"), true)]
#[case(path::PathBuf::from("this is also a markdown file.md"), true)]
#[case(path::PathBuf::from("this is not a markdown file"), false)]
#[case(path::PathBuf::from("this is also not a markdown file.txt"), false)]
fn test_is_markdown_file(#[case] input: path::PathBuf, #[case] expected: bool) {
    assert_eq!(expected, utils::is_markdown_file().file_path(input).call());
}
