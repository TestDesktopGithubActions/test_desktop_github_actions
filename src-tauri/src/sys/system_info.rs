use serde::Deserialize;
use serde::Serialize;
use sysinfo::{CpuExt, DiskExt, SystemExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: Platform,
    // pub net: Vec<Net>,
    pub memory: Memory,
    pub disk: Disk,
    pub cpu: Cpu,
}

impl SystemInfo {
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn to_sha3(&self) -> String {
        let json = self.to_json();
        use sha3::Sha3_256;
        // create a SHA3-256 object
        let mut hasher = Sha3_256::default();

        sha3::digest::Digest::update(&mut hasher, json);
        hex::encode(sha3::Digest::finalize(hasher))
    }

    pub(crate) fn get_os_version(&self) -> &String {
        &self.platform.os_version
    }

    pub fn get_sys() -> Self {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();

        let mut opt_disk = Disk::default();

        if sys.disks().first().is_some() {
            let disk = sys.disks().first().unwrap();
            opt_disk.name = format!("{:?}", disk.name());
            opt_disk.disk_type = format!("{:?}", disk.kind());
            opt_disk.total = disk.total_space();
            opt_disk.count = sys.disks().len() as u32;
        }

        let ram = Memory {
            total: sys.total_memory(),
            swap: sys.total_swap(),
        };

        let platform = Platform {
            host_name: sys.host_name().unwrap_or("none".to_string()),
            os_name: sys.name().unwrap_or("none".to_string()),
            os_version: sys.os_version().unwrap_or("none".to_string()),
            kernel_version: sys.kernel_version().unwrap_or("none".to_string()),
        };

        SystemInfo {
            platform,
            memory: ram,
            cpu: Cpu {
                frequency: sys.global_cpu_info().frequency(),
                vendor_id: sys.global_cpu_info().vendor_id().to_string(),
                brand: sys.global_cpu_info().brand().to_string(),
                count: sys.cpus().len() as u64,
            },
            disk: opt_disk,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Platform {
    pub host_name: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Net {
    pub ip: String,
    pub name: String,
    pub ip_v6: String,
    pub mac: String,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub packets_sent: u64,
    pub packets_recv: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Memory {
    pub total: u64,
    pub swap: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Disk {
    pub total: u64,
    pub disk_type: String,
    pub name: String,
    pub count: u32,
}

impl Disk {
    pub fn new(total: u64, count: u32, disk_type: String, name: String) -> Self {
        Self {
            total,
            disk_type,
            name,
            count,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Cpu {
    pub count: u64,
    pub frequency: u64,
    pub vendor_id: String,
    pub brand: String,
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sys_info() {
        let sys = super::SystemInfo::get_sys();
        println!("sys_info: {sys:#?}");
    }
}
