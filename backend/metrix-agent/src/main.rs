#[macro_use]
extern crate log;

use crate::models::SystemInformation;
use crossbeam::channel::{unbounded, Receiver, Sender};
use ctrlc;
use std::time::Duration;
use tokio::try_join;

mod device_metrics;
mod models;
mod send_metrics;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    headers();
    let (tx, rx) = unbounded();
    set_ctrl_c_handler(tx);

    let host_information = device_metrics::get_host_information().await?;

    let cancel = rx.clone();
    let hi = host_information.clone();
    let mount_point_watcher_task = tokio::spawn(async move {
        mount_point_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let networks_watcher_task = tokio::spawn(async move {
        networks_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let battery_life_watcher_task = tokio::spawn(async move {
        battery_life_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let ac_power_watcher_task = tokio::spawn(async move {
        ac_power_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let uptime_watcher_task = tokio::spawn(async move {
        uptime_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let boot_time_watcher_task = tokio::spawn(async move {
        boot_time_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let cpu_watcher_task = tokio::spawn(async move {
        cpu_watcher(cancel, hi).await.expect("");
    });

    let cancel = rx.clone();
    let hi = host_information.clone();
    let memory_watcher_task = tokio::spawn(async move {
        memory_watcher(cancel, hi).await.expect("");
    });

    try_join!(
        mount_point_watcher_task,
        networks_watcher_task,
        battery_life_watcher_task,
        ac_power_watcher_task,
        uptime_watcher_task,
        boot_time_watcher_task,
        cpu_watcher_task,
        memory_watcher_task
    )?;
    Ok(())
}

async fn mount_point_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mount_points = device_metrics::get_mounts();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Mount Points".to_string(),
            "Count".to_string(),
            mount_points.len() as f64,
        )
        .await?;

        for mount_point in &mount_points {
            send_metrics::send_metric(
                sys_info.hostname.clone(),
                format!("Available Space {}", mount_point.fs_mounted_on).to_string(),
                "Bytes".to_string(),
                mount_point.avail.as_u64() as f64,
            )
            .await?;
            send_metrics::send_metric(
                sys_info.hostname.clone(),
                format!("Total Space {}", mount_point.fs_mounted_on).to_string(),
                "Bytes".to_string(),
                mount_point.total.as_u64() as f64,
            )
            .await?;
        }

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn networks_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let net_addresses = device_metrics::get_networks();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Network Interfaces".to_string(),
            "Count".to_string(),
            net_addresses.len() as f64,
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn battery_life_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let battery_life = device_metrics::get_battery_life();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Battery Life".to_string(),
            "Percentage".to_string(),
            battery_life.unwrap().remaining_life as f64,
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn ac_power_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let ac_power = device_metrics::get_on_ac_power();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "AC Power".to_string(),
            "Boolean".to_string(),
            match ac_power {
                true => 1,
                false => 0,
            } as f64,
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn uptime_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let uptime_seconds = device_metrics::get_uptime().unwrap();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Uptime".to_string(),
            "Seconds".to_string(),
            uptime_seconds.as_secs_f64(),
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn boot_time_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let boot_time = device_metrics::get_boot_time().unwrap();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Boot Time".to_string(),
            "Unix Time".to_string(),
            boot_time.timestamp() as f64,
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn cpu_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let cpu_load = device_metrics::get_cpu_load().unwrap();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "CPU Load (User)".to_string(),
            "Percentage".to_string(),
            cpu_load.user as f64,
        )
        .await?;

        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "CPU Load (System)".to_string(),
            "Percentage".to_string(),
            cpu_load.system as f64,
        )
        .await?;

        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "CPU Idle".to_string(),
            "Percentage".to_string(),
            cpu_load.idle as f64,
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

async fn memory_watcher(
    rx: Receiver<()>,
    sys_info: SystemInformation,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let memory_stats = device_metrics::get_memory_usage().unwrap();
        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Memory Used".to_string(),
            "Bytes".to_string(),
            memory_stats.used as f64,
        )
        .await?;

        send_metrics::send_metric(
            sys_info.hostname.clone(),
            "Memory Total".to_string(),
            "Bytes".to_string(),
            memory_stats.all as f64,
        )
        .await?;

        match wait_on_ctrl_c(rx.clone()) {
            true => (),
            false => continue,
        };
    }
}

fn headers() {
    pretty_env_logger::init();
    println!("{}", metrix_utils::get_header("Agent"));
}

fn set_ctrl_c_handler(tx: Sender<()>) {
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
}

fn wait_on_ctrl_c(rx: Receiver<()>) -> bool {
    info!("Done. Use Ctrl + C to quit.");
    info!("1 seconds until next run.");
    match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(_) => {
            info!("Ctrl + C invoked. Application will quit now");
            true
        }
        _ => false,
    }
}
