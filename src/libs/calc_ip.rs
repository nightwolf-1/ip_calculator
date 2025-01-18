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

pub enum Command {
    Subnets {
        cidr: String,
        prefix: u8,
        filter: Option<usize>,
        page: Option<usize>
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
                aliases: &[""],
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
                           number of displayed subnets by default 512 if is possible.",
                usage: "./ip_calculator (-s|--subnets) <CIDR> <new_prefix> ([-f <number_of_subnets>]|[-p <page_number>])",
                examples: &[
                    "./ip_calculator -s 192.168.1.0/24 26",
                    "./ip_calculator --subnets 10.0.0.0/8 16 -f 5",
                    "./ip_calculator -s 10.0.0.0/15 30 -p 10"
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

        let mask = Ipv4Addr::from(!(u32::MAX >> prefix));
        let network_u32 = u32::from(network) & u32::from(mask);
        let broadcast_u32 = network_u32 | !u32::from(mask);

        let (first_usable, last_usable) = if prefix < 31 {
            (
                Some(Ipv4Addr::from(network_u32 + 1)),
                Some(Ipv4Addr::from(broadcast_u32 - 1)),
            )
        } else {
            (None, None)
        };

        let num_hosts = match prefix {
            32 => 1,
            31 => 2,
            _ => 2u32.pow((32 - prefix) as u32) - 2,
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
        if self.prefix >= 31 {
            return Err(IpCalculatorError::SubnetError(
                "No usable hosts in /31 or /32 networks".to_string(),
            ));
        }

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

pub fn check_ip(ip: &str) -> Result<bool, IpCalculatorError> {
    match Ipv4Addr::from_str(ip) {
        Ok(_) => Ok(true),
        Err(_) => Err(IpCalculatorError::IpError(
            format!("Invalid IP address format: {} - IP addresses must be in the format xxx.xxx.xxx.xxx with values between 0 and 255", ip)
        ))
    }
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
) -> Result<(Vec<Subnet>, u32, usize), IpCalculatorError> {
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
    let increment = 1u32 << (32 - new_prefix);

    let page_size = 512;
    let total_to_display = filter.unwrap_or(num_subnets as usize);
    let total_pages = (total_to_display + page_size - 1) / page_size;
    
    let page_number = match page {
        Some(p) if p >= total_pages => {
            if total_pages > 0 {
                total_pages - 1
            } else {
                0
            }
        },
        Some(p) => p,
        None => 0,
    };

    let start_index = page_number * page_size;
    let mut subnets: Vec<Subnet> = Vec::new();
    let end_index = std::cmp::min(
        start_index + page_size,
        std::cmp::min(num_subnets as usize, total_to_display)
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

    Ok((subnets, num_subnets, total_pages))
}

pub fn get_subnet(cidr: &str, new_prefix: u8, index: u32) -> Result<(), IpCalculatorError> {
    let base_subnet = match calculate_subnet(cidr) {
        Ok(subnet) => subnet,
        Err(_) => {
            return Err(IpCalculatorError::InvalidCIDR(format!(
                "Invalid CIDR format: {}",
                cidr
            )))
        }
    };

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

    let num_subnets = match 1u32.checked_shl((new_prefix - base_subnet.prefix) as u32) {
        Some(n) => n,
        None => {
            return Err(IpCalculatorError::SubnetError(
                "Cannot calculate number of subnets - value overflow".to_string(),
            ))
        }
    };

    if index >= num_subnets {
        println!(
            "Warning: Requested subnet index {} exceeds available subnets ({})",
            index, num_subnets
        );

        let last_index = num_subnets - 1;
        let base_network = u32::from(base_subnet.network);
        let increment = 1u32 << (32 - new_prefix);

        let new_network = match base_network.checked_add(last_index * increment) {
            Some(n) => n,
            None => {
                return Err(IpCalculatorError::SubnetError(
                    "Network address calculation overflow".to_string(),
                ))
            }
        };

        match Subnet::new(Ipv4Addr::from(new_network), new_prefix) {
            Ok(subnet) => println!("{}", subnet),
            Err(e) => return Err(e),
        }
    } else {
        let base_network = u32::from(base_subnet.network);
        let increment = 1u32 << (32 - new_prefix);

        let new_network = match base_network.checked_add(index * increment) {
            Some(n) => n,
            None => {
                return Err(IpCalculatorError::SubnetError(
                    "Network address calculation overflow".to_string(),
                ))
            }
        };

        match Subnet::new(Ipv4Addr::from(new_network), new_prefix) {
            Ok(subnet) => println!("{}", subnet),
            Err(e) => return Err(e),
        }
    }

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

pub fn is_cidr_or_mask(input: &str) -> Result<&'static str, IpCalculatorError> {
    if check_mask(input)? {
        if let Ok(cidr) = input.parse::<u8>() {
            if cidr <= 32 {
                return Ok("CIDR");
            }
        } else if Ipv4Addr::from_str(input).is_ok() {
            return Ok("Mask");
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
    Ok(Ipv4Addr::from(!(u32::MAX >> cidr)))
}

pub fn mask_to_cidr(mask: Ipv4Addr) -> Result<u8, IpCalculatorError> {
    let mask_u32 = u32::from(mask);
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
    let inverted = !mask_u32;
    Ok(inverted.wrapping_add(1).is_power_of_two())
}

pub fn parse_mask_or_cidr(input: &str, return_type: &str) -> Result<MaskOrCidr, IpCalculatorError> {
    let input_type = match is_cidr_or_mask(input)? {
        "Mask" => "Mask",
        "CIDR" => "CIDR",
        _ => return Err(IpCalculatorError::ConversionError(
            format!("Input '{}' is neither a valid mask nor CIDR", input)
        ))
    };

    match (input_type, return_type) {
        ("Mask", "cidr") => {
            let mask = Ipv4Addr::from_str(input).map_err(|_| 
                IpCalculatorError::MaskError(
                    format!("Invalid mask format: {}", input)
                )
            )?;
            let cidr = mask_to_cidr(mask)?;
            Ok(MaskOrCidr::Cidr(cidr))
        },
        ("CIDR", "mask") => {
            let cidr = input.parse::<u8>().map_err(|_| 
                IpCalculatorError::InvalidPrefix(
                    format!("Invalid CIDR prefix: {}", input)
                )
            )?;
            let mask = cidr_to_mask(cidr)?;
            Ok(MaskOrCidr::Mask(mask))
        },
        ("Mask", "mask") => {
            let mask = Ipv4Addr::from_str(input).map_err(|_|
                IpCalculatorError::MaskError(
                    format!("Invalid mask format: {}", input)
                )
            )?;
            Ok(MaskOrCidr::Mask(mask))
        },
        ("CIDR", "cidr") => {
            let cidr = input.parse::<u8>().map_err(|_|
                IpCalculatorError::InvalidPrefix(
                    format!("Invalid CIDR prefix: {}", input)
                )
            )?;
            Ok(MaskOrCidr::Cidr(cidr))
        },
        _ => Err(IpCalculatorError::ConversionError(
            format!("Cannot convert {} to {}", input_type, return_type)
        ))
    }
}

pub fn execute_command(command: Command) -> Result<(), IpCalculatorError> {
    match command {
        Command::Subnets { cidr, prefix, filter, page } => {
            let (subnets, total_subnets, total_pages) = generate_subnets(&cidr, prefix, filter, page)
                .map_err(|e| IpCalculatorError::SubnetError(
                    format!("Failed to generate subnets: {}", e)
                ))?;
            
            let displayed = subnets.len();
            let requested_page = page.unwrap_or(0);    
            
            for subnet in subnets {
                println!("{}", subnet);
                println!("----------------------------");
            }
            if requested_page >= total_pages {
                println!("\x1b[1;33mWarning: Only {} pages available. Showing last page.\x1b[0m", total_pages);
            }
            println!("\nTotal subnets: {} | Subnets displayed: {}", total_subnets, displayed);
            println!("Page {}/{}", total_pages, total_pages);
            
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
            match check_ip(&ip)? {
                true => {
                    println!("IP address {} is valid", ip);
                    Ok(())
                },
                false => Err(IpCalculatorError::InvalidIP(
                    format!("Invalid IP address format: {}", ip)
                ))
            }
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
