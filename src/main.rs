use reqwest::Client;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("🦀 cRustyClaw - AI Coding Assistant");
        println!("");
        println!("Usage:");
        println!("  crustyclaw ask <question>  - Ask AI a question");
        println!("  crustyclaw status          - Show system status");
        println!("  crustyclaw prices          - Show metal prices");
        println!("  crustyclaw help            - Show this help");
        return Ok(());
    }
    
    match args[1].as_str() {
        "ask" if args.len() > 2 => {
            let question = &args[2..].join(" ");
            ask_groq(question).await?;
        }
        "status" => {
            println!("🏛️ PHOENIXPME STATUS");
            println!("  Fee: 1.1% | Collateral: 10%");
            println!("  Inspection: 48h | PHNX: 1 per $1");
        }
        "prices" => {
            println!("🏛️ METAL PRICES");
            println!("  Gold: $4,676 | Silver: $72.90");
            println!("  Platinum: $1,980 | Palladium: $1,490");
        }
        _ => {
            println!("Commands: ask | status | prices | help");
        }
    }
    
    Ok(())
}

async fn ask_groq(question: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");
    let client = Client::new();
    
    println!("🤔 Thinking...");
    
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "llama-3.3-70b-versatile",
            "messages": [{"role": "user", "content": question}],
            "max_tokens": 2000
        }))
        .send()
        .await?;
    
    let result: serde_json::Value = response.json().await?;
    if let Some(answer) = result["choices"][0]["message"]["content"].as_str() {
        println!("\n{}", answer);
    }
    
    Ok(())
}
