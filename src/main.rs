use reqwest::Client;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return Ok(());
    }
    
    match args[1].as_str() {
        "ask" if args.len() > 2 => {
            let question = &args[2..].join(" ");
            ask_groq(question).await?;
        }
        "analyze" if args.len() > 2 => {
            analyze_file(&args[2]).await?;
        }
        "audit" if args.len() > 2 => {
            audit_project(&args[2]).await?;
        }
        "status" => system_status(),
        "optimize" => system_optimize(),
        "clean" => system_clean(),
        "prices" => show_prices(),
        "claw" => show_lobster(),
        "help" => print_help(),
        _ => println!("🦞 Unknown command. Try: help, ask, audit, analyze, status, claw"),
    }
    
    Ok(())
}

async fn ask_groq(question: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GROQ_API_KEY").expect("🦞 GROQ_API_KEY not set");
    let client = Client::new();
    
    println!("🦞 Thinking... 🧠");
    
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
        println!("\n🦞 {}\n", answer);
    }
    Ok(())
}

async fn analyze_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        println!("🦞 File not found: {}", path);
        return Ok(());
    }
    
    let content = fs::read_to_string(path)?;
    let preview = if content.len() > 1500 {
        format!("{}...\n\n[File truncated: {} total chars]", &content[..1500], content.len())
    } else {
        content
    };
    
    let question = format!("Analyze this file and provide insights:\n\n{}", preview);
    println!("🦞 Claws deep into the code... 🔍");
    ask_groq(&question).await?;
    Ok(())
}

async fn audit_project(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🦞 CRUSTYCLAW SECURITY AUDIT");
    println!("🦞 ===========================");
    println!("🦞 Target: {}", path);
    println!("");
    
    let mut issues = Vec::new();
    let mut good = Vec::new();
    
    println!("🦞 Scuttling through the codebase... 🦀");
    
    // Check for common Rust issues
    let output = Command::new("grep")
        .args(["-rn", "unwrap()", path, "--include=*.rs"])
        .output()?;
    if !output.stdout.is_empty() {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        issues.push(format!("🦞 Found {} .unwrap() calls - can cause panics", count));
    }
    
    let output = Command::new("grep")
        .args(["-rn", "unsafe", path, "--include=*.rs"])
        .output()?;
    if !output.stdout.is_empty() {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        issues.push(format!("🦞 Found {} unsafe blocks - review carefully", count));
    }
    
    let output = Command::new("grep")
        .args(["-rn", "expect(", path, "--include=*.rs"])
        .output()?;
    if !output.stdout.is_empty() {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        issues.push(format!("🦞 Found {} .expect() calls - potential panic points", count));
    }
    
    // PhoenixPME specific checks
    let output = Command::new("grep")
        .args(["-rn", "collateral", path, "--include=*.rs"])
        .output()?;
    if output.stdout.is_empty() {
        issues.push("🦞 ⚠️ No collateral mechanism found - Required for PhoenixPME".to_string());
    } else {
        good.push("🦞 ✅ Collateral mechanism present".to_string());
    }
    
    let output = Command::new("grep")
        .args(["-rn", "1.1", path, "--include=*.rs"])
        .output()?;
    if output.stdout.is_empty() {
        issues.push("🦞 ⚠️ No 1.1% fee found - Required for PhoenixPME".to_string());
    } else {
        good.push("🦞 ✅ Fee structure present (1.1%)".to_string());
    }
    
    // Print report
    println!("\n🦞 📊 AUDIT REPORT");
    println!("🦞 ================");
    
    if !good.is_empty() {
        println!("\n🦞 ✅ PASSED CHECKS:");
        for g in &good {
            println!("  {}", g);
        }
    }
    
    if !issues.is_empty() {
        println!("\n🦞 ⚠️ ISSUES FOUND:");
        for issue in &issues {
            println!("  {}", issue);
        }
    }
    
    if issues.is_empty() && !good.is_empty() {
        println!("\n🦞 🎉 EXCELLENT! No critical issues found!");
        println!("🦞 Your code is claw-some! 🦞");
    }
    
    // AI analysis
    println!("\n🦞 🤖 AI ANALYSIS:");
    let ai_prompt = format!("Based on this audit of {} which found these issues: {}\nAnd these passing checks: {}\nProvide specific recommendations for fixing the issues and maintaining best practices.", 
        path, issues.join(", "), good.join(", "));
    ask_groq(&ai_prompt).await?;
    
    Ok(())
}

fn system_status() {
    println!("🦞 SYSTEM STATUS");
    println!("🦞 =============");
    
    if let Ok(output) = Command::new("nproc").output() {
        if let Ok(cpus) = String::from_utf8(output.stdout) {
            println!("🦞 CPU Cores: {}", cpus.trim());
        }
    }
    
    if let Ok(output) = Command::new("free").args(["-h"]).output() {
        if let Ok(mem) = String::from_utf8(output.stdout) {
            for line in mem.lines().skip(1).take(1) {
                println!("🦞 Memory: {}", line);
            }
        }
    }
    
    if let Ok(output) = Command::new("df").args(["-h", "/"]).output() {
        if let Ok(disk) = String::from_utf8(output.stdout) {
            if let Some(line) = disk.lines().nth(1) {
                println!("🦞 Disk: {}", line);
            }
        }
    }
}

fn system_optimize() {
    println!("🦞 🔧 Optimizing system...");
    Command::new("sudo").args(["apt", "autoremove", "-y"]).output().ok();
    Command::new("sudo").args(["apt", "autoclean", "-y"]).output().ok();
    println!("🦞 ✅ System optimized!");
}

fn system_clean() {
    println!("🦞 🧹 Cleaning system...");
    Command::new("sudo").args(["apt", "clean"]).output().ok();
    println!("🦞 ✅ System cleaned!");
}

fn show_prices() {
    println!("🦞 🏛️ METAL PRICES");
    println!("🦞   Gold: $4,676 | Silver: $72.90");
    println!("🦞   Platinum: $1,980 | Palladium: $1,490");
}

fn show_lobster() {
    println!("🦞");
    println!("🦞  .--.");
    println!("🦞 |o_o |");
    println!("🦞 |:_/ |");
    println!("🦞 //   \\ \\");
    println!("🦞(|     |)");
    println!("🦞/'\\_   _/`\\");
    println!("🦞\\___)=(___/");
    println!("🦞");
    println!("🦞 CRUSTYCLAW - The Lobster of Code!");
    println!("🦞 Claw-some AI assistant for developers");
}

fn print_help() {
    println!("🦞 CRUSTYCLAW - The Lobster of Code");
    println!("🦞 ================================");
    println!("");
    println!("🦞 COMMANDS:");
    println!("  ask <question>           - Ask AI anything");
    println!("  analyze <file>           - AI analysis of any file");
    println!("  audit <path>             - Security audit of codebase");
    println!("  status                   - System status");
    println!("  optimize                 - Optimize system");
    println!("  clean                    - Clean system");
    println!("  prices                   - Show metal prices");
    println!("  claw                     - Show the lobster");
    println!("  help                     - Show this help");
    println!("");
    println!("🦞 EXAMPLES:");
    println!("  crustyclaw ask 'What is Rust?'");
    println!("  crustyclaw analyze ~/dev/TX/escrow.rs");
    println!("  crustyclaw audit ~/dev/TX");
    println!("  crustyclaw status");
    println!("  crustyclaw claw");
}
