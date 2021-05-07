use clap::{value_t, App, Arg};
use phf::phf_map;

struct Unicode {
    name: &'static str,
    number: &'static str,
}

static UNICODE_MAP: phf::Map<&'static str, Unicode> = phf_map! {
    "a" => Unicode { name: "Cyrillic Small Letter A", number: "\u{0430}" },
    "c" => Unicode { name: "Greek Lunate Sigma Symbol", number: "\u{03F2}"},
    "e" => Unicode { name: "Cyrillic Small Letter Ie", number: "\u{0435}"},
    "o" => Unicode { name: "Cyrillic Small Letter O", number: "\u{043E}"},
    "p" => Unicode { name: "Cyrillic Small Letter Er", number: "\u{0440}"},
    "s" => Unicode { name: "Cyrillic Small Letter Dze", number: "\u{0455}"},
    "d" => Unicode { name: "Cyrillic Small Letter Komi De", number: "\u{0501}"},
    "q" => Unicode { name: "Cyrillic Small Letter Qa", number: "\u{051B}"},
    "w" => Unicode { name: "Cyrillic Small Letter We", number: "\u{051D}" },
};

fn show_evil_url(chars: Vec<&'static str>, url: &str, end: &str) {
    let mut chars_replaced = Vec::new();
    let mut unicodes = Vec::new();
    let mut evil_url = url.to_string();
    for ch in chars {
        if let Some(unicode) = UNICODE_MAP.get(&ch) {
            chars_replaced.push(ch);
            unicodes.push(unicode);
            evil_url = evil_url.replace(ch, unicode.number);
        }
    }

    println!(
        "[*] Char replaced: {}",
        chars_replaced
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "[*] Using Unicode: {}",
        unicodes
            .iter()
            .map(|u| u.name)
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "[*] Unicode number: {}",
        unicodes
            .iter()
            .map(|u| u.number)
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("Evil url: {}.{}", evil_url, end);
    println!("-------------------------------");
}

fn generate(full_url: &str) {
    let v: Vec<&str> = full_url.split('.').collect();
    let url = v[0].to_lowercase();
    let end = v[1].to_lowercase();
    println!("{}.{}", v[0], v[1]);
    let mut chars: Vec<&str> = Vec::new();
    for ch in UNICODE_MAP.keys() {
        if url.contains(ch) {
            chars.push(ch);
            show_evil_url(vec![ch], &url, &end);
        }
    }
    if chars.len() > 1 {
        show_evil_url(chars, &url, &end);
    }
}

fn main() {
    let app = App::new("evil_url").arg(
        Arg::with_name("DOMAIN")
            .help("Domain name with termination (example.com)")
            .index(1)
            .multiple(false)
            .required(true),
    );

    let matches = match app.get_matches_from_safe(std::env::args_os().into_iter()) {
        Ok(m) => m,
        Err(ref e)
            if e.kind == clap::ErrorKind::HelpDisplayed
                || e.kind == clap::ErrorKind::VersionDisplayed =>
        {
            println!("{}", e);
            std::process::exit(0);
        }
        Err(f) => {
            eprintln!("{}", f);
            std::process::exit(1)
        }
    };

    let domain = value_t!(matches, "DOMAIN", String).unwrap_or_else(|e| e.exit());
    generate(&domain);
}
