fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read");
    let out = lexlean::sdk::to_notation(&text).unwrap_or_else(|e| panic!("{e:?}"));
    std::fs::write(&args[2], out).expect("write");
}
