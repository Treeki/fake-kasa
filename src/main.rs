use std::collections::HashMap;
use std::env;
use std::net::TcpStream;
mod lightbulbs;

fn try_toggle_light(addr: &str, state: bool) -> lightbulbs::Result<()> {
    let full_addr = format!("{}:9999", addr);
    let mut tcp = TcpStream::connect(full_addr)?;
    lightbulbs::set_on_off(&mut tcp, state, 100)?;
    Ok(())
}

fn main() {
    let mut lights = HashMap::new();
    lights.insert("livingroom".to_string(), "192.168.0.21".to_string());
    lights.insert("hallway1".to_string(), "192.168.0.22".to_string());
    lights.insert("hallway2".to_string(), "192.168.0.23".to_string());
    lights.insert("bedroom".to_string(), "192.168.0.101".to_string());

    if env::args().len() > 2 {
        let mode_str = env::args().nth(1).unwrap();
        let mut mode = false;
        if mode_str == "on" {
            mode = true;
        } else if mode_str != "off" {
            println!("Unknown mode: expecting 'on' or 'off'");
            return;
        }

        for name in env::args().skip(2) {
            match lights.get(&name) {
                Some(addr) => match try_toggle_light(addr, mode) {
                    Ok(()) => println!("Turned {} {}", name, mode_str),
                    Err(_) => println!("Error contacting {}", name)
                }
                None => println!("Unknown light: {}", name)
            }
        }
    } else {
        println!("Usage: {} on|off name ...", env::args().nth(0).unwrap());
        println!();
        println!("Known lights:");
        for name in lights.keys() {
            println!("  {}", name);
        }
    }
}
