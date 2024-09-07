use my_llm_client::llm_client::MyLLMClient;

fn main() {
    let llm_client = MyLLMClient::new(true);
    let message = llm_client.chat("message");
    println!("{}", message);
}
