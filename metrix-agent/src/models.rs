
use systemstat::ByteSize;

pub struct MountPoint {
    pub fs_mounted_from: String, 
    pub fs_type: String, 
    pub fs_mounted_on: String, 
    pub avail: ByteSize, 
    pub total: ByteSize
}