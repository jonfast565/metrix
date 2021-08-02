use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

fn main() {
    loop {
        get_mounts();
        get_block_device_stats();
        get_networks();
        get_network_interface_stats();
        get_battery_life();
        get_on_ac_power();
        get_memory_usage();
        get_load_average();
        get_uptime();
        get_boot_time();
        get_cpu_load();
        get_cpu_temp();
        get_socket_stats();
        thread::sleep(Duration::from_secs(30));
    }
}

fn get_mounts() {
    let sys = System::new();
    match sys.mounts() {
        Ok(mounts) => {
            println!("\nMounts:");
            for mount in mounts.iter() {
                println!("{} ---{}---> {} (available {} of {})",
                         mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
            }
        }
        Err(x) => println!("\nMounts: error: {}", x)
    }
}

fn get_block_device_stats() {
    let sys = System::new();
    match sys.block_device_statistics() {
        Ok(stats) => {
            for blkstats in stats.values() {
                println!("{}: {:?}", blkstats.name, blkstats);
            }
        }
        Err(x) => println!("\nBlock statistics error: {}", x.to_string())
    }
}

fn get_networks() {
    let sys = System::new();
    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetworks:");
            for netif in netifs.values() {
                println!("{} ({:?})", netif.name, netif.addrs);
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }
}

fn get_network_interface_stats() {
    let sys = System::new();
    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetwork interface statistics:");
            for netif in netifs.values() {
                println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x)
    }
}

fn get_battery_life() {
    let sys = System::new();
    match sys.battery_life() {
        Ok(battery) =>
            print!("\nBattery: {}%, {}h{}m remaining",
                   battery.remaining_capacity*100.0,
                   battery.remaining_time.as_secs() / 3600,
                   battery.remaining_time.as_secs() % 60),
        Err(x) => print!("\nBattery: error: {}", x)
    }
}

fn get_on_ac_power() {
    let sys = System::new();
    match sys.on_ac_power() {
        Ok(power) => println!(", AC power: {}", power),
        Err(x) => println!(", AC power: error: {}", x)
    }
}

fn get_memory_usage() {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => println!("\nMemory: error: {}", x)
    }
}

fn get_load_average() {
    let sys = System::new();
    if cfg!(linux) || cfg!(macos) {
        match sys.load_average() {
            Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => println!("\nLoad average: error: {}", x)
        }
    }
}

fn get_uptime() {
    let sys = System::new();
    match sys.uptime() {
        Ok(uptime) => println!("\nUptime: {:?}", uptime),
        Err(x) => println!("\nUptime: error: {}", x)
    }
}

fn get_boot_time() {
    let sys = System::new();
    match sys.boot_time() {
        Ok(boot_time) => println!("\nBoot time: {}", boot_time),
        Err(x) => println!("\nBoot time: error: {}", x)
    }
}

fn get_cpu_load() {
    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            println!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => println!("\nCPU load: error: {}", x)
    }
}

fn get_cpu_temp() {
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x)
    }
}

fn get_socket_stats() {
    let sys = System::new();
    match sys.socket_stats() {
        Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
        Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
    }
}