use crate::MountPoint;
use std::thread;
use std::time::Duration;
use systemstat::{System, Platform, saturating_sub_bytes};

use heim::{cpu, Result};
#[cfg(target_os = "linux")]
use heim::cpu::os::linux::CpuStatsExt;
#[cfg(target_os = "windows")]
use heim::cpu::os::windows::CpuStatsExt;

pub fn get_mounts() -> Vec<MountPoint> {
    let sys = System::new();
    let mut mount_points = Vec::<MountPoint>::new();
    match sys.mounts() {
        Ok(mounts) => {
            info!("\nMounts:");
            for mount in mounts.iter() {
                mount_points.push(MountPoint {
                    fs_mounted_from: mount.fs_mounted_from.clone(),
                    fs_type: mount.fs_type.clone(),
                    fs_mounted_on: mount.fs_mounted_on.clone(),
                    avail: mount.avail,
                    total: mount.total
                });
            }
        }
        Err(x) => error!("\nMounts: error: {}", x)
    }
    mount_points
}

pub fn get_block_device_stats() {
    let sys = System::new();
    match sys.block_device_statistics() {
        Ok(stats) => {
            for blkstats in stats.values() {
                info!("{}: {:?}", blkstats.name, blkstats);
            }
        }
        Err(x) => error!("\nBlock statistics error: {}", x.to_string())
    }
}

pub fn get_networks() {
    let sys = System::new();
    match sys.networks() {
        Ok(netifs) => {
            info!("\nNetworks:");
            for netif in netifs.values() {
                info!("{} ({:?})", netif.name, netif.addrs);
            }
        }
        Err(x) => error!("\nNetworks: error: {}", x)
    }
}

/*
pub fn get_network_interface_stats() {
    let sys = System::new();
    match sys.networks() {
        Ok(netifs) => {
            info!("\nNetwork interface statistics:");
            for netif in netifs.values() {
                info!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
            }
        }
        Err(x) => error!("\nNetworks: error: {}", x)
    }
}
*/

pub fn get_battery_life() {
    let sys = System::new();
    match sys.battery_life() {
        Ok(battery) =>
            info!("\nBattery: {}%, {}h{}m remaining",
                   battery.remaining_capacity*100.0,
                   battery.remaining_time.as_secs() / 3600,
                   battery.remaining_time.as_secs() % 60),
        Err(x) => error!("\nBattery: error: {}", x)
    }
}

pub fn get_on_ac_power() {
    let sys = System::new();
    match sys.on_ac_power() {
        Ok(power) => info!(", AC power: {}", power),
        Err(x) => error!(", AC power: error: {}", x)
    }
}

pub fn get_memory_usage() {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => info!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory),
        Err(x) => error!("\nMemory: error: {}", x)
    }
}

pub fn get_load_average() {
    let sys = System::new();
    if cfg!(linux) || cfg!(macos) {
        match sys.load_average() {
            Ok(loadavg) => info!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => error!("\nLoad average: error: {}", x)
        }
    }
}

pub fn get_uptime() {
    let sys = System::new();
    match sys.uptime() {
        Ok(uptime) => info!("\nUptime: {:?}", uptime),
        Err(x) => error!("\nUptime: error: {}", x)
    }
}

pub fn get_boot_time() {
    let sys = System::new();
    match sys.boot_time() {
        Ok(boot_time) => info!("\nBoot time: {}", boot_time),
        Err(x) => error!("\nBoot time: error: {}", x)
    }
}

pub fn get_cpu_load() {
    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu)=> {
            info!("\nMeasuring CPU load...");
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            info!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
        },
        Err(x) => error!("\nCPU load: error: {}", x)
    }
}

pub fn get_cpu_temp() {
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => info!("\nCPU temp: {}", cpu_temp),
        Err(x) => error!("\nCPU temp: {}", x)
    }
}

pub fn get_socket_stats() {
    let sys = System::new();
    match sys.socket_stats() {
        Ok(stats) => info!("\nSystem socket statistics: {:?}", stats),
        Err(x) => error!("\nSystem socket statistics: error: {}", x.to_string())
    }
}