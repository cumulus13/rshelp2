use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand, ValueEnum};
use clap_color_help::ColorHelp;
use colored::*;
use make_colors::make_color;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// 🦀 rshelp - Rust Enhanced Help Tool with Beautiful Terminal Output
/// 
/// Quickly view documentation (docstrings) for Rust functions, 
/// structs, traits, and modules directly from your terminal.
#[derive(Parser, ColorHelp)]
#[command(author = "Hadi Cahyadi <cumulus13@gmail.com>")]
#[command(version)]
#[command(about = "Rust enhanced help tool with beautiful terminal output", long_about = None)]
#[command(name = "rshelp")]
#[command(help_template = "\
{before-help}{name} {version}
{author}
{about}

{usage-heading} {usage}

{all-args}{after-help}

Examples:
  rshelp std::fs::File                Show help for std::fs::File
  rshelp serde_json::from_str        Show help for serde_json::from_str
  rshelp -s regex::Regex             Show source code for regex::Regex
  rshelp --interactive               Start interactive mode
  rshelp --search HashMap            Search for items containing 'HashMap'
")]
pub struct Cli {
    /// Module, function, struct, or trait to get help for (e.g., std::fs::File, serde_json::from_str)
    pub query: Option<String>,

    /// Show source code instead of help documentation
    #[arg(short = 's', long = "source", conflicts_with = "search")]
    pub source: bool,

    /// Show all public items in module
    #[arg(short = 'a', long = "show-all")]
    pub show_all: bool,

    /// Start interactive mode
    #[arg(short = 'i', long = "interactive")]
    pub interactive: bool,

    /// Search for items containing the query string
    #[arg(short = 'S', long = "search", conflicts_with = "source")]
    pub search: Option<String>,

    /// Show detailed type information
    #[arg(short = 'd', long = "detailed")]
    pub detailed: bool,

    /// Clear screen before showing results
    #[arg(short = 'c', long = "clear")]
    pub clear_screen: bool,

    /// Show version information
    #[arg(short = 'V', long = "version")]
    pub version_flag: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Handle version flag separately due to clap-version-flag integration
    if cli.version_flag {
        println!("rshelp {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if cli.clear_screen {
        clear_screen()?;
    }

    if cli.interactive {
        run_interactive_mode(&cli)?;
    } else if let Some(query) = cli.query {
        if cli.source {
            show_source_code(&query)?;
        } else if cli.show_all {
            list_module_items(&query)?;
        } else {
            show_help(&query, cli.detailed)?;
        }
    } else if let Some(search_query) = cli.search {
        search_items(&search_query)?;
    } else {
        // If no query provided, show interactive mode automatically
        run_interactive_mode(&cli)?;
    }

    Ok(())
}

fn clear_screen() -> Result<()> {
    #[cfg(windows)]
    let _ = Command::new("cmd").arg("/c").arg("cls").status();
    #[cfg(not(windows))]
    let _ = Command::new("clear").status();
    Ok(())
}

fn run_interactive_mode(cli: &Cli) -> Result<()> {
    println!("{}", "🦀 rshelp interactive mode".bright_cyan().bold());
    println!("{}", "Type 'exit' or 'quit' to exit, or 'c query' to clear screen".bright_black());
    println!("{}", "Type 'help' for more commands".bright_black());
    println!();

    let mut rl = DefaultEditor::new().context("Failed to create readline editor")?;
    
    loop {
        let readline = rl.readline("rshelp> ");
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                
                let _ = rl.add_history_entry(line.as_ref());
                
                if line == "exit" || line == "quit" {
                    println!("{}", "👋 Goodbye!".bright_yellow());
                    break;
                } else if line == "help" {
                    print_interactive_help();
                    continue;
                } else if line == "clear" {
                    clear_screen()?;
                    continue;
                }
                
                // Handle "c query" pattern
                let (should_clear, query) = if let Some(stripped) = line.strip_prefix("c ") {
                    (true, stripped.to_string())
                } else if line.ends_with(" c") {
                    (true, line[..line.len()-2].to_string())
                } else {
                    (false, line.to_string())
                };
                
                if should_clear {
                    clear_screen()?;
                }
                
                // Check if query is a command
                if query.starts_with(':') {
                    handle_interactive_command(&query)?;
                    continue;
                }
                
                if cli.source {
                    if let Err(e) = show_source_code(&query) {
                        eprintln!("{} Error: {}", "❌".red(), e);
                    }
                } else {
                    if let Err(e) = show_help(&query, cli.detailed) {
                        eprintln!("{} Error: {}", "❌".red(), e);
                    }
                }
                
                println!(); // Add spacing between results
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "🔴 Interrupted".bright_red());
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "👋 Goodbye!".bright_yellow());
                break;
            }
            Err(err) => {
                eprintln!("{} Readline error: {}", "❌".red(), err);
                break;
            }
        }
    }
    
    Ok(())
}

fn print_interactive_help() {
    println!("\n{}", "📚 Interactive Commands:".bright_yellow().bold());
    println!("  {}", "help, ?       Show this help message".bright_white());
    println!("  {}", "clear, cls    Clear the screen".bright_white());
    println!("  {}", "exit, quit    Exit interactive mode".bright_white());
    println!("  {}", "c <query>     Clear screen and show help for <query>".bright_white());
    println!("  {}", "<query> c     Same as above, but query first".bright_white());
    println!("  {}", ":source <q>   Show source code for <query>".bright_white());
    println!("  {}", ":list <q>     List items in <query> module".bright_white());
    println!("  {}", ":search <q>   Search for items containing <q>".bright_white());
    println!("  {}", ":detailed     Toggle detailed mode".bright_white());
    println!("  {}", ":version      Show version".bright_white());
    println!();
}

fn handle_interactive_command(query: &str) -> Result<()> {
    let parts: Vec<&str> = query.splitn(2, ' ').collect();
    let cmd = parts[0];
    let arg = parts.get(1).unwrap_or(&"").trim();
    
    match cmd {
        ":source" | ":s" => {
            if arg.is_empty() {
                eprintln!("{} Please specify a query for source", "⚠️".bright_yellow());
                return Ok(());
            }
            show_source_code(arg)?;
        }
        ":list" | ":l" => {
            if arg.is_empty() {
                eprintln!("{} Please specify a module to list", "⚠️".bright_yellow());
                return Ok(());
            }
            list_module_items(arg)?;
        }
        ":search" | ":S" => {
            if arg.is_empty() {
                eprintln!("{} Please specify a search query", "⚠️".bright_yellow());
                return Ok(());
            }
            search_items(arg)?;
        }
        ":detailed" | ":d" => {
            // Toggle detailed mode - this would need to modify global state
            println!("{} Detailed mode toggled (not implemented in this version)", "ℹ️".bright_blue());
        }
        ":version" | ":v" => {
            println!("rshelp {}", env!("CARGO_PKG_VERSION"));
        }
        ":clear" | ":c" => {
            clear_screen()?;
        }
        _ => {
            eprintln!("{} Unknown command: {}", "❌".red(), cmd);
            print_interactive_help();
        }
    }
    
    Ok(())
}

fn show_help(query: &str, detailed: bool) -> Result<()> {
    let colored_query = query.truecolor(0, 255, 255); // #00FFFF
    
    println!("\n{} {} {}", "📘".bright_blue(), "Help for".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    // Simulate rustdoc lookup - in production, would use actual rustdoc
    let item = lookup_item(query)?;
    
    print_item_help(&item, detailed)?;
    
    println!("{}", "═".repeat(60).bright_black());
    println!("{} {}", "ℹ️".bright_blue(), "Tip: Use -s flag to see source code".bright_black());
    
    Ok(())
}

fn show_source_code(query: &str) -> Result<()> {
    let colored_query = query.truecolor(0, 255, 255); // #00FFFF
    
    println!("\n{} {} {}", "📄".bright_yellow(), "Source for".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    // In production, this would fetch actual source code
    let source = fetch_source(query)?;
    
    // Syntax highlighting simulation
    for line in source.lines() {
        if line.starts_with("fn") || line.starts_with("pub fn") {
            println!("{}", line.bright_yellow());
        } else if line.starts_with("struct") || line.starts_with("pub struct") {
            println!("{}", line.bright_magenta());
        } else if line.starts_with("impl") {
            println!("{}", line.bright_cyan());
        } else if line.trim().starts_with("//") {
            println!("{}", line.bright_black());
        } else if line.trim().is_empty() {
            println!();
        } else {
            println!("{}", line);
        }
    }
    
    println!("{}", "═".repeat(60).bright_black());
    
    Ok(())
}

fn list_module_items(query: &str) -> Result<()> {
    let colored_query = query.truecolor(0, 255, 255); // #00FFFF
    
    println!("\n{} {} {}", "📦".bright_green(), "Items in".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    let items = get_module_items(query)?;
    
    // Group items by type
    let mut functions = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut traits = Vec::new();
    let mut other = Vec::new();
    
    for item in items {
        let item_type = guess_item_type(&item);
        match item_type {
            "function" => functions.push(item),
            "struct" => structs.push(item),
            "enum" => enums.push(item),
            "trait" => traits.push(item),
            _ => other.push(item),
        }
    }
    
    if !functions.is_empty() {
        println!("{}", "🔧 Functions:".bright_green());
        for f in functions {
            println!("  {}", f.bright_white());
        }
    }
    
    if !structs.is_empty() {
        println!("{}", "📐 Structs:".bright_magenta());
        for s in structs {
            println!("  {}", s.bright_white());
        }
    }
    
    if !enums.is_empty() {
        println!("{}", "🎨 Enums:".bright_cyan());
        for e in enums {
            println!("  {}", e.bright_white());
        }
    }
    
    if !traits.is_empty() {
        println!("{}", "🧬 Traits:".bright_yellow());
        for t in traits {
            println!("  {}", t.bright_white());
        }
    }
    
    if !other.is_empty() {
        println!("{}", "📌 Other:".bright_black());
        for o in other {
            println!("  {}", o.bright_white());
        }
    }
    
    println!("{}", "═".repeat(60).bright_black());
    
    Ok(())
}

fn search_items(query: &str) -> Result<()> {
    let colored_query = query.truecolor(0, 255, 255); // #00FFFF
    
    println!("\n{} {} {}", "🔍".bright_blue(), "Search results for".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    let results = perform_search(query)?;
    
    if results.is_empty() {
        println!("{} {}", "ℹ️".bright_blue(), "No results found".bright_black());
    } else {
        for (path, item_type) in results {
            let icon = match item_type.as_str() {
                "function" => "🔧",
                "struct" => "📐",
                "enum" => "🎨",
                "trait" => "🧬",
                "module" => "📦",
                _ => "📄",
            };
            println!("  {} {} ({})", icon, path.bright_white(), item_type.bright_black());
        }
    }
    
    println!("{}", "═".repeat(60).bright_black());
    
    Ok(())
}

// Mock functions that would be replaced with actual Rust doc implementations

fn lookup_item(query: &str) -> Result<String> {
    // This would use rustdoc, rustc, or cargo doc to fetch actual documentation
    // For demo purposes, return mock data
    Ok(format!("Documentation for {}", query))
}

fn fetch_source(query: &str) -> Result<String> {
    // Would use rustc source code fetching
    Ok(format!("// Source code for {}\npub fn example() {{}}\nimpl std::fmt::Display for Example {{}}", query))
}

fn get_module_items(query: &str) -> Result<Vec<String>> {
    // Would inspect module items using rustdoc JSON
    Ok(vec![
        format!("{}{}", query, "::new"),
        format!("{}{}", query, "::from_str"),
        format!("{}{}", query, "::to_string"),
        format!("{}{}", query, "::clone"),
    ])
}

fn perform_search(query: &str) -> Result<Vec<(String, String)>> {
    // Would search through installed crates and std
    Ok(vec![
        (format!("std::collections::{}", query), "struct".to_string()),
        (format!("std::iter::{}", query), "function".to_string()),
        (format!("std::string::{}", query), "function".to_string()),
    ])
}

fn print_item_help(item: &str, detailed: bool) -> Result<()> {
    // Mock print function - in production would use proper formatting
    println!("{}", item);
    
    if detailed {
        println!("\n{}", "Detailed Information:".bright_yellow());
        println!("  Type: Function/Struct/Trait");
        println!("  Module: std::example");
        println!("  Source: src/lib.rs");
        println!("  Visibility: pub");
        println!("  Attributes: #[derive(Debug)]");
    }
    
    Ok(())
}

fn guess_item_type(item: &str) -> String {
    if item.contains("::new") || item.contains("::from") {
        "function".to_string()
    } else if item.contains("::to_") || item.contains("::as_") {
        "function".to_string()
    } else if item.contains("Struct") || item.contains("struct") {
        "struct".to_string()
    } else if item.contains("Enum") || item.contains("enum") {
        "enum".to_string()
    } else if item.contains("Trait") || item.contains("trait") {
        "trait".to_string()
    } else if item.contains("std::") || item.contains("core::") {
        "module".to_string()
    } else {
        "other".to_string()
    }
}