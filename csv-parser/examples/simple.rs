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

fn main() {
    let text = generate_csv(10, 1000000, true);
    let text_size = text.len() as f32 / 1024f32 / 1024f32;
    let now = std::time::Instant::now();
    let mut csv_parser = CsvParser::new(',', true);
    csv_parser.parse(&text).unwrap();
    println!("{:.2}mb in {:?}", text_size, now.elapsed());
}
