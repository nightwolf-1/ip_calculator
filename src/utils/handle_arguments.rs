use crate::libs::calc_ip::{
    execute_command, is_cidr_or_mask, mask_to_cidr, parse_mask_or_cidr, InputType, Command, IpCalculatorError, CommandHelp
};
use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn handle_arguments(args: Vec<String>) -> Result<String, IpCalculatorError> {
    if args.len() < 2 {
        return Err(IpCalculatorError::ArgumentsError(
            "Usage: ./ip_calculator <options> <arguments>".to_string()
        ));
    }

    let command = match args[1].as_str() {
        "-h" | "--help" => {
            if args.len() == 2 {
                CommandHelp::display_command_list();
                return Ok("Help information displayed.".to_string());
            } else {
                match CommandHelp::find_by_name_or_alias(&args[2]) {
                    Some(cmd_help) => {
                        cmd_help.display_command_help();
                        return Ok("Command help displayed.".to_string());
                    },
                    None => {
                        return Err(IpCalculatorError::ArgumentsError(
                            format!("Unknown command '{}'. Use --help for a list of available commands.", args[2])
                        ));
                    }
                }
            }
        },
        "-s" | "--subnets" => {
            if args.len() < 4 {
                return Err(IpCalculatorError::ArgumentsError(
                    "Missing arguments for subnet calculation! Usage: ./ip_calculator -s or --subnets <CIDR> <prefix> [-f <number_of_subnet_to_print>] [-p <page_number>]".to_string()
                ));
            }

            let cidr = args[2].clone();
            let prefix = args[3].parse::<u8>().map_err(|_| 
                IpCalculatorError::InvalidPrefix("Invalid prefix value! Prefix should be a valid number between 0 and 32.".to_string())
            )?;

            let mut filter = None;
            let mut page = None;
            let mut output_file = None;
            let mut i = 4;
            while i < args.len() {
                let arg = args[i].as_str();
                if arg.starts_with("-f") {
                    let val = if arg == "-f" {
                        i += 1;
                        if i >= args.len() {
                            return Err(IpCalculatorError::ArgumentsError("Missing value for -f option".to_string()));
                        }
                        args[i].as_str()
                    } else {
                        &arg[2..]
                    };
                    filter = Some(val.parse::<usize>().map_err(|_|
                        IpCalculatorError::ArgumentsError("Invalid filter value".to_string())
                    )?);
                } else if arg.starts_with("-p") {
                    let val = if arg == "-p" {
                        i += 1;
                        if i >= args.len() {
                            return Err(IpCalculatorError::ArgumentsError("Missing value for -p option".to_string()));
                        }
                        args[i].as_str()
                    } else {
                        &arg[2..]
                    };
                    let p = val.parse::<usize>().map_err(|_|
                        IpCalculatorError::ArgumentsError("Invalid page number".to_string())
                    )?;
                    page = Some(p.saturating_sub(1));
                } else if arg == "-o" || arg == "--output" {
                    i += 1;
                    if i >= args.len() {
                        return Err(IpCalculatorError::ArgumentsError("Missing value for -o option".to_string()));
                    }
                    output_file = Some(args[i].clone());
                } else if let Some(val) = arg.strip_prefix("-o") {
                    output_file = Some(val.to_string());
                } else if let Some(val) = arg.strip_prefix("--output=") {
                    output_file = Some(val.to_string());
                }
                i += 1;
            }

            Command::Subnets {
                cidr,
                prefix,
                filter,
                page,
                output_file,
            }
        },
        "--get-subnet" => {
            if args.len() < 5 {
                return Err(IpCalculatorError::ArgumentsError(
                    "Missing arguments for specific subnet retrieval! Expected: <CIDR> <prefix> <index>".to_string()
                ));
            }

            let cidr = args[2].clone();
            let prefix = args[3].parse::<u8>().map_err(|_| 
                IpCalculatorError::InvalidPrefix("Invalid prefix value!".to_string())
            )?;
            let index = args[4].parse::<u32>().map_err(|_| 
                IpCalculatorError::SubnetError("Invalid subnet index!".to_string())
            )?;

            Command::GetSubnet {
                cidr,
                prefix,
                index,
            }
        },
        "-same" | "--same-subnet" => {
            let min_args = 5;
            let max_args = 6;
            if args.len() < min_args || args.len() > max_args {
                return Err(IpCalculatorError::ArgumentsError(
                    "Invalid number of arguments for same subnet check! Expected: <IP1> <IP2> <mask1> [mask2]".to_string()
                ));
            }

            let ip1 = Ipv4Addr::from_str(&args[2]).map_err(|_| 
                IpCalculatorError::InvalidIP("Invalid IP address format for first IP".to_string())
            )?;
            let ip2 = Ipv4Addr::from_str(&args[3]).map_err(|_| 
                IpCalculatorError::InvalidIP("Invalid IP address format for second IP".to_string())
            )?;
            let mask1 = parse_mask_or_cidr(&args[4], InputType::Mask)?.expect_mask();
            let mask2 = if args.len() == max_args {
                Some(parse_mask_or_cidr(&args[5], InputType::Mask)?.expect_mask())
            } else {
                None
            };

            Command::SameSubnet { ip1, ip2, mask1, mask2 }
        },
        "-cip" | "--check-ip" => {
            if args.len() < 3 {
                return Err(IpCalculatorError::ArgumentsError(
                    "Missing IP for validation! Expected: <IP>".to_string()
                ));
            }
            Command::CheckIP {
                ip: args[2].clone(),
            }
        },
        "-cmask" | "--check-mask" => {
            if args.len() < 3 {
                return Err(IpCalculatorError::ArgumentsError(
                    "Missing mask for validation! Expected: <mask>".to_string()
                ));
            }
            Command::CheckMask {
                mask: args[2].clone(),
            }
        },
        "-fr" | "--find-range" => {
            if args.len() < 4 {
                return Err(IpCalculatorError::ArgumentsError(
                    "Missing arguments for --find-range! Usage: ./ip_calculator -fr or --find-range <CIDR> <number_of_hosts> [exclusion..]".to_string()
                ));
            }

            let calculate_subnet = crate::libs::calc_ip::calculate_subnet;
            let (cidr, range_size, exclusions) = if args[2].contains('/') {
                calculate_subnet(&args[2]).map_err(|_| IpCalculatorError::InvalidCIDR(
                    format!("Invalid CIDR format for {}", args[2])
                ))?;

                let range_size = args[3].parse::<usize>().map_err(|_| 
                    IpCalculatorError::InvalidRange("Invalid number of hosts specified!".to_string())
                )?;

                let exclusions: Vec<Ipv4Addr> = args[4..]
                    .iter()
                    .filter_map(|ip| Ipv4Addr::from_str(ip).ok())
                    .collect();

                (args[2].clone(), range_size, exclusions)
            } else {
                if args.len() < 5 {
                    return Err(IpCalculatorError::ArgumentsError(
                        "Missing arguments for --find-range with IP and mask!".to_string()
                    ));
                }

                let prefix = parse_mask_or_cidr(&args[3], InputType::Cidr)?.expect_cidr();
                let cidr = format!("{}/{}", args[2], prefix);

                let range_size = args[4].parse::<usize>().map_err(|_| 
                    IpCalculatorError::InvalidRange("Invalid number of hosts specified!".to_string())
                )?;

                let exclusions: Vec<Ipv4Addr> = args[5..]
                    .iter()
                    .filter_map(|ip| Ipv4Addr::from_str(ip).ok())
                    .collect();

                (cidr, range_size, exclusions)
            };

            Command::FindRange {
                cidr,
                range_size,
                exclusions,
            }
        },
        _ => {
            if args[1].contains('/') {
                Command::Display {
                    cidr: args[1].clone(),
                }
            } else {
                if args.len() == 3 {
                    let cidr = match is_cidr_or_mask(&args[2])? {
                        InputType::Mask => {
                            let mask = Ipv4Addr::from_str(&args[2]).map_err(|_| 
                                IpCalculatorError::InvalidMask("Invalid mask format!".to_string())
                            )?;
                            let cidr_prefix = mask_to_cidr(mask)?;
                            format!("{}/{}", args[1], cidr_prefix)
                        }
                        InputType::Cidr => format!("{}/{}", args[1], args[2]),
                    };
                    Command::Display { cidr }
                } else {
                    return Err(IpCalculatorError::ArgumentsError(
                        "Unknown command! Check the usage instructions for proper syntax.".to_string()
                    ));
                }
            }
        }
    };

    execute_command(command)?;
    Ok("Operation completed successfully.".to_string())
}