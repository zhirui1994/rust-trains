use clap::{Parser, Subcommand, Args};

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
  #[clap(value_parser)]
  url: String,

  #[clap(value_parser)]
  body: Vec<String>,
}

#[derive(Args, Debug)]
struct Get {
  #[clap(value_parser)]
  url: String,
}

fn main() {
  println!("{:?}", Cli::parse());
}
