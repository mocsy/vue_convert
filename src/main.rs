use heck::CamelCase;
use select::document::Document;
use select::predicate::Name;
use std::{env, fs, path::Path};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let files: Vec<String> = args.clone().drain(1..).collect();
    for filename in files {
        match fs::File::open(filename.clone()) {
            Ok(read) => match Document::from_read(read) {
                Ok(doc) => {
                    println!("{:?}", filename);
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
                    let(comp_name, out) = create_vue_paths(&filename);

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
                        filename,
                        body,
                        comp_name,
                        links.join("")
                    );
                    let _res = fs::write(out, cont);
                }
                Err(err) => println!("Error: {}", err),
            },
            Err(err) => println!("Error: {}", err),
        };
    }
    Ok(())
}

fn strip_extension(name: &String) -> String {
    if let Some(file_ext) = std::path::Path::new(name)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        return name.replace(file_ext, "");
    }
    name.clone()
}

fn strip_dir(name: &String) -> String {
    if let Some(file_stem) = std::path::Path::new(name)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
    {
        return file_stem.to_owned();
    }
    name.clone()
}

fn create_vue_paths(name: &String) -> (String, String) {
    let pth = Path::new(name);

    let base = strip_extension(name);
    let stem = strip_dir(&base);

    let comp_name = stem.to_camel_case();
    if let Some(out_path) = pth.with_file_name(comp_name.clone())
    .with_extension("vue")
    .into_os_string().to_str() {
    return ( comp_name, out_path.to_owned() );
    } else {
        return ( comp_name, change_extension(name, "vue") );
    }
}

fn change_extension(name: &String, ext: &str) -> String {
    if let Some(file_ext) = std::path::Path::new(name)
        .with_extension(ext)
        .into_os_string().to_str()
    {
        return file_ext.to_owned();
    };
    let mut s = name.to_owned();
    s.push_str(".");
    s.push_str(ext);
    s
}
