//---------- This is where the calculations live ----------
fn calculate_network(ip: u32, mask: u32) -> u32 {
    ip & mask
}

fn calculate_broadcast(ip: u32, mask: u32) -> u32 {
    (ip & mask) + !mask
}

fn ip_to_index(ip: u32, mask: u32) -> u32 {
    ip - calculate_network(ip, mask)
}

fn index_to_ip(index: i64, ip: u32, mask: u32) -> u32 {
    if index < 0 {
        (calculate_broadcast(ip, mask) as i64 + index) as u32
    } else {
        calculate_network(ip, mask) + index as u32
    }
}

fn number_of_hosts(ip: u32, mask: u32) -> u32 {
    calculate_broadcast(ip, mask) - calculate_network(ip, mask) - 1
}

//---------- This is where user interface starts ----------
fn valid_mask(mask: u32) -> bool {
    if mask.leading_ones() < mask.count_ones() {
        return false;
    }
    true
}

fn dotted_to_u32(address: &str) -> Result<u32, u32> {
    let mut octet_number: u32 = 0;
    let mut result: u32 = 0;
    for i in address
        .split('.')
        .rev()
        .map(|x| x.to_owned().parse::<u32>())
    {
        match i {
            Ok(x) => {
                if x > 255 {
                    return Err(0);
                };
                result += x << 8 * octet_number;
                octet_number += 1
            }
            Err(_) => return Err(0),
        }
    }
    Ok(result)
}

fn u32_to_dotted(address: u32) -> String {
    let mut result: [u32; 4] = [0; 4];
    for i in 0..4 {
        result[i] = 255 & address >> (3 - i) * 8
    }
    result
        .map(|x| x.to_string() + ".")
        .concat()
        .trim_end_matches('.')
        .to_string()
}

fn prefix_to_u32(prefix: u32) -> u32 {
    (2u32.pow(prefix) - 1).reverse_bits()
}

fn color_bits() {
    println!("\u{001b}[1;31m piros szöveg")
}

fn ip_parser(input: String) -> (u32, String) {
    // pattern: abc.def.ghi.jkl
    (1, "1".to_string())
}

fn user_input_ip() -> (u32, u32) {
    println!("Ip address: ");
    let mut ip = String::new();
    std::io::stdin().read_line(&mut ip).expect("Not good");
    let mask = ip
        .split("/")
        .last()
        .unwrap_or("69")
        .trim()
        .parse::<u32>()
        .unwrap_or(0);
    let ip = match dotted_to_u32(ip.split("/").next().unwrap_or("69").trim()) {
        Ok(x) => x,
        Err(e) => e,
    };
    if ip == 0 {
        println!("Invalid address.");
        user_input_ip()
    } else {
        (ip, mask)
    }
}

fn user_input_mask() -> u32 {
    println!("Subnet mask: ");
    let mut mask = String::new();
    std::io::stdin().read_line(&mut mask).expect("Not good");
    let mask = match dotted_to_u32(mask.trim()) {
        Ok(x) => x,
        Err(e) => e,
    };

    if mask == 0 || !valid_mask(mask) {
        println!("Invalid subnet mask.");
        user_input_mask()
    } else {
        mask
    }
}

fn user_input_find_host(ip: u32, mask: u32) -> u32 {
    let mut host = String::new();
    std::io::stdin().read_line(&mut host).expect("Not good");
    index_to_ip(host.trim().parse::<i64>().unwrap(), ip, mask)
}

fn ip_output(ip: u32, mask: u32) {
    println!(
        "\nip: {} /{} \nmask: {}\nindex in network: {}\nnetwork: {}\nbroadcast: {}\nnumber of hosts: {}",
        u32_to_dotted(ip),
        mask.count_ones(),
        u32_to_dotted(mask),
        ip_to_index(ip, mask),
        u32_to_dotted(calculate_network(ip, mask)),
        u32_to_dotted(calculate_broadcast(ip, mask)),
        number_of_hosts(ip, mask)
    );
}

//---------- This is where the tests start ----------
fn main() {
    let (ip, mask) = user_input_ip();
    if mask == 0 {
        let mask = user_input_mask();
    }
    ip_output(ip, mask);

    loop {
        println!("find host: ");
        ip_output(user_input_find_host(ip, mask), mask)
    }
}
