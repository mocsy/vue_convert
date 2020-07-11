use askama::Template;
use select::document::Document;
use select::predicate::Name;
use std::{env, fs};
use vue_convert::*;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    let files: Vec<String> = args.drain(1..).collect();
    for file_name in files {
        match fs::File::open(file_name.clone()) {
            Ok(read) => match Document::from_read(read) {
                Ok(doc) => {
                    println!("{:?}", file_name);
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
                    let (comp_name, out) = create_vue_paths(&file_name);

                    let sfc_template = SfcTemplate {
                        file_name: &file_name,
                        body: &body,
                        comp_name: &comp_name,
                        css_links: &links.join(""),
                    };

                    let cont = sfc_template.render().unwrap();
                    let _res = fs::write(out, cont);
                }
                Err(err) => println!("Error: {}", err),
            },
            Err(err) => println!("Error: {}", err),
        };
    }
    Ok(())
}
