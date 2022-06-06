use sysinfo::{System, SystemExt};

fn main() {
    get_basic_info_using_cupid();
    get_sysinfo();
}

fn get_sysinfo() {
    if System::IS_SUPPORTED {
        let mut system = System::new_all();
        system.refresh_all();
        let ram = system.free_memory();
        let cores = system.cpus();
        let num_cores = cores.len();
        println!("System has {ram}KB total memory");
        println!("System has {num_cores} cores");

        if let Some(cpu) = cores.first() {
            println!("System core 1: {:?} ", cpu);
        }
    } else {
        println!("System is not supported, skipping memory checks")
    }
}

fn get_basic_info_using_cupid() {
    let information = cupid::master();
    println!("CPU information available: {:#?}", information);
    if let Some(information) = information {
        if information.sse4_2() {
            println!("SSE 4.2 Available");
        }
        if information.intel_64_bit_architecture() {
            println!("Intel 64 bit architecture");
        }

        if information.aesni() {
            println!("AES new instructions available");
        }
    }
}
