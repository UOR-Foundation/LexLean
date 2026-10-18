use lexlean::surface::module::locate;
use lexlean::surface::core as notation;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (from, to) = (&args[1], &args[2]);
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !name.ends_with(".lex.tex") { continue; }
        let text = std::fs::read_to_string(&path).unwrap();
        let located = locate(&text).unwrap();
        let body = &text[located.body.0..located.body.1];
        let data = notation::data_text(body, "lexlean/core-module/2").unwrap();
        let out = format!("{}\n\\coredata{{{}}}\n{}", &text[..located.body.0].trim_end_matches('\n'), data, &text[located.body.1..]);
        std::fs::write(std::path::Path::new(to).join(name), out).unwrap();
    }
}
