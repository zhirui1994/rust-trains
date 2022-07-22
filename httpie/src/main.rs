use std::str::FromStr;

use clap::{Parser, Subcommand, Args};
use reqwest::Url;
use anyhow::{anyhow, Result};

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

  #[clap(parse(try_from_str = parse_kv_pair))]
  body: Vec<KvPair>,
}

#[derive(Args, Debug)]
struct Get {
  #[clap(parse(try_from_str = parse_url))]
  url: String,
}

#[derive(Debug)]
struct KvPair {
  k: String,
  v: String,
}

impl FromStr for KvPair {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<KvPair, Self::Err> {
    let mut split = s.split("=");
    let err = || anyhow!(format!("Faild to parse {}", s));
    Ok(Self {
      k: (split.next().ok_or_else(err)?).to_string(),
      v: (split.next().ok_or_else(err)?).to_string(),
    })
  }
}

fn parse_url(s: &str) -> Result<String> {
  let _url: Url = s.parse()?;

  Ok(s.into())
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
  Ok(s.parse()?)
}

fn main() {
  println!("{:?}", Cli::parse());
}
