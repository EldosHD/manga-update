use clap::Parser;
use dirs;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio;

#[derive(Deserialize, Debug)]
struct Manga {
    name: String,
    short_name: String,
    url: String,
    current_chapter: u32,
}

#[derive(Parser, Debug, StructOpt)]
#[clap(name = "manga-update")]
#[clap(author = "EldosHD")]
struct Cli {
    /// The manga to check for new chapters
    /// Checks all manga in the config file if not specified
    #[structopt(short, long, conflicts_with = "check")]
    manga_name: Option<String>,

    /// The config file to use
    /// Defaults to .manga_update.json in your home directory
    #[structopt(short, long)]
    config_file: Option<PathBuf>,

    /// Checks the config file for errors
    #[structopt(long)]
    check: bool,

    /// add a new manga to the config file
    #[structopt(short, long, requires = "manga_name")]
    add_manga: bool,

    /// The url to check for new chapters
    /// Must contain [CHAPTER] which will be replaced with the next chapter number
    #[structopt(short, long, requires = "add_manga")]
    url: Option<String>,

    /// The short name to use for the manga
    /// Must be unique
    #[structopt(short, long, requires = "add_manga")]
    short_name: Option<String>,

    /// The current chapter number
    /// Must be a number
    /// Defaults to 0
    /// Must be specified if adding a new manga
    #[arg(short = 'C', long, requires = "add_manga")]
    current_chapter: Option<u32>,
}

async fn check_url(url: &str) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.head(url)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36")
    .send();
    let status = response.await.unwrap().status();
    Ok(status.is_success())
}

fn url_exists(url: &str) -> bool {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(check_url(url))
        .unwrap()
}

fn main() {
    let home_dir = dirs::home_dir().unwrap();
    let default_config_file = home_dir.join(".manga_update.json");
    let mut config_file: PathBuf = PathBuf::new();
    let mut manga_name: String = String::new();

    // parse args
    let args = Cli::parse();
    // set config file to default if not specified
    match args.config_file {
        Some(path) => config_file = path,
        None => config_file = default_config_file,
    }

    match args.manga_name {
        Some(mn) => manga_name = mn,
        None => manga_name = "".to_owned(),
    }

    match args.check {
        true => {
            todo!("Check config file for errors");
        }
        false => {}
    }

    let add_manga = false;
    let mut new_manga = Manga {
        name: "".to_owned(),
        short_name: "".to_owned(),
        url: "".to_owned(),
        current_chapter: 0,
    };
    match args.add_manga {
        true => match args.url {
            Some(url) => match args.short_name {
                Some(short_name) => match args.current_chapter {
                    Some(current_chapter) => {
                        new_manga.name = manga_name.clone();
                        new_manga.short_name = short_name.clone();
                        new_manga.url = url.clone();
                        new_manga.current_chapter = current_chapter.clone();
                        println!("Adding new manga:");
                        println!("Name: {}", new_manga.name);
                        println!("Short name: {}", new_manga.short_name);
                        println!("Url: {}", new_manga.url);
                        println!("Current chapter: {}", new_manga.current_chapter);
                        todo!("Add new manga to config file");
                    }
                    None => {
                        println!("Please specify a current chapter");
                        std::process::exit(1);
                    }
                },
                None => {
                    println!("Please specify a short name");
                    std::process::exit(1);
                }
            },
            None => {
                println!("Please specify a url");
                std::process::exit(1);
            }
        },
        false => {}
    }

    // read config file
    let error_message =
        String::from("Unable to read config file ").to_owned() + &config_file.to_str().unwrap();
    let json_file = fs::read_to_string(&config_file).expect(&error_message);

    let error_message =
        String::from("Unable to parse config file ").to_owned() + &config_file.to_str().unwrap();
    let manga_list: Vec<Manga> = serde_json::from_str(&json_file).expect(&error_message);

    let mut new_chapters: Vec<Manga> = Vec::new();
    for mut manga in manga_list {
        if manga.short_name != manga_name && manga_name != "" {
            continue;
        }
        let chapter = manga.current_chapter + 1;
        let url = str::replace(&manga.url, "[CHAPTER]", &chapter.to_string());
        // println!("Checking {} for new chapter at url {}", manga.name, url);
        if url_exists(&url) {
            manga.url = url;
            new_chapters.push(manga);
        }
    }

    if new_chapters.len() > 0 {
        println!("New chapters found!");
        for manga in new_chapters {
            println!("{}: {}", manga.name, manga.url);
        }
    } else {
        println!("No new chapters found");
    }
}
