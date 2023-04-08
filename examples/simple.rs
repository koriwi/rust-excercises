use rs_csv_parse::CsvParser;
fn main() {
    let text = std::fs::read_to_string("./small.csv").expect("no file");
    let now = std::time::Instant::now();
    let mut csv_parser = CsvParser::new(',', true);
    csv_parser.parse(&text).unwrap();
    println!("time: {:?}", now.elapsed());
}
