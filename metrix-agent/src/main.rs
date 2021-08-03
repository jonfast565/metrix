#[macro_use] extern crate log;

use std::time::Duration;
use crate::models::MountPoint;
use std::thread;

mod models;
mod device_metrics;

fn main() {
    pretty_env_logger::init();
    loop {
        let _mount_points = device_metrics::get_mounts();
        let _block_device_stats = device_metrics::get_block_device_stats();
        device_metrics::get_networks();
        // device_metrics::get_network_interface_stats();
        device_metrics::get_battery_life();
        device_metrics::get_on_ac_power();
        device_metrics::get_memory_usage();
        device_metrics::get_load_average();
        device_metrics::get_uptime();
        device_metrics::get_boot_time();
        device_metrics::get_cpu_load();
        device_metrics::get_cpu_temp();
        device_metrics::get_socket_stats();
        thread::sleep(Duration::from_secs(30));
    }
}