fn main() {
    let package = "tree-sitter-sql";
    let dir = format!("../{}/src", package);
    let source_file = format!("{}/parser.c", dir);

    cc::Build::new()
        .file(source_file)
        .include(dir)
        .compile(package);
}
