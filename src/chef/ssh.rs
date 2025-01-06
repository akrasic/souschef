use crate::chef::search::search_nodes;
use crate::config::KnifeConfig;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn ssh_nodes(
    config: &KnifeConfig,
    query: &str,
    command: &str,
    user: Option<String>,
) -> Result<(), Box<dyn Error>> {
    match search_nodes(config, query).await {
        Ok(nodes) => {
            let nodes: Vec<String> = nodes
                .rows
                .iter()
                .map(|n| n.automatic.ipaddress.clone())
                .collect();

            let k = call_ssh(nodes, command, user).await.unwrap();
            Ok(k)
        }
        Err(e) => Err(format!("Problems during search. {}", e).into()),
    }
}

/// call_ssh - Creation of parallel tasks to be executed for SSH connections
pub async fn call_ssh(
    nodes: Vec<String>,
    command: &str,
    user: Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();

    for node in nodes {
        let results = Arc::clone(&results);
        let node = node.clone();
        let command = command.to_string();
        let user = user.clone();

        // Spawn a new task for each of the nodes
        let handle = tokio::task::spawn(async move {
            let result = execute_ssh_command(node.clone(), command, user.clone()).await;
            results.lock().await.push((node.clone(), result));
        });

        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }

    Ok(())
}

/// execute_ssh_command - create a SSH session using `openssh` crate and execute the command.
pub async fn execute_ssh_command(
    host: String,
    command: String,
    user: Option<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let connection_string = match user {
        Some(u) => format!("{}@{}", u, host),
        None => format!("{}", host),
    };

    match openssh::Session::connect(connection_string, openssh::KnownHosts::Accept).await {
        Ok(session) => match session.raw_command(command).output().await {
            Ok(output) => {
                if output.status.success() {
                    if let Ok(stdout) = String::from_utf8(output.stdout) {
                        println!("{}: {}", host, stdout);
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("{}:Failure {}", host, stderr);
                }
            }
            Err(e) => eprintln!("{}: Command execution error: {}", host, e),
        },
        Err(e) => eprintln!("Failed to connect to {}: {}", host, e),
    }
    Ok(())
}
