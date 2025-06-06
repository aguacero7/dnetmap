use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct IPAMConfig {
    Subnet: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IPAM {
    Config: Option<Vec<IPAMConfig>>,
}

#[derive(Debug, Deserialize)]
struct Network {
    Name: String,
    IPAM: Option<IPAM>,
    Id: String,
    Labels: Option<HashMap<String, String>>,
    Options: Option<HashMap<String, String>>,
}

/// Try to find the Linux interface name associated with a Docker network ID
fn find_linux_interface(docker_net_id: &str) -> Option<String> {
    let prefix = &docker_net_id[..12];
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.starts_with("br-") && name.contains(&prefix[..7]) {
                return Some(name);
            }
        }
    }
    None
}

pub fn list_all_networks() {
    let output = Command::new("docker")
        .args(["network", "ls", "--format", "{{.Name}}"])
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => {
            eprintln!(
                "{} {}",
                "ERROR:".red().bold(),
                "Docker daemon does not seem to be running or accessible.".yellow()
            );
            std::process::exit(1);
        }
    };

    let names = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    for name in names {
        let inspect_output = Command::new("docker")
            .args(["network", "inspect", &name])
            .output()
            .expect("Error: failed to inspect Docker network");

        let json = String::from_utf8_lossy(&inspect_output.stdout);
        let parsed: Vec<Network> = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(_) => {
                eprintln!(
                    "{} Could not parse network {}",
                    "Error:".red(),
                    name.yellow()
                );
                continue;
            }
        };

        if let Some(net) = parsed.first() {
            println!("{} {}", "Network name     :".bold(), net.Name.green().bold());

            if let Some(ipam) = &net.IPAM {
                if let Some(configs) = &ipam.Config {
                    for cfg in configs {
                        if let Some(subnet) = &cfg.Subnet {
                            println!("{} {}", "IP range         :".bold(), subnet.cyan());
                        }
                    }
                }
            }

            if let Some(interface) = find_linux_interface(&net.Id) {
                println!("{} {}", "Linux interface  :".bold(), interface.magenta());
            }

            if let Some(labels) = &net.Labels {
                if let Some(project) = labels.get("com.docker.compose.project") {
                    println!(
                        "{} {} {}",
                        "Linked to Compose:".bold(),
                        "yes,".green(),
                        format!("project `{}`", project).green()
                    );
                } else {
                    println!("{} {}", "Linked to Compose:".bold(), "no".dimmed());
                }
            } else {
                println!("{} {}", "Linked to Compose:".bold(), "no".dimmed());
            }
            println!("{} {}", "ID               :".bold(), net.Id[..12].to_string().blue());
            println!("{}", "-".repeat(60).dimmed());
        }
    }
}
