use character_converter::{is_simplified, simplified_to_traditional};
use csv::{ReaderBuilder, WriterBuilder};
use pinyin_zhuyin::pinyin_to_zhuyin;
use std::{
    env,
    fs::File,
    io::{Read, Write},
};

fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let src = args.get(1).map(|s| s.as_str()).unwrap_or("test.txt");
    let both = args.contains(&"both".to_string());
    let zhuyin = !args.contains(&"no-zhuyin".to_string());
    let trad = !args.contains(&"no-trad".to_string());

    let mut raw = String::new();
    File::open(src)?.read_to_string(&mut raw)?;
    let mut lines = raw.lines();
    let preamble: Vec<&str> = lines.by_ref().take(6).collect();
    let tsv_body: String = lines.collect::<Vec<_>>().join("\n");

    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(tsv_body.as_bytes());

    let mut wtr = WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(Vec::new());

    for rec in rdr.records() {
        let rec = rec?;
        let mut row: Vec<String> = rec.iter().map(|s| s.to_owned()).collect();

        if let Some(col) = row.get_mut(6) {
            *col = transform(col, both, zhuyin, trad);
        }
        wtr.write_record(&row)?;
    }

    let mut out = File::create("output.txt")?;
    for line in preamble {
        writeln!(out, "{line}")?;
    }
    out.write_all(&wtr.into_inner()?)?;
    Ok(())
}

fn transform(s: &str, both: bool, zhuyin: bool, trad: bool) -> String {
    let mut out = s.to_owned();
    if trad && is_simplified(s) {
        let t = simplified_to_traditional(s).to_string();
        out = if both { format!("{s}/{t}") } else { t };
    }
    if zhuyin && let Some(z) = pinyin_to_zhuyin(&out) {
        out = if both { format!("{out}/{z}") } else { z };
    }
    out
}
