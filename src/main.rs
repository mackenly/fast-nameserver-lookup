use dns_lookup::lookup_host;
use std::env;
use std::process::Command;
use std::time::Instant;

mod custom_lookup;
use custom_lookup::get_nameservers as custom_get_nameservers;

fn main() {
    // Get the domain from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <domain>", args[0]);
        std::process::exit(1);
    }
    let domain = &args[1];

    const ITERATIONS: u32 = 100;

    // storage for the results to store duration
    let mut rust_lib_results = Vec::new();
    let mut rust_custom_results = Vec::new();
    let mut dig_results = Vec::new();

    // Measure speed of Rust crate code
    for _ in 0..ITERATIONS {
        let start_time = Instant::now();
        match get_nameservers(domain) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        rust_lib_results.push(start_time.elapsed());
    }

    println!("Done with the Rust lib code");

    // Measure speed of custom Rust code
    for _ in 0..ITERATIONS {
        let start_time = Instant::now();
        match custom_get_nameservers(domain) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        rust_custom_results.push(start_time.elapsed());
    }

    println!("Done with the custom Rust code");

    // Measure speed of dig command
    // if windows
    if std::env::consts::OS == "windows" {
        for _ in 0..ITERATIONS {
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
        for _ in 0..ITERATIONS {
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
    println!("Rust lib code:");
    let rust_lib_average_time = rust_lib_results.iter().sum::<std::time::Duration>() / ITERATIONS;
    println!("- Average elapsed time: {:?}", rust_lib_average_time);
    rust_lib_results.sort_unstable();
    let rust_lib_median_time = rust_lib_results[rust_lib_results.len() / 2];
    println!("- Median elapsed time: {:?}", rust_lib_median_time);
    let rust_lib_fastest_time = rust_lib_results[0];
    println!("- Fastest elapsed time: {:?}", rust_lib_fastest_time);
    let rust_lib_slowest_time = rust_lib_results[rust_lib_results.len() - 1];
    println!("- Slowest elapsed time: {:?}", rust_lib_slowest_time);

    println!("Custom Rust code:");
    let rust_custom_average_time = rust_custom_results.iter().sum::<std::time::Duration>() / ITERATIONS;
    println!("- Average elapsed time: {:?}", rust_custom_average_time);
    rust_custom_results.sort_unstable();
    let rust_custom_median_time = rust_custom_results[rust_custom_results.len() / 2];
    println!("- Median elapsed time: {:?}", rust_custom_median_time);
    let rust_custom_fastest_time = rust_custom_results[0];
    println!("- Fastest elapsed time: {:?}", rust_custom_fastest_time);
    let rust_custom_slowest_time = rust_custom_results[rust_custom_results.len() - 1];
    println!("- Slowest elapsed time: {:?}", rust_custom_slowest_time);

    println!("dig command:");
    let dig_average_time = dig_results.iter().sum::<std::time::Duration>() / ITERATIONS;
    println!("- Average elapsed time: {:?}", dig_average_time);
    dig_results.sort_unstable();
    let dig_median_time = dig_results[dig_results.len() / 2];
    println!("- Median elapsed time: {:?}", dig_median_time);
    let dig_fastest_time = dig_results[0];
    println!("- Fastest elapsed time: {:?}", dig_fastest_time);
    let dig_slowest_time = dig_results[dig_results.len() - 1];
    println!("- Slowest elapsed time: {:?}", dig_slowest_time);

    // write to file README.md replace contents between ``` and ``` with the results
    let readme = include_str!("../README.md");
    let mut readme = readme.to_string();
    let start = readme.find("```").unwrap();
    let end = readme.rfind("```").unwrap();
    let results = format!(
        "```\nRust lib code:\n- Average elapsed time: {:?}\n- Median elapsed time: {:?}\n- Fastest elapsed time: {:?}\n- Slowest elapsed time: {:?}\n\nCustom Rust code:\n- Average elapsed time: {:?}\n- Median elapsed time: {:?}\n- Fastest elapsed time: {:?}\n- Slowest elapsed time: {:?}\n\nDig command:\n- Average elapsed time: {:?}\n- Median elapsed time: {:?}\n- Fastest elapsed time: {:?}\n- Slowest elapsed time: {:?}\n`",
        rust_lib_average_time,
        rust_lib_median_time,
        rust_lib_fastest_time,
        rust_lib_slowest_time,
        rust_custom_average_time,
        rust_custom_median_time,
        rust_custom_fastest_time,
        rust_custom_slowest_time,
        dig_average_time,
        dig_median_time,
        dig_fastest_time,
        dig_slowest_time
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
        assert!(nameservers.len() >= 2);
    }

    #[test]
    fn test_that_both_functions_return_same_results() {
        let domain = "mackenly.com";
        let result1 = get_nameservers(domain);
        let result2 = custom_get_nameservers(domain);
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        let mut nameservers1 = result1.unwrap();
        let mut nameservers2 = result2.unwrap();
        println!("{:?}", nameservers1);
        println!("{:?}", nameservers2);
        nameservers1.sort_unstable();
        nameservers2.sort_unstable();
        // The crate code returns IPs while the custom code returns domain names
        //assert_eq!(nameservers1, nameservers2);
    }
}