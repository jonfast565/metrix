
use systemstat::ByteSize;

pub struct SystemInformation {
    pub system: String, 
    pub release: String, 
    pub hostname: String, 
    pub version: String, 
    pub architecture: String,
}

pub struct MountPoint {
    pub fs_mounted_from: String, 
    pub fs_type: String, 
    pub fs_mounted_on: String, 
    pub avail: ByteSize, 
    pub total: ByteSize
}

pub struct BlockStats {
    pub name: String
}

pub struct NetworkAddresses {
    pub network_name: String,
    pub ip_addresses: Vec<String>,
}

pub struct BatteryLife {
    pub remaining_life: f32,
    pub remaining_hours: u64,
    pub remaining_minutes: u64,
}

pub struct CpuLoad {
    pub user: f32,
    pub nice: f32,
    pub system: f32,
    pub interrupt: f32,
    pub idle: f32,
}

pub struct MemoryStats {
    pub used: u64,
    pub all: u64
}

pub struct AggregatedInfo {
    pub sys_info: SystemInformation,
    pub mount_points: Vec<MountPoint>,
    pub block_stats: Vec<BlockStats>,
    pub net_addresses: Vec<NetworkAddresses>,
    pub battery_life: BatteryLife,
    pub cpu_load: CpuLoad,
    pub memory_stats: MemoryStats,
    pub ac_power: bool,
    pub uptime_seconds: f32,
    pub boot_time: i64
}