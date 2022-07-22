use clap::{Parser, Subcommand, Args};
use reqwest::Url;
use anyhow::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
  Post(Post),
  Get(Get),
}

#[derive(Args, Debug)]
struct Post {
  #[clap(parse(try_from_str = parse_url))]
  url: String,

  #[clap(value_parser)]
  body: Vec<String>,
}

#[derive(Args, Debug)]
struct Get {
  #[clap(parse(try_from_str = parse_url))]
  url: String,
}

fn parse_url(url: &str) -> Result<String> {
  let _url: Url = url.parse()?;

  Ok(url.into())
}

fn main() {
  println!("{:?}", Cli::parse());
}
