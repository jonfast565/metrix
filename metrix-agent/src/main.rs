#[macro_use] extern crate log;

use tokio::time::sleep;
use crate::models::AggregatedInfo;
use std::time::Duration;
use std::sync::mpsc::channel;
use ctrlc;

mod models;
mod device_metrics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    info!("{}", metrix_utils::get_header("Agent"));
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
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
        match rx.try_recv() {
            Ok(_) => {
                info!("Ctrl + C invoked. Application will quit now");
                break
            },
            _ => {
                // wait
                info!("Waiting 30 seconds");
                sleep(Duration::from_secs(30)).await;
            }
        }
    }
    Ok(())
}

async fn send_metric(hostname: String, data_point: String, data_type: String, value: f64) -> Result<(), Box<dyn std::error::Error>> {
    let model = metrix_models::MetricInsertPartial {
        data_group: Some(hostname),
        data_point: data_point,
        data_type: data_type,
        data_value_numeric: value 
    };
    Ok(metrix_http::post_metric(model).await?)
}

async fn send_metrics(aggregated_info: &AggregatedInfo) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Sending metrics to server...");
    Ok(())
}