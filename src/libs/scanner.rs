use std::net::Ipv4Addr;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::libs::calc_ip::{IpCalculatorError, Subnet};

const MAX_CONCURRENT_PINGS: usize = 50;

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub responsive_hosts: Vec<Ipv4Addr>,
    pub total_scanned: usize,
}

pub fn scan_subnet_with_progress(
    cidr: &str,
    completed: Arc<AtomicUsize>,
) -> Result<ScanResult, IpCalculatorError> {
    let (hosts, total) = prepare_scan(cidr)?;
    completed.store(0, Ordering::Relaxed);
    let results = Arc::new(Mutex::new(Vec::new()));
    scan_hosts(&hosts, &results, Some(&completed));
    let responsive = results.lock().unwrap().clone();
    completed.store(total, Ordering::Relaxed);
    Ok(ScanResult {
        responsive_hosts: responsive,
        total_scanned: total,
    })
}

fn prepare_scan(cidr: &str) -> Result<(Vec<Ipv4Addr>, usize), IpCalculatorError> {
    let (ip_str, prefix_str) = cidr.split_once('/').ok_or_else(|| {
        IpCalculatorError::InvalidCIDR(format!("Invalid CIDR format: {}", cidr))
    })?;

    let ip: Ipv4Addr = ip_str.parse().map_err(|_| {
        IpCalculatorError::InvalidIP(format!("Invalid IP: {}", ip_str))
    })?;

    let prefix: u8 = prefix_str.parse().map_err(|_| {
        IpCalculatorError::InvalidPrefix(format!("Invalid prefix: {}", prefix_str))
    })?;

    if prefix < 16 {
        return Err(IpCalculatorError::InvalidCIDR(
            "Subnet too large to scan (minimum /16)".to_string(),
        ));
    }

    let subnet = Subnet::new(ip, prefix)?;
    let hosts = subnet.get_available_hosts()?;
    let total = hosts.len();
    Ok((hosts, total))
}

fn scan_hosts(
    hosts: &[Ipv4Addr],
    results: &Arc<Mutex<Vec<Ipv4Addr>>>,
    external_progress: Option<&Arc<AtomicUsize>>,
) {
    let total = hosts.len();
    let num_threads = MAX_CONCURRENT_PINGS.min(total);
    let internal_completed = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let thread_hosts: Vec<Ipv4Addr> = hosts
            .iter()
            .enumerate()
            .filter(|(i, _)| i % num_threads == thread_id)
            .map(|(_, h)| *h)
            .collect();
        let results = Arc::clone(results);
        let internal_completed = Arc::clone(&internal_completed);
        let external_progress = external_progress.map(|p| Arc::clone(p));

        handles.push(thread::spawn(move || {
            for host in &thread_hosts {
                if ping_host(host) {
                    if let Ok(mut r) = results.lock() {
                        r.push(*host);
                    }
                }
                let c = internal_completed.fetch_add(1, Ordering::SeqCst) + 1;
                if let Some(ref ext) = external_progress {
                    ext.store(c, Ordering::Relaxed);
                }
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn ping_host(ip: &Ipv4Addr) -> bool {
    let ip_str = ip.to_string();
    Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg("-W")
        .arg("1")
        .arg(&ip_str)
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}
