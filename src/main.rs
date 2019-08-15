use heck::CamelCase;
use select::document::Document;
use select::predicate::Name;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    if let Ok(read) = fs::File::open(filename) {
        let doc = Document::from_read(read).unwrap();
        let mut links = Vec::new();
        doc.find(Name("link"))
            .filter_map(|n| n.attr("href"))
            .for_each(|x| links.push(format!("@import '{}';\n", x)));
        let body = doc
            .find(Name("body"))
            .into_selection()
            .first()
            .unwrap()
            .html();
        let body = body.replace("<body>", "<div>");
        let body = body.replace("</body>", "</div>");
        let comp_name = strip_extension(filename.clone()).to_camel_case();
        let mut out = comp_name.clone();
        out.push_str(".vue");
        let cont = format!(
            r#"// https://github.com/mocsy/vue_convert from {}
<template>
{}
</template>

<script lang="ts">
import Vue from 'vue';

export default Vue.extend({{
  name: '{}'
}});
</script>

<style lang="scss">
{}
</style>
"#,
            filename, body, comp_name, links.join("")
        );
        return fs::write(out, cont);
    };
    Ok(())
}

fn strip_extension(name: String) -> String {
    if let Some(file_ext) = std::path::Path::new(&name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        return name.replace(file_ext, "");
    }
    name
}

fn _change_extension(name: String, ext: &str) -> String {
    if let Some(file_ext) = std::path::Path::new(&name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        return name.replace(file_ext, ext);
    };
    let mut s = name.to_owned();
    s.push_str(".");
    s.push_str(ext);
    s
}
