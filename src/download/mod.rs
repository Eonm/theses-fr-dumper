use std::thread;
use std::time::Duration;
use serde_json;
use indicatif::ProgressBar;

#[derive(Debug, PartialEq)]
pub enum DownloadMod {
    KeepAlive,
    Reset,
}

pub fn download_in_sequence(
    sequence: Vec<(u32, u32)>,
    url_builder: impl Fn(&(u32, u32)) -> String,
    processor: impl Fn(String, &usize, &(u32, u32), &usize) -> String,
    post_processor: impl Fn(String),
    download_mod: DownloadMod,
    debounce: Option<u64>,
) {
    let bar_size = (sequence.last().unwrap().0 + sequence.last().unwrap().1) as u64;
    let pb = ProgressBar::new(bar_size);

    let keep_alive_client = reqwest::Client::new();
        
    for (index, step) in sequence.iter().enumerate() {
        let client = if download_mod == DownloadMod::Reset {
            reqwest::Client::new()
        } else {
            keep_alive_client.clone()
        };

        let r = client.get(&url_builder(&step));
        let data = download_data(r, debounce);
        let result = processor(data, &index, &step, &(sequence.len() -1));
        post_processor(result);
        pb.inc(step.1 as u64);
    }

    pb.finish_with_message("Done");
}

fn download_data(client: reqwest::RequestBuilder, debounce: Option<u64>) -> String {
    thread::spawn(move || {
        let result = client.send().unwrap().text().unwrap();

        if let Some(debounce) = debounce {
            thread::sleep(Duration::from_millis(debounce));
        }

        result
    })
    .join()
    .expect("Faild to start thread")
}

pub fn get_total_reccords() -> String {
    reqwest::get("https://www.theses.fr/?q=*:*&format=json&type=avancee&rows=0").expect("Failed to get theses.fr data")
        .json::<serde_json::value::Value>()
        .expect("Failed to get json data")["response"]["numFound"]
        .to_string()
}
