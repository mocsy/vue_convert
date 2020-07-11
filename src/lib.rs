#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use askama::Template;
use heck::CamelCase;
use std::path::Path;

#[derive(Template)]
#[template(path = "sfc.vue.j2", escape = "none")]
pub struct SfcTemplate<'a> {
    pub file_name: &'a str,
    pub body: &'a str,
    pub comp_name: &'a str,
    pub css_links: &'a str,
}

#[must_use]
pub fn strip_extension(name: &str) -> String {
    if let Some(file_ext) = std::path::Path::new(name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        return name.replace(file_ext, "");
    }
    name.to_owned()
}
#[must_use]
pub fn strip_dir(name: &str) -> String {
    if let Some(file_stem) = std::path::Path::new(name)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
    {
        return file_stem.to_owned();
    }
    name.to_owned()
}
#[must_use]
pub fn create_vue_paths(name: &str) -> (String, String) {
    let pth = Path::new(name);

    let base = strip_extension(name);
    let stem = strip_dir(&base);

    let comp_name = stem.to_camel_case();
    if let Some(out_path) = pth
        .with_file_name(comp_name.clone())
        .with_extension("vue")
        .into_os_string()
        .to_str()
    {
        (comp_name, out_path.to_owned())
    } else {
        (comp_name, change_extension(name, "vue"))
    }
}
#[must_use]
pub fn change_extension(name: &str, ext: &str) -> String {
    if let Some(file_ext) = std::path::Path::new(name)
        .with_extension(ext)
        .into_os_string()
        .to_str()
    {
        return file_ext.to_owned();
    };
    let mut s = name.to_owned();
    s.push_str(".");
    s.push_str(ext);
    s
}
