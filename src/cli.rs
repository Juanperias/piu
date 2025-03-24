use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub config: String,
}
