use clap::{Arg, Command};
use colored::*;
use reqwest;
use std::error::Error;
use tokio;
use tokio::io::AsyncWriteExt;
use std::env;

const VALID_DEPENDENCIES: &[&str] = &[
    "web", "data-jpa", "security", "thymeleaf", "actuator", "batch", "kafka", "devtools", "mail", "flyway", "liquibase", "redis", "amqp", "validation", "h2", "web-services", "graphql", "cache", "mysql", "postgresql"
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Spring Initialzr CLI")
        .version("1.0")
        .about("Generates a Spring project with specified dependencies \nAuthor: Gabriel Ferreira Alves <gabrielf.04.2002@gmail.com>")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("PROJECT_NAME")
                .help("Set the name of the project")
                .required(true),
        )
        .arg(
            Arg::new("dependencies")
                .short('d')
                .long("dependencies")
                .value_name("PROJECT_DEPENDENCIES")
                .help("Set the dependencies of the project")
                .required(false),
        )
        .arg(
            Arg::new("package")
                .short('p')
                .long("package")
                .value_name("PROJECT_PACKAGE_NAME")
                .help("Set the name of the package")
                .required(true),
        )
        .get_matches();

    let project_name = matches.get_one::<String>("name").unwrap();
    let project_package_name = matches.get_one::<String>("package").unwrap();
    let project_dependencies = matches.get_one::<String>("dependencies").map(|deps| deps.as_str()).unwrap_or("");
    let dependencies = validate_dependencies(project_dependencies);

    if let Err(e) = generate_project(dependencies, project_name, project_package_name).await {
        eprintln!("{}: {}", "Error generating project".red(), e);
    }
    
    Ok(())
}

fn validate_dependencies(dependencies: &str) -> Vec<String> {
    let mut valid_dependencies: Vec<String> = Vec::new();
    for d in dependencies.split(',') {
        let trimmed = d.trim().to_string();
        if VALID_DEPENDENCIES.contains(&trimmed.as_str()) {
            valid_dependencies.push(trimmed);
        }
    }

    valid_dependencies
}

async fn generate_project(dependencies: Vec<String>, name: &str, package_name: &str) -> Result<(), Box<dyn Error>> {
    let deps = dependencies.join(",");
    let url = format!("https://start.spring.io/starter.zip?type=maven-project&language=java&bootVersion=3.2.0&baseDir={}&artifactId={}&name={}&packageName={}&dependencies={}", name, name, name, package_name, deps);

    match reqwest::get(url).await {
        Ok(response) => {
            let content = response.bytes().await?;

            let mut downloads_dir = dirs::download_dir().unwrap_or_else(|| env::current_dir().unwrap());
            downloads_dir.push(format!("{}.zip", name));

            match tokio::fs::File::create(&downloads_dir).await {
                Ok(mut file) => {
                    file.write_all(&content).await?;
                    println!("\n{}\n{} \n", "Project generated successfully!".bold().green(), "Saved on your folder Downloads".bold().green());

                    println!("{} \n  {}", 
                        "Project name".bold().green(), 
                        name.bold()
                    );

                    println!("{} \n  {}", 
                        "Package name".bold().green(), 
                        package_name.bold()
                    );
            
                    println!("{}", "Dependencies".bold().green());
                    if dependencies.is_empty() {
                        println!("  {}", "No dependencies added".bold());
                    } else {
                        for d in dependencies {
                            println!("  {}", d.bold());
                        }
                    }

                    println!("");
                }
                Err(e) => {
                    eprintln!("{}: {}", "Error creating file".red(), e);
                    return Err(Box::new(e));
                } 
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error fetching project".red(), e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}