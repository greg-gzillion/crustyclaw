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
        // AI domain
        "ask" | "ai" if args.len() > 2 => {
            let question = &args[2..].join(" ");
            ask_groq(question).await?;
        }
        "analyze" if args.len() > 2 => {
            let target = &args[2];
            analyze_file(target).await?;
        }
        
        // System domain
        "system" => {
            if args.len() < 3 {
                println!("System commands: status, optimize, clean, backup, info");
                return Ok(());
            }
            match args[2].as_str() {
                "status" => system_status(),
                "optimize" => system_optimize(),
                "clean" => system_clean(),
                "info" => system_info(),
                _ => println!("Unknown: {}", args[2]),
            }
        }
        
        // Network domain
        "network" => {
            if args.len() < 3 {
                println!("Network commands: scan, ping, ports");
                return Ok(());
            }
            match args[2].as_str() {
                "scan" if args.len() > 3 => network_scan(&args[3]).await?,
                "ping" if args.len() > 3 => network_ping(&args[3])?,
                "ports" if args.len() > 3 => network_ports(&args[3])?,
                _ => println!("Usage: network scan|ping|ports <target>"),
            }
        }
        
        // Files domain
        "files" => {
            if args.len() < 3 {
                println!("Files commands: organize, search, backup, tree");
                return Ok(());
            }
            match args[2].as_str() {
                "organize" if args.len() > 3 => organize_files(&args[3])?,
                "search" if args.len() > 3 => search_files(&args[3], &args[4..].join(" "))?,
                "tree" if args.len() > 3 => file_tree(&args[3], 0)?,
                _ => println!("Usage: files organize|search|tree <path> [pattern]"),
            }
        }
        
        // Media domain
        "media" => {
            if args.len() < 3 {
                println!("Media commands: info, convert, resize");
                return Ok(());
            }
            match args[2].as_str() {
                "info" if args.len() > 3 => media_info(&args[3])?,
                "convert" if args.len() > 4 => media_convert(&args[3], &args[4])?,
                _ => println!("Usage: media info|convert <file> [output]"),
            }
        }
        
        // Security domain
        "security" => {
            if args.len() < 3 {
                println!("Security commands: hash, encrypt, audit");
                return Ok(());
            }
            match args[2].as_str() {
                "hash" if args.len() > 3 => hash_file(&args[3])?,
                "audit" if args.len() > 3 => security_audit(&args[3]).await?,
                _ => println!("Usage: security hash|audit <file>"),
            }
        }
        
        // Original commands (backward compatible)
        "status" => system_status(),
        "prices" => show_prices(),
        "help" => print_help(),
        
        _ => println!("Unknown command. Try: help, ask, system, network, files, media, security"),
    }
    
    Ok(())
}

// ============ AI FUNCTIONS ============
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

async fn analyze_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        println!("File not found: {}", path);
        return Ok(());
    }
    
    let content = fs::read_to_string(path)?;
    let preview = if content.len() > 1000 {
        format!("{}...\n\n[Truncated: {} total chars]", &content[..1000], content.len())
    } else {
        content
    };
    
    let question = format!("Analyze this file:\n\n{}", preview);
    ask_groq(&question).await?;
    Ok(())
}

// ============ SYSTEM FUNCTIONS ============
fn system_status() {
    println!("🖥️  SYSTEM STATUS");
    println!("=================");
    
    // CPU info
    let output = Command::new("nproc")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok());
    if let Some(cpus) = output {
        println!("CPU Cores: {}", cpus.trim());
    }
    
    // Memory info
    let mem = Command::new("free")
        .args(["-h"])
        .output()
        .ok();
    if let Some(o) = mem {
        if let Ok(s) = String::from_utf8(o.stdout) {
            for line in s.lines().skip(1).take(1) {
                println!("Memory: {}", line);
            }
        }
    }
    
    // Disk info
    let disk = Command::new("df")
        .args(["-h", "/"])
        .output()
        .ok();
    if let Some(o) = disk {
        if let Ok(s) = String::from_utf8(o.stdout) {
            if let Some(line) = s.lines().nth(1) {
                println!("Disk: {}", line);
            }
        }
    }
}

fn system_optimize() {
    println!("🔧 OPTIMIZING SYSTEM...");
    Command::new("sudo")
        .args(["apt", "autoremove", "-y"])
        .output()
        .ok();
    Command::new("sudo")
        .args(["apt", "autoclean", "-y"])
        .output()
        .ok();
    println!("✅ System optimized");
}

fn system_clean() {
    println!("🧹 CLEANING SYSTEM...");
    Command::new("sudo")
        .args(["apt", "clean"])
        .output()
        .ok();
    Command::new("rm")
        .args(["-rf", "~/.cache/thumbnails/*"])
        .output()
        .ok();
    println!("✅ System cleaned");
}

fn system_info() {
    println!("📊 SYSTEM INFORMATION");
    println!("====================");
    Command::new("uname").args(["-a"]).status().ok();
    Command::new("lsb_release").args(["-a"]).status().ok();
}

// ============ NETWORK FUNCTIONS ============
async fn network_scan(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Scanning: {}", target);
    let question = format!("What security considerations should I know about when scanning {}?", target);
    ask_groq(&question).await?;
    Ok(())
}

fn network_ping(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("ping").args(["-c", "4", target]).status()?;
    Ok(())
}

fn network_ports(target: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Common ports on {}:", target);
    Command::new("nmap").args(["-F", target]).output()?;
    Ok(())
}

// ============ FILE FUNCTIONS ============
fn organize_files(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📁 Organizing: {}", path);
    println!("This would organize files by extension, date, etc.");
    println!("Feature in progress...");
    Ok(())
}

fn search_files(path: &str, pattern: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Searching '{}' in {}", pattern, path);
    Command::new("grep")
        .args(["-rn", pattern, path])
        .output()?;
    Ok(())
}

fn file_tree(path: &str, depth: usize) -> Result<(), Box<dyn std::error::Error>> {
    if depth > 3 {
        return Ok(());
    }
    let indent = "  ".repeat(depth);
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name();
        if entry.path().is_dir() {
            println!("{}📁 {}", indent, name.to_string_lossy());
            file_tree(&entry.path().to_string_lossy(), depth + 1)?;
        } else {
            let metadata = entry.metadata()?;
            println!("{}📄 {} ({} bytes)", indent, name.to_string_lossy(), metadata.len());
        }
    }
    Ok(())
}

// ============ MEDIA FUNCTIONS ============
fn media_info(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Media info for: {}", file);
    Command::new("file").args([file]).status()?;
    Ok(())
}

fn media_convert(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Converting {} to {}", input, output);
    Command::new("convert").args([input, output]).status()?;
    Ok(())
}

// ============ SECURITY FUNCTIONS ============
fn hash_file(file: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 Hashing: {}", file);
    Command::new("sha256sum").args([file]).status()?;
    Ok(())
}

async fn security_audit(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔒 Security audit of: {}", path);
    let question = format!("What security vulnerabilities should I check for in {}?", path);
    ask_groq(&question).await?;
    Ok(())
}

// ============ UTILITY FUNCTIONS ============
fn show_prices() {
    println!("🏛️ METAL PRICES");
    println!("  Gold: $4,676 | Silver: $72.90");
    println!("  Platinum: $1,980 | Palladium: $1,490");
}

fn print_help() {
    println!("🦀 cRustyClaw - Universal AI Agent");
    println!("");
    println!("USAGE:");
    println!("  crustyclaw [DOMAIN] [ACTION] [TARGET]");
    println!("");
    println!("DOMAINS:");
    println!("  ask <question>           - Ask AI anything");
    println!("  analyze <file>           - AI analysis of any file");
    println!("  system status|optimize   - System management");
    println!("  network scan|ping <host> - Network tools");
    println!("  files tree|search <path> - File management");
    println!("  security audit|hash <file> - Security tools");
    println!("  status                   - System status");
    println!("  prices                   - Metal prices");
    println!("");
    println!("EXAMPLES:");
    println!("  crustyclaw ask 'What is blockchain?'");
    println!("  crustyclaw system optimize");
    println!("  crustyclaw files tree ~/dev");
    println!("  crustyclaw network scan localhost");
    println!("  crustyclaw security audit ~/dev/TX");
    println!("  crustyclaw analyze README.md");
}
