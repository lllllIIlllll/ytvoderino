use std::env::var;
use std::process::Command;
use clap::Parser;
use sysinfo::{ProcessExt, System, SystemExt};
use reqwest;
use serde::Deserialize;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    channel_id: String,
}

#[derive(Deserialize, Debug)]
struct SearchResponse {
    kind: String,
    items: Vec<Items>,
}

#[derive(Deserialize, Debug)]
struct Items {
    id: ID,
}

#[derive(Deserialize, Debug)]
struct ID {
    kind: String,
    videoId: String,
}

#[tokio::main]
async fn query_channel(channel_id: String) -> Result<SearchResponse, reqwest::Error> {
    let apikey = var("YT_APIKEY").unwrap();
    let res: SearchResponse = reqwest::get(format!("https://www.googleapis.com/youtube/v3/search?part=snippet&channelId={channel_id}&type=video&eventType=live&key={apikey}"))
        .await?
        .json()
        .await?;
    println!("{:?}", res);
    
    Ok(res)
}

fn main() {
    let args = Args::parse();
    let response = query_channel(args.channel_id);
    match response {
        Ok(response) => {
            if response.items.len() > 0 {
                let procs = System::new_all();
                let url: String = format!("https://www.youtube.com/watch?v={}", response.items[0].id.videoId).to_string();

                println!("Running ytarchive:");
                println!("{url}");
                
                Command::new("target/debug/ytarchive")
                    .arg(url)
                    .arg("best")
                    .spawn()
                    .expect("Failed to run ytarchive");
            } else {
                println!("Not live");
            };
        }
        Err(e) => {
            panic!("Missing response. Check API Key")
        }
    }
}