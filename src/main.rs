use dns_lookup::lookup_host;
use std::env;
use std::process::Command;
use std::time::Instant;

fn main() {
    // Get the domain from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        std::process::exit(1);
    }
    let domain = &args[1];

    let iterations = 100;

    // storage for the results to store duration
    let mut rust_results = Vec::new();
    let mut dig_results = Vec::new();

    // Measure speed of your Rust code
    for _ in 0..iterations {
        let start_time = Instant::now();
        match get_nameservers(domain) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        rust_results.push(start_time.elapsed());
    }

    println!("Done with the Rust code");

    // Measure speed of dig command
    // if windows
    if std::env::consts::OS == "windows" {
        for _ in 0..iterations {
            let start_time = Instant::now();
            let output = Command::new("nslookup")
                .arg("-type=NS")
                .arg(domain)
                .output()
                .expect("Failed to execute nslookup command");
            let _ = String::from_utf8_lossy(&output.stdout);
            dig_results.push(start_time.elapsed());
        }
    } else {
        for _ in 0..iterations {
            let start_time = Instant::now();
            let output = Command::new("dig")
                .arg("+short")
                .arg("NS")
                .arg(domain)
                .output()
                .expect("Failed to execute dig command");
            let _ = String::from_utf8_lossy(&output.stdout);
            dig_results.push(start_time.elapsed());
        }
    }

    // Print the results
    println!("Rust code:");
    println!(
        "- Average elapsed time: {:?}",
        rust_results.iter().sum::<std::time::Duration>() / iterations as u32
    );

    println!("dig command:");
    println!(
        "- Average elapsed time: {:?}",
        dig_results.iter().sum::<std::time::Duration>() / iterations as u32
    );

    // write to file README.md replace contents between ``` and ``` with the results
    let readme = include_str!("../README.md");
    let mut readme = readme.to_string();
    let start = readme.find("```").unwrap();
    let end = readme.rfind("```").unwrap();
    let results = format!(
        "```\nRust code:\n- Average elapsed time: {:?}\n\ndig command:\n- Average elapsed time: {:?}\n",
        rust_results.iter().sum::<std::time::Duration>() / iterations as u32,
        dig_results.iter().sum::<std::time::Duration>() / iterations as u32
    );
    readme.replace_range(start..=end, &results);
    std::fs::write("README.md", readme).expect("Failed to write to README.md");

}

fn get_nameservers(domain: &str) -> Result<Vec<String>, String> {
    // Construct the DNS query string
    let query = format!("{}.", domain);

    // Perform DNS resolution
    match lookup_host(&query) {
        Ok(iter) => {
            let mut nameservers = Vec::new();
            for socket_addr in iter {
                nameservers.push(socket_addr.to_string());
            }
            Ok(nameservers)
        }
        Err(e) => Err(format!("DNS resolution failed: {}", e)),
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nameservers() {
        let domain = "mackenly.com";
        let result = get_nameservers(domain);
        assert!(result.is_ok());
        let nameservers = result.unwrap();
        assert!(!nameservers.is_empty());
        assert!(nameservers.len() >= 4);
    }
}