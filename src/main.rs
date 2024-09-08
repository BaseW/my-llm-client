use clap::{Parser, Subcommand};
use my_llm_client::llm_client::MyLLMClient;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    provider: Provider,
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Chat,
}

#[derive(Clone)]
enum Provider {
    Mock,
    ChatGPT,
}

impl From<&str> for Provider {
    fn from(s: &str) -> Self {
        match s {
            "mock" => Provider::Mock,
            "chatgpt" => Provider::ChatGPT,
            _ => panic!("Invalid provider"),
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.provider {
        Provider::Mock => {
            let llm_client = MyLLMClient::new(true);
            let message = llm_client.chat("こんにちは").await;
            println!("{}", message);
        }
        Provider::ChatGPT => {
            let llm_client = MyLLMClient::new(false);
            let message = llm_client.chat("こんにちは").await;
            println!("{}", message);
        }
    }
}
