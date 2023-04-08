use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rs_csv_parse::CsvParser;

fn generate_csv(cols: u32, rows: u32, quotes: bool) -> String {
    let mut csv = String::new();
    for col in 0..cols {
        if quotes {
            csv.push('"');
        }

        csv.push_str("col");
        let col_number_str = col.to_string();
        csv.push_str(&col_number_str);

        if quotes {
            csv.push('"');
        }
        if col <= cols - 2 {
            csv.push(',');
        }
    }
    csv.push('\n');
    for row in 0..rows {
        let row_number_str = row.to_string();
        for col in 0..cols {
            if quotes {
                csv.push('"');
            }

            csv.push_str(&row_number_str);
            let col_number_str = col.to_string();
            csv.push_str(&col_number_str);

            if quotes {
                csv.push('"');
            }
            if col <= cols - 2 {
                csv.push(',');
            }
        }
        csv.push('\n');
    }
    csv
}

fn parse(text: &String) {
    let mut csv_parser = CsvParser::new(',', true);
    csv_parser.parse(black_box(&text)).unwrap();
}

fn small_no_quotes(c: &mut Criterion) {
    let text = generate_csv(10, 1000, false);
    c.bench_function("small-no-quotes", |b| b.iter(|| parse(&text)));
}
fn small_quotes(c: &mut Criterion) {
    let text = generate_csv(10, 1000, true);
    c.bench_function("small-with-quotes", |b| b.iter(|| parse(&text)));
}

fn big_no_quotes(c: &mut Criterion) {
    let text = generate_csv(100, 10000, false);
    c.bench_function("big-no-quotes", |b| b.iter(|| parse(&text)));
}
fn big_quotes(c: &mut Criterion) {
    let text = generate_csv(100, 10000, true);
    c.bench_function("big-with-quotes", |b| b.iter(|| parse(&text)));
}

fn gigantic_no_quotes(c: &mut Criterion) {
    let text = generate_csv(100, 1000000, false);
    c.bench_function("gigantic-no-quotes", |b| b.iter(|| parse(&text)));
}
fn gigantic_quotes(c: &mut Criterion) {
    let text = generate_csv(100, 1000000, true);
    c.bench_function("gigantic-with-quotes", |b| b.iter(|| parse(&text)));
}

criterion_group!(
    benches,
    small_no_quotes,
    small_quotes,
    big_no_quotes,
    big_quotes,
    gigantic_no_quotes,
    gigantic_quotes
);
criterion_main!(benches);
