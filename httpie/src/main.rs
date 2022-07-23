use std::{str::FromStr, collections::HashMap};

use clap::{Parser, Subcommand, Args};
use mime::Mime;
use reqwest::{Url, Client, Response, header};
use anyhow::{anyhow, Result};
use colored::Colorize;

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

async fn get(client: Client, args: &Get) -> Result<()> {
  let res = client.get(&args.url).send().await?;
  Ok(print_res(res).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
  let mut body = HashMap::new();
  for pair in args.body.iter() {
    body.insert(&pair.k, &pair.v);
  }
  let res = client.post(&args.url).json(&body).send().await?;
 
  Ok(print_res(res).await?)
}

async fn print_res(res: Response) -> Result<()> {
  print_status(&res);
  print_header(&res);

  let mime = get_content_type(&res);
  let body = res.text().await?;
  print_body(mime, &body);
  Ok(())
}

fn print_status(res: &Response) {
  let status = format!("{:?} {}", res.version(), res.status()).blue();
  println!("{}\n", status);
}

fn print_header(res: &Response) {
  for (name, value) in res.headers() {
    println!("{}: {:?}", name.to_string().green(), value);
  }

  println!("\n");
}

fn get_content_type(res: &Response) -> Option<Mime> {
  res.headers()
    .get(header::CONTENT_TYPE)
    .map(|v| v.to_str().unwrap().parse().unwrap())
}

fn print_body(m: Option<Mime>, body: &String) {
  match m {
    Some(v) if v == mime::APPLICATION_JSON => {
      print!("{}", jsonxf::pretty_print(body).unwrap().cyan())
    },
    _ => println!("{}", body),
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  let client = Client::new();
  let result = match cli.action {
    Action::Get(args) => get(client, &args).await?,
    Action::Post(args) => post(client, &args).await?,
  };

  Ok(result)
}
