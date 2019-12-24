extern crate reqwest;
extern crate serde_json;

extern crate clap;
use clap::{App, Arg};

mod sequence;
use sequence::sequence;

mod processors;
use processors::{csv_processor, json_processor, jsonl_processor};

mod post_processors;
use post_processors::{post_processor, write_on_disk};

mod url_builder;
use url_builder::{csv_url_builder, json_url_builder};

mod download;
use download::{download_in_sequence, DownloadMod, get_total_reccords};
fn main() {
    let matches = App::new("Theses.fr dumper")
        .version("0.1.0")
        .author("eonm <eon.mathis@gmail.fr>")
        .about("Téléchargement des données de theses.fr par lots")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Fichier de sortie")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .possible_values(&["csv", "json", "jsonl"])
                .required(true)
                .help("Format de sortie")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sequence")
                .short("s")
                .long("sequence")
                .value_name("sequence")
                .help("Créer une séquence (début incrément fin)")
                .min_values(3)
                .max_values(3)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debounce")
                .short("d")
                .long("debounce")
                .value_name("debounce")
                .help("Temps d'attente entre les requêtes (ms)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("downmod")
                .short("m")
                .long("mod")
                .value_name("downmod")
                .possible_values(&["keep-alive", "reset"])
                .help("Mode de téléchargement")
                .takes_value(true),
        )
        .get_matches();

    let seq = if let Some(sq) = matches.values_of("sequence") {
        let s: Vec<u32> = sq
            .into_iter()
            .map(|e| e.parse::<u32>().expect("Invalid sequence number"))
            .collect();
        sequence(s[0], s[1], s[2])
    } else {
        sequence(0, 10_000, get_total_reccords().parse::<u32>().expect("Invalid sequence number"))
    };

    let post_process = if let Some(output_file) = matches.value_of("output") {
        write_on_disk(output_file)
    } else {
        post_processor()
    };

    let debounce = match matches.value_of("debounce") {
        Some(d) => Some(d.parse::<u64>().expect("Failed to parse debounce arg")),
        None => None,
    };

    let download_mod = match matches.value_of("downmod") {
        Some("keep-alive") => DownloadMod::KeepAlive,
        Some("reset") => DownloadMod::Reset,
        None | _ => DownloadMod::KeepAlive,
    };

    match matches.value_of("format") {
        Some("json") => download_in_sequence(
            seq,
            json_url_builder,
            json_processor,
            post_process,
            download_mod,
            debounce,
        ),
        Some("jsonl") => download_in_sequence(
            seq,
            json_url_builder,
            jsonl_processor,
            post_process,
            download_mod,
            debounce,
        ),
        Some("csv") => download_in_sequence(
            seq,
            csv_url_builder,
            csv_processor,
            post_process,
            download_mod,
            debounce,
        ),
        _ => panic!("Unknown output format"),
    }
}
