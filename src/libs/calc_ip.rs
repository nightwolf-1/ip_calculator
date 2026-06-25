use std::fmt;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub enum IpCalculatorError {
    InvalidPrefix(String),
    InvalidMask(String),
    InvalidIP(String),
    InvalidCIDR(String),
    InvalidRange(String),
    SubnetError(String),
    IpError(String),
    MaskError(String),
    RangeError(String),
    ConversionError(String),
    ArgumentsError(String),
}

impl std::error::Error for IpCalculatorError {}

impl std::fmt::Debug for IpCalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPrefix(msg) => write!(f, "InvalidPrefix({})", msg),
            Self::InvalidMask(msg) => write!(f, "InvalidMask({})", msg),
            Self::InvalidIP(msg) => write!(f, "InvalidIP({})", msg),
            Self::InvalidCIDR(msg) => write!(f, "InvalidCIDR({})", msg),
            Self::InvalidRange(msg) => write!(f, "InvalidRange({})", msg),
            Self::SubnetError(msg) => write!(f, "SubnetError({})", msg),
            Self::IpError(msg) => write!(f, "IpError({})", msg),
            Self::MaskError(msg) => write!(f, "MaskError({})", msg),
            Self::RangeError(msg) => write!(f, "RangeError({})", msg),
            Self::ConversionError(msg) => write!(f, "ConversionError({})", msg),
            Self::ArgumentsError(msg) => write!(f, "ArgumentsError({})", msg),
        }
    }
}

impl fmt::Display for IpCalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPrefix(msg) => write!(f, "Invalid prefix: {}", msg),
            Self::InvalidMask(msg) => write!(f, "Invalid mask: {}", msg),
            Self::InvalidIP(msg) => write!(f, "Invalid IP: {}", msg),
            Self::InvalidCIDR(msg) => write!(f, "Invalid CIDR: {}", msg),
            Self::InvalidRange(msg) => write!(f, "Invalid range: {}", msg),
            Self::SubnetError(msg) => write!(f, "Subnet error: {}", msg),
            Self::IpError(msg) => write!(f, "IP error: {}", msg),
            Self::MaskError(msg) => write!(f, "Mask error: {}", msg),
            Self::RangeError(msg) => write!(f, "Range error: {}", msg),
            Self::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            Self::ArgumentsError(msg) => write!(f, "Arguments error: {}", msg),
        }
    }
}
pub enum MaskOrCidr {
    Mask(Ipv4Addr),
    Cidr(u8),
}

impl MaskOrCidr {
    pub fn expect_mask(self) -> Ipv4Addr {
        match self {
            MaskOrCidr::Mask(m) => m,
            _ => unreachable!(),
        }
    }
    pub fn expect_cidr(self) -> u8 {
        match self {
            MaskOrCidr::Cidr(c) => c,
            _ => unreachable!(),
        }
    }
}

pub enum InputType {
    Cidr,
    Mask,
}

pub enum Command {
    Subnets {
        cidr: String,
        prefix: u8,
        filter: Option<usize>,
        page: Option<usize>,
        output_file: Option<String>,
    },
    GetSubnet {
        cidr: String,
        prefix: u8,
        index: u32,
    },
    SameSubnet {
        ip1: Ipv4Addr,
        ip2: Ipv4Addr,
        mask1: Ipv4Addr,
        mask2: Option<Ipv4Addr>,
    },
    CheckIP {
        ip: String,
    },
    CheckMask {
        mask: String,
    },
    FindRange {
        cidr: String,
        range_size: usize,
        exclusions: Vec<Ipv4Addr>,
    },
    Display {
        cidr: String,
    },
}

pub struct CommandHelp {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub short_desc: &'static str,
    pub long_desc: &'static str,
    pub usage: &'static str,
    pub examples: &'static [&'static str],
}

impl CommandHelp {
    pub fn get_all() -> Vec<CommandHelp> {
        vec![
            CommandHelp {
                name: "display",
                aliases: &[],
                short_desc: "Display subnet information",
                long_desc: "Display detailed information about a subnet, including network address, \
                           broadcast address, mask, number of available hosts, and usable IP range.",
                usage: "./ip_calculator <ip>/<cidr> or ./ip_calculator <ip> <mask or cidr>",
                examples: &[
                    "./ip_calculator 192.168.1.0/24",
                    "./ip_calculator 192.168.1.0 255.255.255.0",
                    "./ip_calculator 10.0.0.0 24",
                ],
            },
            CommandHelp {
                name: "subnets",
                aliases: &["-s", "--subnets"],
                short_desc: "Calculate network subnets",
                long_desc: "Divide a network into smaller subnets. The command ensures no subnet overlapping \
                           and provides detailed information for each subnet. Use the -f option to limit the \
                           number of displayed subnets by default 4 if is possible. Use -o to write all \
                           matching subnets to a file.",
                usage: "./ip_calculator (-s|--subnets) <CIDR> <new_prefix> [-f <number>] [-p <page (1-indexed)>] [-o <file>]",
                examples: &[
                    "./ip_calculator -s 192.168.1.0/24 26",
                    "./ip_calculator --subnets 10.0.0.0/8 16 -f 5",
                    "./ip_calculator -s 10.0.0.0/15 30 -p 10",
                    "./ip_calculator -s 10.0.0.0/16 24 -o output.txt"
                ],
            },
            CommandHelp {
                name: "get-subnet",
                aliases: &["--get-subnet"],
                short_desc: "Get a specific subnet by index",
                long_desc: "Retrieve information about a specific subnet by its index after dividing the \
                           network. The index starts at 0 and must be within the range of possible subnets.",
                usage: "./ip_calculator --get-subnet <CIDR> <new_prefix> <index>",
                examples: &[
                    "./ip_calculator --get-subnet 192.168.1.0/24 26 2",
                    "./ip_calculator --get-subnet 10.0.0.0/8 16 5",
                ],
            },
            CommandHelp {
                name: "same-subnet",
                aliases: &["-same", "--same-subnet"],
                short_desc: "Check if IPs are in the same subnet",
                long_desc: "Verify if two IP addresses belong to the same subnet. You can specify different \
                           masks for each IP. If only one mask is provided, it will be used for both IPs.",
                usage: "./ip_calculator (-same|--same-subnet) <IP1> <IP2> <mask1> [mask2]",
                examples: &[
                    "./ip_calculator -same 192.168.1.10 192.168.1.20 255.255.255.0",
                    "./ip_calculator --same-subnet 10.0.0.1 10.0.0.2 255.0.0.0 255.255.0.0",
                ],
            },
            CommandHelp {
                name: "check-ip",
                aliases: &["-cip", "--check-ip"],
                short_desc: "Validate IP address format",
                long_desc: "Check if an IP address is valid according to IPv4 format rules. Each octet \
                           must be between 0 and 255.",
                usage: "./ip_calculator (-cip|--check-ip) <IP>",
                examples: &[
                    "./ip_calculator -cip 192.168.1.1",
                    "./ip_calculator --check-ip 10.0.0.1",
                ],
            },
            CommandHelp {
                name: "check-mask",
                aliases: &["-cmask", "--check-mask"],
                short_desc: "Validate subnet mask",
                long_desc: "Verify if a subnet mask is valid. Can accept both traditional mask format \
                           (e.g., 255.255.255.0) and CIDR notation (e.g., 24).",
                usage: "./ip_calculator (-cmask|--check-mask) <mask|cidr>",
                examples: &[
                    "./ip_calculator -cmask 255.255.255.0",
                    "./ip_calculator --check-mask 24",
                ],
            },
            CommandHelp {
                name: "find-range",
                aliases: &["-fr", "--find-range"],
                short_desc: "Find available IP range",
                long_desc: "Find a continuous range of available IP addresses in a subnet. You can specify \
                           IP addresses to exclude from the search. The command will find the first available \
                           range that meets the size requirement.",
                usage: "./ip_calculator (-fr|--find-range) <CIDR> <range_size> [exclusions...]",
                examples: &[
                    "./ip_calculator -fr 192.168.1.0/24 10",
                    "./ip_calculator --find-range 10.0.0.0/24 5 10.0.0.1 10.0.0.2",
                ],
            },
            CommandHelp {
                name: "tui",
                aliases: &["-t", "--tui"],
                short_desc: "Launch terminal interface",
                long_desc: "Start the interactive terminal user interface with IP calculator \
                           and network scanner tools.",
                usage: "./ip_calculator (-t|--tui)",
                examples: &[
                    "./ip_calculator --tui",
                    "./ip_calculator -t",
                ],
            },
            CommandHelp {
                name: "help",
                aliases: &["-h", "--help"],
                short_desc: "Display help information",
                long_desc: "Show help information. Use without arguments to see all available commands, \
                           or specify a command name to get detailed help for that command.",
                usage: "./ip_calculator (-h|--help) [command_name]",
                examples: &[
                    "./ip_calculator --help",
                    "./ip_calculator --help subnets",
                ],
            },
        ]
    }

    pub fn find_by_name_or_alias(name: &str) -> Option<CommandHelp> {
        Self::get_all().into_iter().find(|cmd| {
            cmd.name == name || cmd.aliases.contains(&name)
        })
    }

    pub fn display_command_list() {
        println!("\x1b[1;32mIP Calculator - Available Commands:\x1b[0m\n");
        for cmd in Self::get_all() {
            let aliases = if cmd.aliases.is_empty() {
                String::new()
            } else {
                format!(" ({})", cmd.aliases.join(", "))
            };
            println!("\x1b[1;34m{}{}\x1b[0m", cmd.name, aliases);
            println!("    {}\n", cmd.short_desc);
        }
        println!("\nFor detailed help on a specific command, use: ./ip_calculator --help <command>");
    }

    pub fn display_command_help(&self) {
        println!("\x1b[1;32mHelp for command: {}\x1b[0m\n", self.name);
        
        if !self.aliases.is_empty() {
            println!("\x1b[1;34mAliases:\x1b[0m {}\n", self.aliases.join(", "));
        }
        
        println!("\x1b[1;34mDescription:\x1b[0m");
        println!("{}\n", self.long_desc);
        
        println!("\x1b[1;34mUsage:\x1b[0m");
        println!("{}\n", self.usage);
        
        println!("\x1b[1;34mExamples:\x1b[0m");
        for example in self.examples {
            println!("  {}", example);
        }
    }
}


#[derive(Debug)]
pub struct Subnet {
    pub network: Ipv4Addr,
    pub mask: Ipv4Addr,
    pub broadcast: Ipv4Addr,
    pub first_usable: Option<Ipv4Addr>,
    pub last_usable: Option<Ipv4Addr>,
    pub prefix: u8,
    pub num_hosts: u32,
}

impl Subnet {
    pub fn new(network: Ipv4Addr, prefix: u8) -> Result<Self, IpCalculatorError> {
        if prefix > 32 {
            return Err(IpCalculatorError::InvalidPrefix(
                "Prefix cannot exceed 32".to_string(),
            ));
        }

        let mask = Ipv4Addr::from(if prefix == 32 {
            u32::MAX
        } else {
            !(u32::MAX >> prefix)
        });
        let network_u32 = u32::from(network) & u32::from(mask);
        let broadcast_u32 = network_u32 | !u32::from(mask);

        let (first_usable, last_usable) = if prefix >= 31 {
            (
                Some(Ipv4Addr::from(network_u32)),
                Some(Ipv4Addr::from(broadcast_u32)),
            )
        } else {
            (
                Some(Ipv4Addr::from(network_u32 + 1)),
                Some(Ipv4Addr::from(broadcast_u32 - 1)),
            )
        };

        let num_hosts = match prefix {
            32 => 1,
            31 => 2,
            0 => u32::MAX - 1,
            _ => (1u32 << (32 - prefix)) - 2,
        };

        Ok(Subnet {
            network: Ipv4Addr::from(network_u32),
            broadcast: Ipv4Addr::from(broadcast_u32),
            first_usable,
            last_usable,
            prefix,
            mask,
            num_hosts,
        })
    }

    pub fn contains_ip(&self, ip: Ipv4Addr) -> bool {
        let network_start = u32::from(self.network);
        let network_end = u32::from(self.broadcast);
        let ip_u32 = u32::from(ip);
        ip_u32 >= network_start && ip_u32 <= network_end
    }

    pub fn get_available_hosts(&self) -> Result<Vec<Ipv4Addr>, IpCalculatorError> {
        let first = self.first_usable.ok_or_else(|| {
            IpCalculatorError::SubnetError("No first usable address available".to_string())
        })?;
        let last = self.last_usable.ok_or_else(|| {
            IpCalculatorError::SubnetError("No last usable address available".to_string())
        })?;

        Ok((u32::from(first)..=u32::from(last))
            .map(Ipv4Addr::from)
            .collect())
    }

    pub fn overlaps_with(&self, other: &Subnet) -> bool {
        self.contains_ip(other.network)
            || self.contains_ip(other.broadcast)
            || other.contains_ip(self.network)
            || other.contains_ip(self.broadcast)
    }
}

impl Subnet {
    pub fn to_plain_text(&self) -> String {
        format!(
            "Network: {}\nMask: {}\nCidr: {}\nBroadcast: {}\nFirst: {}\nLast: {}\nHosts: {}",
            self.network,
            self.mask,
            self.prefix,
            self.broadcast,
            self.first_usable.map_or("N/A".to_string(), |ip| ip.to_string()),
            self.last_usable.map_or("N/A".to_string(), |ip| ip.to_string()),
            self.num_hosts
        )
    }
}

impl fmt::Display for Subnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
                f,
                "\x1b[1;34mNetwork:\x1b[0m {}\n\x1b[1;34mMask:\x1b[0m {}\n\x1b[1;34mCidr:\x1b[0m {}\n\x1b[1;34mBroadcast:\x1b[0m {}\n\x1b[1;34mFirst:\x1b[0m {}\n\x1b[1;34mLast:\x1b[0m {}\n\x1b[1;34mHosts:\x1b[0m {}",
                self.network,
                self.mask,
                self.prefix,
                self.broadcast,
                self.first_usable.map_or("N/A".to_string(), |ip| ip.to_string()),
                self.last_usable.map_or("N/A".to_string(), |ip| ip.to_string()),
                self.num_hosts
            )
    }
}

pub fn calculate_subnet(cidr: &str) -> Result<Subnet, IpCalculatorError> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Err(IpCalculatorError::InvalidCIDR(
            "CIDR format must be IP/prefix".to_string(),
        ));
    }

    let ip = Ipv4Addr::from_str(parts[0])
        .map_err(|_| IpCalculatorError::InvalidIP("Invalid IP address format".to_string()))?;

    let prefix: u8 = parts[1].parse().map_err(|_| {
        IpCalculatorError::InvalidPrefix("Prefix must be a number between 0 and 32".to_string())
    })?;

    if prefix > 32 {
        return Err(IpCalculatorError::InvalidPrefix(
            "Prefix cannot exceed 32".to_string(),
        ));
    }

    Subnet::new(ip, prefix)
}

pub fn check_ip(ip: &str) -> Result<(), IpCalculatorError> {
    Ipv4Addr::from_str(ip).map_err(|_| IpCalculatorError::IpError(
        format!("Invalid IP address format: {} - IP addresses must be in the format xxx.xxx.xxx.xxx with values between 0 and 255", ip)
    )).map(|_| ())
}

pub fn are_in_same_subnet(
    ip1: Ipv4Addr,
    ip2: Ipv4Addr,
    mask1: Ipv4Addr,
    mask2: Option<Ipv4Addr>,
) -> Result<bool, IpCalculatorError> {
    let prefix1 = mask_to_cidr(mask1)?;
    let subnet1 = Subnet::new(ip1, prefix1)?;

    let mask2 = mask2.unwrap_or(mask1);
    let prefix2 = mask_to_cidr(mask2)?;
    let subnet2 = Subnet::new(ip2, prefix2)?;

    Ok(subnet1.contains_ip(ip2) && subnet2.contains_ip(ip1))
}

pub fn generate_subnets(
    cidr: &str,
    new_prefix: u8,
    filter: Option<usize>,
    page: Option<usize>,
) -> Result<(Vec<Subnet>, u32, usize, usize), IpCalculatorError> {
    let subnet = calculate_subnet(cidr)?;

    if new_prefix > 32 {
        return Err(IpCalculatorError::InvalidPrefix(
            "The new prefix cannot exceed 32".to_string(),
        ));
    }

    if new_prefix <= subnet.prefix {
        return Err(IpCalculatorError::InvalidPrefix(format!(
            "New prefix ({}) must be greater than current prefix ({})",
            new_prefix, subnet.prefix
        )));
    }

    let num_subnets = 1u32
        .checked_shl((new_prefix - subnet.prefix) as u32)
        .ok_or_else(|| IpCalculatorError::SubnetError("Unable to calculate subnets".to_string()))?;

    let base_network = u32::from(subnet.network);
    let increment = 1u32.checked_shl(32 - new_prefix as u32)
        .ok_or_else(|| IpCalculatorError::SubnetError(
            "Subnet increment calculation overflow".to_string(),
        ))?;

    let page_size = 4;
    let max_to_show = std::cmp::min(num_subnets as usize, filter.unwrap_or(usize::MAX));
    let total_pages = if max_to_show > 0 {
        (max_to_show + page_size - 1) / page_size
    } else {
        0
    };
    
    let page_number = match page {
        Some(_) if total_pages == 0 => 0,
        Some(p) if p >= total_pages => total_pages - 1,
        Some(p) => p,
        None => 0,
    };

    let start_index = page_number * page_size;
    let mut subnets: Vec<Subnet> = Vec::new();
    let end_index = std::cmp::min(
        start_index + page_size,
        max_to_show
    );

    for i in start_index..end_index {
        let new_network = base_network.checked_add((i as u32) * increment).ok_or_else(|| {
            IpCalculatorError::SubnetError(
                "Overflow occurred while calculating subnets".to_string(),
            )
        })?;
        
        let new_subnet = Subnet::new(Ipv4Addr::from(new_network), new_prefix)?;
        
        if subnets.iter().any(|existing| existing.overlaps_with(&new_subnet)) {
            return Err(IpCalculatorError::SubnetError(
                format!("Generated subnet {} overlaps with existing subnets - this should not happen!", 
                    new_subnet.network
                )
            ));
        }
        
        subnets.push(new_subnet);
    }

    Ok((subnets, num_subnets, total_pages, page_number))
}

pub fn generate_all_subnets(
    cidr: &str,
    new_prefix: u8,
    filter: Option<usize>,
) -> Result<Vec<Subnet>, IpCalculatorError> {
    let subnet = calculate_subnet(cidr)?;

    if new_prefix > 32 {
        return Err(IpCalculatorError::InvalidPrefix(
            "The new prefix cannot exceed 32".to_string(),
        ));
    }

    if new_prefix <= subnet.prefix {
        return Err(IpCalculatorError::InvalidPrefix(format!(
            "New prefix ({}) must be greater than current prefix ({})",
            new_prefix, subnet.prefix
        )));
    }

    let num_subnets = 1u32
        .checked_shl((new_prefix - subnet.prefix) as u32)
        .ok_or_else(|| IpCalculatorError::SubnetError("Unable to calculate subnets".to_string()))?;

    let base_network = u32::from(subnet.network);
    let increment = 1u32.checked_shl(32 - new_prefix as u32)
        .ok_or_else(|| IpCalculatorError::SubnetError(
            "Subnet increment calculation overflow".to_string(),
        ))?;

    let count = std::cmp::min(num_subnets as usize, filter.unwrap_or(num_subnets as usize));
    let mut subnets: Vec<Subnet> = Vec::with_capacity(count);

    for i in 0..count {
        let new_network = base_network.checked_add((i as u32) * increment).ok_or_else(|| {
            IpCalculatorError::SubnetError(
                "Overflow occurred while calculating subnets".to_string(),
            )
        })?;

        let new_subnet = Subnet::new(Ipv4Addr::from(new_network), new_prefix)?;

        if subnets.iter().any(|existing| existing.overlaps_with(&new_subnet)) {
            return Err(IpCalculatorError::SubnetError(
                format!("Generated subnet {} overlaps with existing subnets - this should not happen!",
                    new_subnet.network
                )
            ));
        }

        subnets.push(new_subnet);
    }

    Ok(subnets)
}

pub fn get_subnet(cidr: &str, new_prefix: u8, index: u32) -> Result<(), IpCalculatorError> {
    let base_subnet = calculate_subnet(cidr)
        .map_err(|_| IpCalculatorError::InvalidCIDR(format!(
            "Invalid CIDR format: {}", cidr
        )))?;

    if new_prefix > 32 {
        return Err(IpCalculatorError::InvalidPrefix(
            "The prefix must be between 0 and 32".to_string(),
        ));
    }

    if new_prefix <= base_subnet.prefix {
        return Err(IpCalculatorError::InvalidPrefix(format!(
            "New prefix ({}) must be larger than current prefix ({})",
            new_prefix, base_subnet.prefix
        )));
    }

    let num_subnets = 1u32.checked_shl((new_prefix - base_subnet.prefix) as u32)
        .ok_or_else(|| IpCalculatorError::SubnetError(
            "Cannot calculate number of subnets - value overflow".to_string(),
        ))?;

    let actual_index = if index >= num_subnets {
        println!(
            "Warning: Requested subnet index {} exceeds available subnets ({})",
            index, num_subnets
        );
        num_subnets - 1
    } else {
        index
    };

    let base_network = u32::from(base_subnet.network);
    let increment = 1u32.checked_shl(32 - new_prefix as u32)
        .ok_or_else(|| IpCalculatorError::SubnetError(
            "Subnet increment calculation overflow".to_string(),
        ))?;

    let new_network = base_network.checked_add(actual_index * increment)
        .ok_or_else(|| IpCalculatorError::SubnetError(
            "Network address calculation overflow".to_string(),
        ))?;

    let subnet = Subnet::new(Ipv4Addr::from(new_network), new_prefix)?;
    println!("{}", subnet);
    Ok(())
}

pub fn find_ip_range(
    cidr: &str,
    range_size: usize,
    exclusions: Vec<Ipv4Addr>,
) -> Result<(Ipv4Addr, Ipv4Addr), IpCalculatorError> {
    if range_size == 0 {
        return Err(IpCalculatorError::InvalidRange(
            "Range size must be greater than 0".to_string(),
        ));
    }

    let subnet = calculate_subnet(cidr)?;
    let available_ips: Vec<_> = subnet
        .get_available_hosts()?
        .into_iter()
        .filter(|ip| !exclusions.contains(ip))
        .collect();

    if available_ips.is_empty() {
        return Err(IpCalculatorError::RangeError(
            "No available IPs in subnet".to_string(),
        ));
    }

    let mut best_start = None;
    let mut current_start = 0;
    let mut current_len = 0;

    for (i, ip) in available_ips.iter().enumerate() {
        if i > 0 && u32::from(*ip) != u32::from(available_ips[i - 1]) + 1 {
            current_start = i;
            current_len = 1;
        } else {
            current_len += 1;
        }

        if current_len >= range_size {
            best_start = Some(current_start);
            break;
        }
    }

    best_start
        .map(|start| (available_ips[start], available_ips[start + range_size - 1]))
        .ok_or_else(|| IpCalculatorError::InvalidRange("No suitable range found".to_string()))
}

pub fn is_cidr_or_mask(input: &str) -> Result<InputType, IpCalculatorError> {
    if check_mask(input)? {
        if let Ok(cidr) = input.parse::<u8>() {
            if cidr <= 32 {
                return Ok(InputType::Cidr);
            }
        } else if Ipv4Addr::from_str(input).is_ok() {
            return Ok(InputType::Mask);
        }
    }
    Err(IpCalculatorError::ConversionError(
        "Invalid CIDR or mask format".to_string(),
    ))
}

pub fn cidr_to_mask(cidr: u8) -> Result<Ipv4Addr, IpCalculatorError> {
    if cidr > 32 {
        return Err(IpCalculatorError::InvalidPrefix(
            "CIDR prefix cannot exceed 32".to_string(),
        ));
    }
    Ok(Ipv4Addr::from(if cidr == 32 {
        u32::MAX
    } else {
        !(u32::MAX >> cidr)
    }))
}

pub fn mask_to_cidr(mask: Ipv4Addr) -> Result<u8, IpCalculatorError> {
    let mask_u32 = u32::from(mask);

    if mask_u32 == 0 {
        return Ok(0);
    }

    let inverted = !mask_u32;

    if !inverted.wrapping_add(1).is_power_of_two() {
        return Err(IpCalculatorError::InvalidMask(
            "Invalid subnet mask format".to_string(),
        ));
    }

    Ok(mask_u32.count_ones() as u8)
}

pub fn check_mask(mask: &str) -> Result<bool, IpCalculatorError> {
    if let Ok(cidr) = mask.parse::<u8>() {
        return Ok(cidr <= 32);
    }

    let mask_addr = Ipv4Addr::from_str(mask)
        .map_err(|_| IpCalculatorError::InvalidMask("Invalid mask format".to_string()))?;

    let mask_u32 = u32::from(mask_addr);
    if mask_u32 == 0 {
        return Ok(true);
    }
    let inverted = !mask_u32;
    Ok(inverted.wrapping_add(1).is_power_of_two())
}

pub fn parse_mask_or_cidr(input: &str, return_type: InputType) -> Result<MaskOrCidr, IpCalculatorError> {
    let input_type = is_cidr_or_mask(input)?;

    match (&input_type, &return_type) {
        (InputType::Mask, InputType::Cidr) => {
            let mask = Ipv4Addr::from_str(input).map_err(|_| 
                IpCalculatorError::MaskError(
                    format!("Invalid mask format: {}", input)
                )
            )?;
            let cidr = mask_to_cidr(mask)?;
            Ok(MaskOrCidr::Cidr(cidr))
        },
        (InputType::Cidr, InputType::Mask) => {
            let cidr = input.parse::<u8>().map_err(|_| 
                IpCalculatorError::InvalidPrefix(
                    format!("Invalid CIDR prefix: {}", input)
                )
            )?;
            let mask = cidr_to_mask(cidr)?;
            Ok(MaskOrCidr::Mask(mask))
        },
        (InputType::Mask, InputType::Mask) => {
            let mask = Ipv4Addr::from_str(input).map_err(|_|
                IpCalculatorError::MaskError(
                    format!("Invalid mask format: {}", input)
                )
            )?;
            Ok(MaskOrCidr::Mask(mask))
        },
        (InputType::Cidr, InputType::Cidr) => {
            let cidr = input.parse::<u8>().map_err(|_|
                IpCalculatorError::InvalidPrefix(
                    format!("Invalid CIDR prefix: {}", input)
                )
            )?;
            Ok(MaskOrCidr::Cidr(cidr))
        },
    }
}

pub fn execute_command(command: Command) -> Result<(), IpCalculatorError> {
    match command {
        Command::Subnets { cidr, prefix, filter, page, output_file } => {
            let (subnets, total_subnets, total_pages, page_number) = generate_subnets(&cidr, prefix, filter, page)
                .map_err(|e| IpCalculatorError::SubnetError(
                    format!("Failed to generate subnets: {}", e)
                ))?;
            
            let displayed = subnets.len();
            let requested_page = page.unwrap_or(0);    
            
            for subnet in &subnets {
                println!("{}", subnet);
                println!("----------------------------");
            }
            if requested_page >= total_pages && total_pages > 0 {
                println!("\x1b[1;33mWarning: Only {} pages available. Showing last page.\x1b[0m", total_pages);
            }
            println!("\nTotal subnets: {} | Subnets displayed: {}", total_subnets, displayed);
            if total_pages > 0 {
                println!("Page {}/{}", page_number + 1, total_pages);
            }

            if let Some(path) = output_file {
                let all = generate_all_subnets(&cidr, prefix, filter)
                    .map_err(|e| IpCalculatorError::SubnetError(
                        format!("Failed to generate subnets for output file: {}", e)
                    ))?;
                let content = all.iter()
                    .map(|s| s.to_plain_text())
                    .collect::<Vec<_>>()
                    .join("\n----------------------------\n");
                std::fs::write(&path, content)
                    .map_err(|e| IpCalculatorError::SubnetError(
                        format!("Failed to write output file '{}': {}", path, e)
                    ))?;
                println!("Output written to {}", path);
            }
            
            Ok(())
        },
        Command::GetSubnet { cidr, prefix, index } => {
            get_subnet(&cidr, prefix, index)
                .map_err(|e| IpCalculatorError::SubnetError(
                    format!("Failed to get subnet {}: {}", index, e)
                ))
        },
        Command::SameSubnet { ip1, ip2, mask1, mask2 } => {
            let result = are_in_same_subnet(ip1, ip2, mask1, mask2)
                .map_err(|e| IpCalculatorError::SubnetError(
                    format!("Failed to compare subnets: {}", e)
                ))?;
            
            println!(
                "IP addresses {} and {} {} in the same subnet",
                ip1, ip2,
                if result { "are" } else { "are not" }
            );
            Ok(())
        },
        Command::CheckIP { ip } => {
            check_ip(&ip).map_err(|_| IpCalculatorError::InvalidIP(
                format!("Invalid IP address format: {}", ip)
            ))?;
            println!("IP address {} is valid", ip);
            Ok(())
        },
        Command::CheckMask { mask } => {
            match check_mask(&mask)? {
                true => {
                    println!("Subnet mask {} is valid", mask);
                    Ok(())
                },
                false => Err(IpCalculatorError::InvalidMask(
                    format!("Invalid subnet mask format: {}", mask)
                ))
            }
        },
        Command::FindRange { cidr, range_size, exclusions } => {
            let (start, end) = find_ip_range(&cidr, range_size, exclusions)
                .map_err(|e| IpCalculatorError::RangeError(
                    format!("Failed to find IP range: {}", e)
                ))?;
            println!("Available IP range: {} - {}", start, end);
            Ok(())
        },
        Command::Display { cidr } => {
            let subnet = calculate_subnet(&cidr)
                .map_err(|e| IpCalculatorError::InvalidCIDR(
                    format!("Failed to calculate subnet for {}: {}", cidr, e)
                ))?;
            println!("{}", subnet);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subnet_new_basic() {
        let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 0), 24).unwrap();
        assert_eq!(subnet.network, Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(subnet.mask, Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(subnet.broadcast, Ipv4Addr::new(192, 168, 1, 255));
        assert_eq!(subnet.first_usable, Some(Ipv4Addr::new(192, 168, 1, 1)));
        assert_eq!(subnet.last_usable, Some(Ipv4Addr::new(192, 168, 1, 254)));
        assert_eq!(subnet.prefix, 24);
        assert_eq!(subnet.num_hosts, 254);
    }

    #[test]
    fn test_subnet_new_invalid_prefix() {
        let err = Subnet::new(Ipv4Addr::new(0, 0, 0, 0), 33).unwrap_err();
        assert!(matches!(err, IpCalculatorError::InvalidPrefix(_)));
    }

    #[test]
    fn test_subnet_new_prefix_31() {
        let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 31).unwrap();
        assert_eq!(subnet.network, Ipv4Addr::new(10, 0, 0, 0));
        assert_eq!(subnet.mask, Ipv4Addr::new(255, 255, 255, 254));
        assert_eq!(subnet.broadcast, Ipv4Addr::new(10, 0, 0, 1));
        assert_eq!(subnet.first_usable, Some(Ipv4Addr::new(10, 0, 0, 0)));
        assert_eq!(subnet.last_usable, Some(Ipv4Addr::new(10, 0, 0, 1)));
        assert_eq!(subnet.num_hosts, 2);
    }

    #[test]
    fn test_subnet_new_prefix_32() {
        let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 5), 32).unwrap();
        assert_eq!(subnet.network, Ipv4Addr::new(10, 0, 0, 5));
        assert_eq!(subnet.mask, Ipv4Addr::new(255, 255, 255, 255));
        assert_eq!(subnet.broadcast, Ipv4Addr::new(10, 0, 0, 5));
        assert_eq!(subnet.first_usable, Some(Ipv4Addr::new(10, 0, 0, 5)));
        assert_eq!(subnet.last_usable, Some(Ipv4Addr::new(10, 0, 0, 5)));
        assert_eq!(subnet.num_hosts, 1);
    }

    #[test]
    fn test_subnet_new_prefix_0() {
        let subnet = Subnet::new(Ipv4Addr::new(0, 0, 0, 0), 0).unwrap();
        assert_eq!(subnet.network, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(subnet.mask, Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(subnet.prefix, 0);
        assert_eq!(subnet.num_hosts, u32::MAX - 1);
    }

    #[test]
    fn test_subnet_network_alignment() {
        let subnet = Subnet::new(Ipv4Addr::new(192, 168, 1, 42), 24).unwrap();
        assert_eq!(subnet.network, Ipv4Addr::new(192, 168, 1, 0));
    }

    #[test]
    fn test_contains_ip() {
        let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 24).unwrap();
        assert!(subnet.contains_ip(Ipv4Addr::new(10, 0, 0, 1)));
        assert!(subnet.contains_ip(Ipv4Addr::new(10, 0, 0, 255)));
        assert!(subnet.contains_ip(Ipv4Addr::new(10, 0, 0, 0)));
        assert!(!subnet.contains_ip(Ipv4Addr::new(10, 0, 1, 0)));
        assert!(!subnet.contains_ip(Ipv4Addr::new(11, 0, 0, 1)));
    }

    #[test]
    fn test_get_available_hosts_24() {
        let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 24).unwrap();
        let hosts = subnet.get_available_hosts().unwrap();
        assert_eq!(hosts.len(), 254);
        assert_eq!(hosts[0], Ipv4Addr::new(10, 0, 0, 1));
        assert_eq!(hosts[253], Ipv4Addr::new(10, 0, 0, 254));
    }

    #[test]
    fn test_get_available_hosts_31() {
        let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 31).unwrap();
        let hosts = subnet.get_available_hosts().unwrap();
        assert_eq!(hosts.len(), 2);
        assert_eq!(hosts[0], Ipv4Addr::new(10, 0, 0, 0));
        assert_eq!(hosts[1], Ipv4Addr::new(10, 0, 0, 1));
    }

    #[test]
    fn test_get_available_hosts_32() {
        let subnet = Subnet::new(Ipv4Addr::new(10, 0, 0, 5), 32).unwrap();
        let hosts = subnet.get_available_hosts().unwrap();
        assert_eq!(hosts.len(), 1);
        assert_eq!(hosts[0], Ipv4Addr::new(10, 0, 0, 5));
    }

    #[test]
    fn test_overlaps_with() {
        let a = Subnet::new(Ipv4Addr::new(10, 0, 0, 0), 24).unwrap();
        let b = Subnet::new(Ipv4Addr::new(10, 0, 0, 128), 25).unwrap();
        assert!(a.overlaps_with(&b));
        assert!(b.overlaps_with(&a));

        let c = Subnet::new(Ipv4Addr::new(10, 0, 1, 0), 24).unwrap();
        assert!(!a.overlaps_with(&c));
        assert!(!c.overlaps_with(&a));
    }

    #[test]
    fn test_calculate_subnet() {
        let subnet = calculate_subnet("192.168.1.0/24").unwrap();
        assert_eq!(subnet.network, Ipv4Addr::new(192, 168, 1, 0));
        assert_eq!(subnet.prefix, 24);
    }

    #[test]
    fn test_calculate_subnet_invalid_format() {
        let err = calculate_subnet("192.168.1.0").unwrap_err();
        assert!(matches!(err, IpCalculatorError::InvalidCIDR(_)));
    }

    #[test]
    fn test_calculate_subnet_invalid_ip() {
        let err = calculate_subnet("999.999.999.999/24").unwrap_err();
        assert!(matches!(err, IpCalculatorError::InvalidIP(_)));
    }

    #[test]
    fn test_check_ip_valid() {
        assert!(check_ip("192.168.1.1").is_ok());
        assert!(check_ip("0.0.0.0").is_ok());
        assert!(check_ip("255.255.255.255").is_ok());
    }

    #[test]
    fn test_check_ip_invalid() {
        assert!(check_ip("999.999.999.999").is_err());
        assert!(check_ip("not-an-ip").is_err());
        assert!(check_ip("256.0.0.0").is_err());
    }

    #[test]
    fn test_check_mask() {
        assert_eq!(check_mask("24").unwrap(), true);
        assert_eq!(check_mask("0").unwrap(), true);
        assert_eq!(check_mask("32").unwrap(), true);
        assert_eq!(check_mask("33").unwrap(), false);
        assert_eq!(check_mask("255.255.255.0").unwrap(), true);
        assert_eq!(check_mask("0.0.0.0").unwrap(), true);
        assert_eq!(check_mask("255.255.255.255").unwrap(), true);
        assert!(check_mask("not-a-mask").is_err());
    }

    #[test]
    fn test_cidr_to_mask() {
        assert_eq!(cidr_to_mask(0).unwrap(), Ipv4Addr::new(0, 0, 0, 0));
        assert_eq!(cidr_to_mask(24).unwrap(), Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(cidr_to_mask(32).unwrap(), Ipv4Addr::new(255, 255, 255, 255));
        assert!(cidr_to_mask(33).is_err());
    }

    #[test]
    fn test_mask_to_cidr() {
        assert_eq!(mask_to_cidr(Ipv4Addr::new(0, 0, 0, 0)).unwrap(), 0);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 0)).unwrap(), 24);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 255, 255, 255)).unwrap(), 32);
        assert_eq!(mask_to_cidr(Ipv4Addr::new(255, 0, 0, 0)).unwrap(), 8);
        assert!(mask_to_cidr(Ipv4Addr::new(255, 0, 0, 1)).is_err());
    }

    #[test]
    fn test_is_cidr_or_mask() {
        assert!(matches!(is_cidr_or_mask("24").unwrap(), InputType::Cidr));
        assert!(matches!(is_cidr_or_mask("0").unwrap(), InputType::Cidr));
        assert!(matches!(is_cidr_or_mask("32").unwrap(), InputType::Cidr));
        assert!(matches!(is_cidr_or_mask("255.255.255.0").unwrap(), InputType::Mask));
        assert!(matches!(is_cidr_or_mask("0.0.0.0").unwrap(), InputType::Mask));
        assert!(is_cidr_or_mask("33").is_err());
        assert!(is_cidr_or_mask("invalid").is_err());
    }

    #[test]
    fn test_parse_mask_or_cidr() {
        let result = parse_mask_or_cidr("24", InputType::Mask).unwrap();
        assert_eq!(result.expect_mask(), Ipv4Addr::new(255, 255, 255, 0));

        let result = parse_mask_or_cidr("255.255.255.0", InputType::Cidr).unwrap();
        assert_eq!(result.expect_cidr(), 24);

        let result = parse_mask_or_cidr("24", InputType::Cidr).unwrap();
        assert_eq!(result.expect_cidr(), 24);

        let result = parse_mask_or_cidr("255.255.255.0", InputType::Mask).unwrap();
        assert_eq!(result.expect_mask(), Ipv4Addr::new(255, 255, 255, 0));
    }

    #[test]
    fn test_are_in_same_subnet_same_mask() {
        let result = are_in_same_subnet(
            Ipv4Addr::new(10, 0, 0, 1),
            Ipv4Addr::new(10, 0, 0, 2),
            Ipv4Addr::new(255, 255, 255, 0),
            None,
        ).unwrap();
        assert!(result);
    }

    #[test]
    fn test_are_in_same_subnet_different_subnets() {
        let result = are_in_same_subnet(
            Ipv4Addr::new(10, 0, 0, 1),
            Ipv4Addr::new(10, 0, 1, 2),
            Ipv4Addr::new(255, 255, 255, 0),
            None,
        ).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_are_in_same_subnet_different_masks() {
        let result = are_in_same_subnet(
            Ipv4Addr::new(10, 0, 0, 1),
            Ipv4Addr::new(10, 0, 0, 2),
            Ipv4Addr::new(255, 0, 0, 0),
            Some(Ipv4Addr::new(255, 255, 255, 0)),
        ).unwrap();
        assert!(result);
    }

    #[test]
    fn test_generate_subnets_basic() {
        let (subnets, total, pages, page_num) = generate_subnets("10.0.0.0/24", 26, None, None).unwrap();
        assert_eq!(subnets.len(), 4);
        assert_eq!(total, 4);
        assert_eq!(pages, 1);
        assert_eq!(page_num, 0);
        assert_eq!(subnets[0].network, Ipv4Addr::new(10, 0, 0, 0));
        assert_eq!(subnets[1].network, Ipv4Addr::new(10, 0, 0, 64));
        assert_eq!(subnets[2].network, Ipv4Addr::new(10, 0, 0, 128));
        assert_eq!(subnets[3].network, Ipv4Addr::new(10, 0, 0, 192));
    }

    #[test]
    fn test_generate_subnets_with_filter() {
        let (subnets, total, _, _) = generate_subnets("10.0.0.0/8", 16, Some(3), None).unwrap();
        assert_eq!(subnets.len(), 3);
        assert_eq!(total, 256);
    }

    #[test]
    fn test_generate_subnets_invalid_new_prefix() {
        let err = generate_subnets("10.0.0.0/24", 24, None, None).unwrap_err();
        assert!(matches!(err, IpCalculatorError::InvalidPrefix(_)));
    }

    #[test]
    fn test_get_subnet() {
        assert!(get_subnet("10.0.0.0/24", 26, 2).is_ok());
    }

    #[test]
    fn test_get_subnet_out_of_range() {
        assert!(get_subnet("10.0.0.0/24", 26, 100).is_ok());
    }

    #[test]
    fn test_find_ip_range_basic() {
        let (start, end) = find_ip_range("192.168.1.0/24", 5, vec![]).unwrap();
        assert_eq!(start, Ipv4Addr::new(192, 168, 1, 1));
        assert_eq!(end, Ipv4Addr::new(192, 168, 1, 5));
    }

    #[test]
    fn test_find_ip_range_with_exclusions() {
        let exclusions = vec![
            Ipv4Addr::new(192, 168, 1, 1),
            Ipv4Addr::new(192, 168, 1, 2),
        ];
        let (start, end) = find_ip_range("192.168.1.0/24", 3, exclusions).unwrap();
        assert_eq!(start, Ipv4Addr::new(192, 168, 1, 3));
        assert_eq!(end, Ipv4Addr::new(192, 168, 1, 5));
    }

    #[test]
    fn test_find_ip_range_too_large() {
        let err = find_ip_range("10.0.0.0/30", 5, vec![]).unwrap_err();
        assert!(matches!(err, IpCalculatorError::InvalidRange(_)));
    }

    #[test]
    fn test_find_ip_range_zero_size() {
        let err = find_ip_range("10.0.0.0/24", 0, vec![]).unwrap_err();
        assert!(matches!(err, IpCalculatorError::InvalidRange(_)));
    }

    #[test]
    fn test_command_help_find_by_name() {
        let help = CommandHelp::find_by_name_or_alias("subnets").unwrap();
        assert_eq!(help.name, "subnets");
    }

    #[test]
    fn test_command_help_find_by_alias() {
        let help = CommandHelp::find_by_name_or_alias("-s").unwrap();
        assert_eq!(help.name, "subnets");
    }

    #[test]
    fn test_command_help_find_unknown() {
        assert!(CommandHelp::find_by_name_or_alias("nonexistent").is_none());
    }

    #[test]
    fn test_execute_command_display() {
        let cmd = Command::Display { cidr: "10.0.0.0/24".to_string() };
        assert!(execute_command(cmd).is_ok());
    }

    #[test]
    fn test_execute_command_check_ip_valid() {
        let cmd = Command::CheckIP { ip: "192.168.1.1".to_string() };
        assert!(execute_command(cmd).is_ok());
    }

    #[test]
    fn test_execute_command_check_ip_invalid() {
        let cmd = Command::CheckIP { ip: "bad".to_string() };
        assert!(execute_command(cmd).is_err());
    }

    #[test]
    fn test_execute_command_check_mask() {
        let cmd = Command::CheckMask { mask: "24".to_string() };
        assert!(execute_command(cmd).is_ok());

        let cmd = Command::CheckMask { mask: "33".to_string() };
        assert!(execute_command(cmd).is_err());
    }

    #[test]
    fn test_execute_command_same_subnet() {
        let cmd = Command::SameSubnet {
            ip1: Ipv4Addr::new(10, 0, 0, 1),
            ip2: Ipv4Addr::new(10, 0, 0, 2),
            mask1: Ipv4Addr::new(255, 255, 255, 0),
            mask2: None,
        };
        assert!(execute_command(cmd).is_ok());
    }

    #[test]
    fn test_execute_command_get_subnet() {
        let cmd = Command::GetSubnet {
            cidr: "10.0.0.0/24".to_string(),
            prefix: 26,
            index: 1,
        };
        assert!(execute_command(cmd).is_ok());
    }

    #[test]
    fn test_execute_command_find_range() {
        let cmd = Command::FindRange {
            cidr: "10.0.0.0/24".to_string(),
            range_size: 5,
            exclusions: vec![],
        };
        assert!(execute_command(cmd).is_ok());
    }
}
