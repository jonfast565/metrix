#[macro_use] extern crate log;

use crate::models::AggregatedInfo;
use std::time::Duration;
use std::thread;

mod models;
mod device_metrics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    info!("{}", metrix_utils::get_header("Agent"));
    loop {
        let host_information = device_metrics::get_host_information().await?;
        let mount_points = device_metrics::get_mounts();
        let block_device_stats = device_metrics::get_block_device_stats();
        let netifs = device_metrics::get_networks();
        // device_metrics::get_network_interface_stats();
        let battery_life = device_metrics::get_battery_life();
        let on_ac_power = device_metrics::get_on_ac_power();
        // device_metrics::get_load_average();
        // let uptime = device_metrics::get_uptime();
        // let boot_time = device_metrics::get_boot_time();
        let cpu_load = device_metrics::get_cpu_load();
        // let cpu_temp = device_metrics::get_cpu_temp();
        let memory_stats = device_metrics::get_memory_usage();
        // device_metrics::get_socket_stats();
        let aggregated_info = AggregatedInfo {
            sys_info: host_information,
            mount_points: mount_points,
            block_stats: block_device_stats,
            net_addresses: netifs,
            battery_life: battery_life.unwrap(),
            cpu_load: cpu_load.unwrap(),
            memory_stats: memory_stats.unwrap(),
            ac_power: on_ac_power,
            uptime_seconds: -1.0,
        };
        send_metrics(&aggregated_info).await?;
        // wait so we don't send crazy amounts of data...
        info!("Waiting 30 seconds");
        thread::sleep(Duration::from_secs(30));
    }
    Ok(())
}

async fn send_metrics(aggregated_info: &AggregatedInfo) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}