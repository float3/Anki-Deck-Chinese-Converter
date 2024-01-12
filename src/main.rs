use character_converter::{is_simplified, simplified_to_traditional};
use pinyin_zhuyin::pinyin_to_zhuyin;
use std::borrow::Borrow;
use std::env;
use std::fmt::format;
use std::io::{BufReader, Read};
use std::{fs::File, io::Write};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file = File::open(args.get(1).map(|z| z.as_str()).unwrap_or("test.txt"))
        .expect("failed to open file");

    let both = args.contains(&"both".to_string());
    let zhuyin = !args.contains(&"no-zhuyin".to_string());
    let trad = !args.contains(&"no-trad".to_string());

    let mut buf_reader = BufReader::new(file.try_clone().unwrap());
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents = contents
        .split('\t')
        .map(|x| {
            if is_simplified(x) && trad {
                let trad = simplified_to_traditional(x).to_string();
                if trad != x {
                    formatwrapper(x, &trad, both)
                } else {
                    x.to_string()
                }
            } else {
                x.to_string()
            }
        })
        .map(|x| {
            if zhuyin {
                pinyin_to_zhuyin(&x)
                    .map(|y| formatwrapper(&x, &y, both))
                    .unwrap_or(x)
            } else {
                x
            }
        })
        .collect::<Vec<_>>()
        .join("\t");
    let mut file = File::create("output.txt").unwrap();
    let _ = file.write(contents.as_bytes());
}

fn formatwrapper<S>(a: S, b: S, c: bool) -> String
where
    S: AsRef<str>,
{
    if c {
        format!("{}/{}", a.as_ref(), b.as_ref())
    } else {
        b.as_ref().to_string()
    }
}
