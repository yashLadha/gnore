mod prompt;

use serde::Deserialize;
use std::error::Error;
use std::{fs, process};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "ignore", about = "Add gitignore to your project in easy way")]
struct Opt {
    #[structopt(short, long = "ignore-list")]
    ignore: bool,
}

#[derive(Deserialize)]
struct IgnoreTemplate<'a> {
    id: &'a str,
    text: &'a str,
}

async fn find_templates() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get("https://www.toptal.com/developers/gitignore/dropdown/templates.json")
        .await?
        .text()
        .await?;
    Ok(resp)
}

fn parse_templates(temp_str: &str) -> Vec<IgnoreTemplate<'_>> {
    let template_vec: Vec<IgnoreTemplate> = serde_json::from_str(temp_str).unwrap();
    template_vec
}

async fn find_git_directory() -> Result<String, Box<dyn Error>> {
    let git_dir = process::Command::new("git")
        .args(&["rev-parse", "--show-toplevel"])
        .output()?;
    let mut git_dir_str = String::from_utf8_lossy(&git_dir.stdout).to_string();
    git_dir_str.pop();
    Ok(git_dir_str)
}

fn create_gitignore(directory: String, content: String) {
    let ignore_file = format!("{}/.gitignore", directory);
    match fs::write(ignore_file, content) {
        Ok(_) => println!("Added gitignore to the directory"),
        Err(x) => println!("Something unexpected happened {}", x),
    }
}

async fn fetch_gitignore_request(ignore_list: String) -> Result<(), reqwest::Error> {
    let request_url = format!(
        "https://www.toptal.com/developers/gitignore/api/{}",
        ignore_list
    );
    let resp = reqwest::get(&request_url).await?;
    let ignore_text = resp.text().await?;

    match find_git_directory().await {
        Ok(directory) => {
            println!("Git directory founded {}", directory);
            create_gitignore(directory, ignore_text);
        }

        Err(_) => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let interactive_list = opt.ignore;

    let template_string = find_templates().await?;
    let template_vec = parse_templates(&template_string);

    if interactive_list {
        prompt::prompt_begin();
        prompt::render_interactive_selector();
    }

    // fetch_gitignore_request(ignore_list).await?;

    Ok(())
}
