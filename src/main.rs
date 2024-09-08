use my_llm_client::llm_client::MyLLMClient;

#[tokio::main]
async fn main() {
    let llm_client = MyLLMClient::new(false);
    let message = llm_client.chat("こんにちは").await;
    println!("{}", message);
}
