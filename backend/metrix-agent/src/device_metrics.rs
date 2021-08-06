use crate::models::CpuLoad;
use crate::models::MemoryStats;
use crate::models::{BatteryLife, BlockStats, MountPoint, NetworkAddresses, SystemInformation};
use chrono::{DateTime, Utc};
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

// #[cfg(target_os = "linux")]
// use heim::cpu::os::linux::CpuStatsExt;
// #[cfg(target_os = "windows")]
// use heim::cpu::os::windows::CpuStatsExt;
// use heim::{cpu, Result};

pub fn get_mounts() -> Vec<MountPoint> {
    let sys = System::new();
    let mut results = Vec::<MountPoint>::new();
    match sys.mounts() {
        Ok(mounts) => {
            info!("Mounts: ");
            for mount in mounts.iter() {
                info!(
                    "\t{} - {} -> {} (available {} / {})",
                    mount.fs_mounted_from,
                    mount.fs_type,
                    mount.fs_mounted_on,
                    mount.avail,
                    mount.total
                );
                results.push(MountPoint {
                    fs_mounted_from: mount.fs_mounted_from.clone(),
                    fs_type: mount.fs_type.clone(),
                    fs_mounted_on: mount.fs_mounted_on.clone(),
                    avail: mount.avail,
                    total: mount.total,
                });
            }
        }
        Err(x) => error!("{}", x),
    }
    results
}

pub fn get_block_device_stats() -> Vec<BlockStats> {
    let sys = System::new();
    let mut results = Vec::<BlockStats>::new();
    match sys.block_device_statistics() {
        Ok(stats) => {
            info!("Block Devices: ");
            for blkstats in stats.values() {
                info!("\t{}: {:?}", blkstats.name, blkstats);
                results.push(BlockStats {
                    name: blkstats.name.clone(),
                    // TODO: Add more fields on Linux
                })
            }
        }
        Err(x) => error!("Block Device: {}", x),
    };
    results
}

pub fn get_networks() -> Vec<NetworkAddresses> {
    let sys = System::new();
    let mut results = Vec::<NetworkAddresses>::new();
    match sys.networks() {
        Ok(netifs) => {
            info!("Networks: ");
            for netif in netifs.values() {
                let addresses: Vec<String> = netif
                    .addrs
                    .clone()
                    .into_iter()
                    .map(|x| match x.addr {
                        systemstat::IpAddr::V4(x) => x.to_string(),
                        systemstat::IpAddr::V6(x) => x.to_string(),
                        _ => "".to_string(),
                    })
                    .filter(|x| x != "")
                    .collect();
                info!("\t{} ({:?})", netif.name, &addresses);
                results.push(NetworkAddresses {
                    network_name: netif.name.clone(),
                    ip_addresses: addresses,
                })
            }
        }
        Err(x) => error!("Networks: {}", x),
    }
    results
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

pub fn get_battery_life() -> Option<BatteryLife> {
    let sys = System::new();
    match sys.battery_life() {
        Ok(battery) => {
            info!(
                "Battery: {}%, {} hours and {} minutes remaining",
                battery.remaining_capacity * 100.0,
                battery.remaining_time.as_secs() / 3600,
                battery.remaining_time.as_secs() % 60
            );
            Some(BatteryLife {
                remaining_life: battery.remaining_capacity * 100.0,
                remaining_hours: battery.remaining_time.as_secs() / 3600,
                remaining_minutes: battery.remaining_time.as_secs() % 60,
            })
        }
        Err(x) => {
            error!("Battery: {}", x);
            None
        }
    }
}

pub fn get_on_ac_power() -> bool {
    let sys = System::new();
    match sys.on_ac_power() {
        Ok(power) => {
            info!("AC power: {}", power);
            power
        }
        Err(x) => {
            error!("AC power: {}", x);
            false
        }
    }
}

pub fn get_memory_usage() -> Option<MemoryStats> {
    let sys = System::new();
    match sys.memory() {
        Ok(mem) => {
            info!(
                "Memory: {} used / {} ({} bytes)",
                saturating_sub_bytes(mem.total, mem.free),
                mem.total,
                mem.total.as_u64()
            );
            Some(MemoryStats {
                used: saturating_sub_bytes(mem.total, mem.free).as_u64(),
                all: mem.total.as_u64(),
            })
        }
        Err(x) => {
            error!("Memory: {}", x);
            None
        }
    }
}

/*
pub fn get_load_average() {
    let sys = System::new();
    if cfg!(linux) || cfg!(macos) {
        match sys.load_average() {
            Ok(loadavg) => info!(
                "\nLoad average: {} {} {}",
                loadavg.one, loadavg.five, loadavg.fifteen
            ),
            Err(x) => error!("\nLoad average: error: {}", x),
        }
    }
}
*/

pub fn get_uptime() -> Option<Duration> {
    let sys = System::new();
    match sys.uptime() {
        Ok(uptime) => {
            info!("Uptime: {:?}", uptime);
            Some(uptime)
        }
        Err(x) => {
            error!("Uptime: {}", x);
            None
        }
    }
}

pub fn get_boot_time() -> Option<DateTime<Utc>> {
    let sys = System::new();
    match sys.boot_time() {
        Ok(boot_time) => {
            info!("Boot Time: {}", boot_time);
            Some(boot_time)
        }
        Err(x) => {
            error!("Boot Time: {}", x);
            None
        }
    }
}

pub fn get_cpu_load() -> Option<CpuLoad> {
    let sys = System::new();
    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            info!(
                "CPU Load: {}% user, {}% nice, {}% system, {}% intr, {}% idle",
                cpu.user * 100.0,
                cpu.nice * 100.0,
                cpu.system * 100.0,
                cpu.interrupt * 100.0,
                cpu.idle * 100.0
            );
            Some(CpuLoad {
                user: cpu.user * 100.0,
                nice: cpu.nice * 100.0,
                system: cpu.system * 100.0,
                interrupt: cpu.interrupt * 100.0,
                idle: cpu.idle * 100.0,
            })
        }
        Err(x) => {
            error!("CPU Load: {}", x);
            None
        }
    }
}

pub fn get_cpu_temp() -> Option<f32> {
    let sys = System::new();
    match sys.cpu_temp() {
        Ok(cpu_temp) => {
            info!("CPU Temperature: {}", cpu_temp);
            Some(cpu_temp)
        }
        Err(x) => {
            error!("CPU Temperature: {}", x);
            None
        }
    }
}

/*
pub fn get_socket_stats() {
    let sys = System::new();
    match sys.socket_stats() {
        Ok(stats) => info!("\nSystem socket statistics: {:?}", stats),
        Err(x) => error!("{}", x),
    }
}
*/

pub async fn get_host_information() -> heim::Result<SystemInformation> {
    let platform = heim::host::platform().await?;
    info!("Host Information: ");
    info!("\tSystem: {}", platform.system());
    info!("\tRelease: {}", platform.release());
    info!("\tHostname: {}", platform.hostname());
    info!("\tVersion: {}", platform.version());
    info!("\tArch: {}", platform.architecture().as_str());

    let sys_info = SystemInformation {
        system: platform.system().to_string(),
        release: platform.release().to_string(),
        hostname: platform.hostname().to_string(),
        version: platform.version().to_string(),
        architecture: platform.architecture().as_str().to_string(),
    };

    Ok(sys_info)
}
