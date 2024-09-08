use clap::{Parser, Subcommand};
use my_llm_client::{llm_client::MyLLMClient, LLMProvider};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    provider: LLMProvider,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Chat,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let llm_client = MyLLMClient::new(cli.provider);
    let message = llm_client.chat("こんにちは").await;
    println!("{}", message);
}
