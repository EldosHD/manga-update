use clap::Parser;

/// A simple CLI application to check for new manga chapters
#[derive(Parser, Debug)]
#[command(author, version, about)] // TODO: author doesnt work
struct Args {
    /// The manga to check for new chapters
    /// Checks all manga in the config file if not specified
    #[arg(short, long, default_value = None)]
    manga: String,

    /// The config file to use
    /// Defaults to manga_update.toml in your home directory
    #[arg(short, long, default_value = "manga_update.toml")]
    config_file: String,

    /// The url to the manga
    /// Defaults to the url in the config file
    #[arg(short, long, default_value = None)]
    url: String,

    /// Generate a default config file
    /// The config file will be generated in your home directory and named manga_update.toml
    /// If a config file already exists, it will be not overwritten unless the --force flag is used
    #[arg(short, long, default_value = false)]
    generate_config: bool,
}


fn main() {
    let args = Args::parse();
}
