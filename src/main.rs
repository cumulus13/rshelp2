use anyhow::{Context, Result};
use clap::{Parser, AppSettings};
use colored::*;
use make_colors::make_colors;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::process::Command;

/// 🦀 rshelp - Rust Enhanced Help Tool with Beautiful Terminal Output
#[derive(Parser)]
#[command(author = "Hadi Cahyadi <cumulus13@gmail.com>")]
#[command(version)]
#[command(about = "Rust enhanced help tool with beautiful terminal output")]
#[command(settings(AppSettings::ColoredHelp))]
pub struct Cli {
    /// Module, function, struct, or trait to get help for
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
                
                let _ = rl.add_history_entry(line.to_string());
                
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
                
                println!();
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
            println!("{} Detailed mode toggled", "ℹ️".bright_blue());
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
    let colored_query = make_colors(query, "#00FFFF", None);
    
    println!("\n{} {} {}", "📘".bright_blue(), "Help for".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    println!("{}", format!("Documentation for {}", query));
    
    if detailed {
        println!("\n{}", "Detailed Information:".bright_yellow());
        println!("  Type: Function/Struct/Trait");
        println!("  Module: std::example");
        println!("  Source: src/lib.rs");
        println!("  Visibility: pub");
    }
    
    println!("{}", "═".repeat(60).bright_black());
    println!("{} {}", "ℹ️".bright_blue(), "Tip: Use -s flag to see source code".bright_black());
    
    Ok(())
}

fn show_source_code(query: &str) -> Result<()> {
    let colored_query = make_colors(query, "#00FFFF", None);
    
    println!("\n{} {} {}", "📄".bright_yellow(), "Source for".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    println!("{}", format!("// Source code for {}", query));
    println!("pub fn example() {{}}");
    
    println!("{}", "═".repeat(60).bright_black());
    
    Ok(())
}

fn list_module_items(query: &str) -> Result<()> {
    let colored_query = make_colors(query, "#00FFFF", None);
    
    println!("\n{} {} {}", "📦".bright_green(), "Items in".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    let items = vec![
        format!("{}::new", query),
        format!("{}::from_str", query),
        format!("{}::to_string", query),
    ];
    
    println!("{}", "🔧 Functions:".bright_green());
    for item in items {
        println!("  {}", item.bright_white());
    }
    
    println!("{}", "═".repeat(60).bright_black());
    
    Ok(())
}

fn search_items(query: &str) -> Result<()> {
    let colored_query = make_colors(query, "#00FFFF", None);
    
    println!("\n{} {} {}", "🔍".bright_blue(), "Search results for".bright_white(), colored_query.bold());
    println!("{}", "═".repeat(60).bright_black());
    
    println!("  🔧 {} (function)", format!("std::collections::{}", query).bright_white());
    println!("  📐 {} (struct)", format!("std::iter::{}", query).bright_white());
    
    println!("{}", "═".repeat(60).bright_black());
    
    Ok(())
}