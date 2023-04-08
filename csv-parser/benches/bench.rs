use criterion::{black_box, criterion_group, criterion_main, Criterion};

use csv_parser::CsvParser;

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
        let row_number_str = (row % 1000).to_string();
        for col in 0..cols {
            if quotes {
                csv.push('"');
            }

            csv.push_str(&row_number_str);
            let col_number_str = (col % 1000).to_string();
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
    let text_size = text.len() as f32 / 1024f32;
    c.bench_function(
        format!("small-no-quotes({:.1}kb)", text_size).as_str(),
        |b| b.iter(|| parse(&text)),
    );
}
fn small_quotes(c: &mut Criterion) {
    let text = generate_csv(10, 1000, true);
    let text_size = text.len() as f32 / 1024f32;
    c.bench_function(
        format!("small-with-quotes({:.1}kb)", text_size).as_str(),
        |b| b.iter(|| parse(&text)),
    );
}

fn big_no_quotes(c: &mut Criterion) {
    let text = generate_csv(10, 1000000, false);
    let text_size = text.len() as f32 / 1024f32;
    c.bench_function(format!("big-no-quotes({:.1}kb)", text_size).as_str(), |b| {
        b.iter(|| parse(&text))
    });
}
fn big_quotes(c: &mut Criterion) {
    let text = generate_csv(10, 1000000, true);
    let text_size = text.len() as f32 / 1024f32;
    c.bench_function(
        format!("big-with-quotes({:.1}kb)", text_size).as_str(),
        |b| b.iter(|| parse(&text)),
    );
}

fn gigantic_no_quotes(c: &mut Criterion) {
    let mut group = c.benchmark_group("lower-samples");
    group.sample_size(10);
    let text = generate_csv(100, 1000000, false);
    let text_size = text.len() as f32 / 1024f32 / 1024f32;
    group.bench_function(
        format!("gigantic-no-quotes({:.1}mb)", text_size).as_str(),
        |b| b.iter(|| parse(&text)),
    );
}
fn gigantic_quotes(c: &mut Criterion) {
    let mut group = c.benchmark_group("lower-samples");
    group.sample_size(10);
    let text = generate_csv(100, 1000000, true);
    let text_size = text.len() as f32 / 1024f32 / 1024f32;
    group.bench_function(
        format!("gigantic-with-quotes({:.1}mb)", text_size).as_str(),
        |b| b.iter(|| parse(&text)),
    );
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
