use serde::{Deserialize, Serialize};
use sysinfo::{Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

#[cfg(target_os = "linux")]
use std::process::Command;



// ——— Data structs (mirror the original JS API shape) ———

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuCache {
    pub l2: u64,
    pub l3: u64,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub brand: String,
    pub vendor: String,
    pub family: String,
    pub model: String,
    pub stepping: String,
    pub physical_cores: usize,
    pub cores: usize,
    pub speed: f64,
    pub speed_max: f64,
    pub cache: CpuCache,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuCurrentSpeed {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CurrentLoad {
    pub current_load: f64,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CpuTemperature {
    pub main: f64,
    pub max: f64,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemorySlot {
    pub slot: usize,
    pub size: u64,
    pub clock_speed: u64,
    #[serde(rename = "type")]
    pub mem_type: String,
    pub form_factor: String,
    pub manufacturer: String,
    pub part_num: String,
    pub serial_num: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub active: u64,
    pub swaptotal: u64,
    pub swapused: u64,
    pub swapfree: u64,
    pub layout: Vec<MemorySlot>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GpuController {
    pub model: String,
    pub vendor: String,
    pub vram: u64,
    pub bus: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GraphicsInfo {
    pub controllers: Vec<GpuController>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiskLayoutEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub disk_type: String,
    pub size: u64,
    pub interface_type: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilesystemEntry {
    pub mount: String,
    #[serde(rename = "type")]
    pub fs_type: String,
    pub used: u64,
    pub size: u64,
    #[serde(rename = "use")]
    pub usage_pct: f64,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StorageInfo {
    pub disk_layout: Vec<DiskLayoutEntry>,
    pub filesystems: Vec<FilesystemEntry>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    pub iface: String,
    pub ip4: String,
    pub ip6: String,
    pub mac: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioDevice {
    pub name: String,
    pub manufacturer: String,
    pub status: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AudioInfo {
    pub devices: Vec<AudioDevice>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UsbDevice {
    pub name: String,
    pub vendor: String,
    pub vendor_id: String,
    pub product_id: String,
    pub bus: String,
    pub device: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PeripheralInfo {
    pub usb_devices: Vec<UsbDevice>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpticalDevice {
    pub name: String,
    pub model: String,
    pub vendor: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpticalInfo {
    pub devices: Vec<OpticalDevice>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BaseboardInfo {
    pub manufacturer: String,
    pub model: String,
    pub version: String,
    pub serial: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct BiosInfo {
    pub vendor: String,
    pub version: String,
    pub release_date: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OsInfo {
    pub platform: String,
    pub distro: String,
    pub release: String,
    pub hostname: String,
    pub kernel: String,
    pub arch: String,
    pub fqdn: String,
    pub uefi: bool,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct UuidInfo {
    pub macs: Vec<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VersionsInfo {
    pub node: String,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StaticData {
    pub baseboard: BaseboardInfo,
    pub bios: BiosInfo,
    pub os: OsInfo,
    pub uuid: UuidInfo,
    pub versions: VersionsInfo,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeInfo {
    pub uptime: u64,
    pub current: u64,
}

// ——— Full response types ———

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub static_data: StaticData,
    pub cpu: CpuInfo,
    pub cpu_current_speed: CpuCurrentSpeed,
    pub current_load: CurrentLoad,
    pub cpu_temperature: CpuTemperature,
    pub graphics: GraphicsInfo,
    pub network: NetworkInfo,
    pub storage: StorageInfo,
    pub memory: MemoryInfo,
    pub audio: AudioInfo,
    pub peripherals: PeripheralInfo,
    pub optical: OpticalInfo,
    pub runtime: RuntimeInfo,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LiveInfo {
    pub cpu_current_speed: CpuCurrentSpeed,
    pub current_load: CurrentLoad,
    pub cpu_temperature: CpuTemperature,
    pub memory: MemoryInfo,
    pub runtime: RuntimeInfo,
}



// ——— Collection functions ———

pub fn collect_hardware_info() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    // Allow CPU usage to be measured (needs 2 refreshes with delay)
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    HardwareInfo {
        static_data: collect_static_data(&sys),
        cpu: collect_cpu_info(&sys),
        cpu_current_speed: collect_cpu_speed(&sys),
        current_load: collect_cpu_load(&sys),
        cpu_temperature: collect_cpu_temp(),
        graphics: collect_graphics(),
        network: collect_network(),
        storage: collect_storage(),
        memory: collect_memory(&sys),
        audio: collect_audio(),
        peripherals: collect_peripherals(),
        optical: collect_optical(),
        runtime: collect_runtime(),
    }
}

pub fn collect_live_info() -> LiveInfo {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();

    LiveInfo {
        cpu_current_speed: collect_cpu_speed(&sys),
        current_load: collect_cpu_load(&sys),
        cpu_temperature: collect_cpu_temp(),
        memory: collect_memory_live(&sys),
        runtime: collect_runtime(),
    }
}

fn collect_cpu_info(sys: &System) -> CpuInfo {
    let cpus = sys.cpus();
    let first = cpus.first();

    let brand = first.map(|c| c.brand().to_string()).unwrap_or_default();
    let vendor = first.map(|c| c.vendor_id().to_string()).unwrap_or_default();
    let physical_cores = sys.physical_core_count().unwrap_or(0);
    let cores = cpus.len();

    // Frequencies
    let freqs: Vec<f64> = cpus.iter().map(|c| c.frequency() as f64 / 1000.0).collect();
    let speed = freqs.iter().copied().sum::<f64>() / freqs.len().max(1) as f64;
    let speed_max = freqs.iter().copied().fold(0.0_f64, f64::max);

    // Cache
    let (l2, l3) = read_cache_sizes();

    // CPU family/model/stepping
    let (family, model, stepping) = read_cpu_ids();

    CpuInfo {
        brand,
        vendor,
        family,
        model,
        stepping,
        physical_cores,
        cores,
        speed,
        speed_max,
        cache: CpuCache { l2, l3 },
    }
}

fn collect_cpu_speed(sys: &System) -> CpuCurrentSpeed {
    let cpus = sys.cpus();
    let freqs: Vec<f64> = cpus.iter().map(|c| c.frequency() as f64 / 1000.0).collect();
    let len = freqs.len().max(1) as f64;
    CpuCurrentSpeed {
        avg: freqs.iter().copied().sum::<f64>() / len,
        min: freqs.iter().copied().fold(f64::MAX, f64::min),
        max: freqs.iter().copied().fold(0.0_f64, f64::max),
    }
}

fn collect_cpu_load(sys: &System) -> CurrentLoad {
    let cpus = sys.cpus();
    let total: f64 = cpus.iter().map(|c| c.cpu_usage() as f64).sum();
    let avg = total / cpus.len().max(1) as f64;
    CurrentLoad { current_load: avg }
}

fn collect_cpu_temp() -> CpuTemperature {
    let components = Components::new_with_refreshed_list();
    let mut main_temp = 0.0_f64;
    let mut max_temp = 0.0_f64;

    for comp in components.iter() {
        let label = comp.label().to_lowercase();
        let temp = comp.temperature() as f64;
        if label.contains("core") || label.contains("cpu") || label.contains("package") || label.contains("tctl") {
            if label.contains("package") || label.contains("tctl") || main_temp == 0.0 {
                main_temp = temp;
            }
            if temp > max_temp {
                max_temp = temp;
            }
        }
    }

    CpuTemperature {
        main: main_temp,
        max: max_temp,
    }
}

fn collect_memory(sys: &System) -> MemoryInfo {
    MemoryInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
        available: sys.available_memory(),
        active: sys.used_memory(),
        swaptotal: sys.total_swap(),
        swapused: sys.used_swap(),
        swapfree: sys.total_swap().saturating_sub(sys.used_swap()),
        layout: read_memory_layout(),
    }
}

fn collect_memory_live(sys: &System) -> MemoryInfo {
    MemoryInfo {
        total: sys.total_memory(),
        used: sys.used_memory(),
        available: sys.available_memory(),
        active: sys.used_memory(),
        swaptotal: sys.total_swap(),
        swapused: sys.used_swap(),
        swapfree: sys.total_swap().saturating_sub(sys.used_swap()),
        layout: Vec::new(),
    }
}

fn collect_graphics() -> GraphicsInfo {
    GraphicsInfo {
        controllers: read_gpu_info(),
    }
}

fn collect_storage() -> StorageInfo {
    let disks = Disks::new_with_refreshed_list();

    let mut disk_layout: Vec<DiskLayoutEntry> = Vec::new();
    let mut filesystems: Vec<FilesystemEntry> = Vec::new();

    for disk in disks.iter() {
        let name = disk.name().to_string_lossy().to_string();
        let mount = disk.mount_point().to_string_lossy().to_string();
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);
        let usage_pct = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // Try to determine disk type from the kind field
        let disk_type = format!("{:?}", disk.kind());

        // Add to filesystems
        filesystems.push(FilesystemEntry {
            mount: mount.clone(),
            fs_type: disk
                .file_system()
                .to_string_lossy()
                .to_string(),
            used,
            size: total,
            usage_pct,
        });

        // Add to disk layout (only if name is not already present)
        if !disk_layout.iter().any(|d| d.name == name) {
            disk_layout.push(DiskLayoutEntry {
                name,
                disk_type,
                size: total,
                interface_type: String::new(),
            });
        }
    }

    StorageInfo {
        disk_layout,
        filesystems,
    }
}

fn collect_network() -> NetworkInfo {
    let networks = Networks::new_with_refreshed_list();
    let mut interfaces = Vec::new();

    for (name, _net) in networks.iter() {
        interfaces.push(NetworkInterface {
            iface: name.to_string(),
            ip4: String::new(),
            ip6: String::new(),
            mac: String::new(), 
        });
    }

    NetworkInfo { interfaces }
}

fn collect_static_data(sys: &System) -> StaticData {
    StaticData {
        baseboard: read_baseboard_info(),
        bios: read_bios_info(),
        os: read_os_info(sys),
        uuid: read_uuid_info(),
        versions: VersionsInfo {
            node: "N/A (Rust backend)".to_string(),
        },
    }
}

fn collect_runtime() -> RuntimeInfo {
    let uptime = System::uptime();
    let current = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    RuntimeInfo { uptime, current }
}

#[cfg(target_os = "linux")]
fn collect_audio() -> AudioInfo {
    let output = Command::new("aplay").arg("-l").output();
    let mut devices = Vec::new();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            if line.starts_with("card") {
                // card 0: PCH [HDA Intel PCH], device 0: ALC3246 Analog...
                let parts: Vec<&str> = line.splitn(4, ':').collect();
                if parts.len() >= 2 {
                    let name_part = parts[1].trim();
                    let name = name_part.split('[').next().unwrap_or(name_part).trim().to_string();
                    let manufacturer = if name_part.contains('[') {
                        name_part.split('[').nth(1).and_then(|s| s.split(']').next()).unwrap_or("Unknown").to_string()
                    } else {
                        "Unknown".to_string()
                    };
                    
                    devices.push(AudioDevice {
                        name,
                        manufacturer,
                        status: "Active".to_string(),
                    });
                }
            }
        }
    }

    AudioInfo { devices }
}

#[cfg(target_os = "linux")]
fn collect_peripherals() -> PeripheralInfo {
    let output = Command::new("lsusb").output();
    let mut usb_devices = Vec::new();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            // Bus 002 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 7 && parts[4] == "ID" {
                let ids: Vec<&str> = parts[5].split(':').collect();
                if ids.len() == 2 {
                    let vendor_id = ids[0].to_string();
                    let product_id = ids[1].to_string();
                    let bus = parts[1].to_string();
                    let device = parts[3].trim_end_matches(':').to_string();
                    let name = parts[6..].join(" ");
                    
                    usb_devices.push(UsbDevice {
                        name,
                        vendor: "USB Device".to_string(), // lsusb doesn't give vendor name easily without -v or a database
                        vendor_id,
                        product_id,
                        bus,
                        device,
                    });
                }
            }
        }
    }

    PeripheralInfo { usb_devices }
}

#[cfg(not(target_os = "linux"))]
fn collect_audio() -> AudioInfo {
    AudioInfo::default()
}

#[cfg(not(target_os = "linux"))]
fn collect_peripherals() -> PeripheralInfo {
    PeripheralInfo::default()
}

#[cfg(target_os = "linux")]
fn collect_optical() -> OpticalInfo {
    let mut devices = Vec::new();
    // Try to read from procfs
    if let Ok(info) = std::fs::read_to_string("/proc/sys/dev/cdrom/info") {
        let mut drive_name = String::new();
        // The file format is a bit weird, with labels followed by values for each drive
        for line in info.lines() {
            if line.starts_with("drive name:") {
                drive_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
            if !drive_name.is_empty() && line.starts_with("drive model:") {
                let model = line.split(':').nth(1).unwrap_or("").trim().to_string();
                devices.push(OpticalDevice {
                    name: drive_name.clone(),
                    model,
                    vendor: "Unknown".to_string(),
                });
                drive_name = String::new(); // Reset for multiple drives
            }
        }
    }
    
    // Fallback: check /dev/sr0 if procfs was empty/failed to parse correctly
    if devices.is_empty() {
        if std::path::Path::new("/dev/sr0").exists() {
            devices.push(OpticalDevice {
                name: "sr0".to_string(),
                model: "CD/DVD Drive".to_string(),
                vendor: "Unknown".to_string(),
            });
        }
    }

    OpticalInfo { devices }
}

#[cfg(not(target_os = "linux"))]
fn collect_optical() -> OpticalInfo {
    OpticalInfo::default()
}

// ——— Platform-specific helpers (Linux) ———

#[cfg(target_os = "linux")]
fn read_cache_sizes() -> (u64, u64) {
    let mut l2: u64 = 0;
    let mut l3: u64 = 0;

    // Read from sysfs
    for index in 0..10 {
        let level_path = format!(
            "/sys/devices/system/cpu/cpu0/cache/index{}/level",
            index
        );
        let size_path = format!(
            "/sys/devices/system/cpu/cpu0/cache/index{}/size",
            index
        );

        if let (Ok(level), Ok(size_str)) = (
            std::fs::read_to_string(&level_path),
            std::fs::read_to_string(&size_path),
        ) {
            let level_num: u32 = level.trim().parse().unwrap_or(0);
            let size_str = size_str.trim();
            let size_bytes = parse_cache_size(size_str);

            match level_num {
                2 => l2 = size_bytes,
                3 => l3 = size_bytes,
                _ => {}
            }
        }
    }

    (l2, l3)
}

#[cfg(target_os = "linux")]
fn parse_cache_size(s: &str) -> u64 {
    let s = s.trim();
    if s.ends_with('K') {
        s[..s.len() - 1].parse::<u64>().unwrap_or(0) * 1024
    } else if s.ends_with('M') {
        s[..s.len() - 1].parse::<u64>().unwrap_or(0) * 1024 * 1024
    } else {
        s.parse::<u64>().unwrap_or(0)
    }
}

#[cfg(target_os = "linux")]
fn read_cpu_ids() -> (String, String, String) {
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let mut family = String::new();
    let mut model = String::new();
    let mut stepping = String::new();

    for line in cpuinfo.lines() {
        if line.starts_with("cpu family") {
            family = line.split(':').last().unwrap_or("").trim().to_string();
        } else if line.starts_with("model\t") || line.starts_with("model ") {
            if !line.starts_with("model name") {
                model = line.split(':').last().unwrap_or("").trim().to_string();
            }
        } else if line.starts_with("stepping") {
            stepping = line.split(':').last().unwrap_or("").trim().to_string();
        }
        if !family.is_empty() && !model.is_empty() && !stepping.is_empty() {
            break;
        }
    }

    (family, model, stepping)
}

#[cfg(target_os = "linux")]
fn read_memory_layout() -> Vec<MemorySlot> {
    // Try dmidecode first (needs root)
    let output = Command::new("dmidecode")
        .args(["-t", "17"])
        .output();

    let text = match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
        _ => {
            // Fallback to pkexec for GUI prompt
            let pk_output = Command::new("pkexec")
                .arg("dmidecode")
                .args(["-t", "17"])
                .output();
            
            match pk_output {
                Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
                _ => String::new(),
            }
        }
    };

    if text.is_empty() {
        return Vec::new();
    }

    parse_dmidecode_memory(&text)
}

#[cfg(target_os = "linux")]
fn parse_dmidecode_memory(text: &str) -> Vec<MemorySlot> {
    let mut slots = Vec::new();
    let mut current: Option<MemorySlot> = None;
    let mut slot_index: usize = 0;

    for line in text.lines() {
        let line = line.trim();

        if line.starts_with("Memory Device") {
            if let Some(slot) = current.take() {
                if slot.size > 0 {
                    slots.push(slot);
                }
            }
            current = Some(MemorySlot {
                slot: slot_index,
                ..Default::default()
            });
            slot_index += 1;
        }

        if let Some(ref mut slot) = current {
            if let Some(val) = line.strip_prefix("Size:") {
                let val = val.trim();
                if let Some(mb_str) = val.strip_suffix("MB") {
                    slot.size = mb_str.trim().parse::<u64>().unwrap_or(0) * 1024 * 1024;
                } else if let Some(gb_str) = val.strip_suffix("GB") {
                    slot.size = gb_str.trim().parse::<u64>().unwrap_or(0) * 1024 * 1024 * 1024;
                }
            } else if let Some(val) = line.strip_prefix("Speed:") {
                let val = val.trim();
                if let Some(mhz_str) = val.strip_suffix("MT/s") {
                    slot.clock_speed = mhz_str.trim().parse().unwrap_or(0);
                } else if let Some(mhz_str) = val.strip_suffix("MHz") {
                    slot.clock_speed = mhz_str.trim().parse().unwrap_or(0);
                }
            } else if let Some(val) = line.strip_prefix("Type:") {
                slot.mem_type = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("Form Factor:") {
                slot.form_factor = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("Manufacturer:") {
                slot.manufacturer = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("Part Number:") {
                slot.part_num = val.trim().to_string();
            } else if let Some(val) = line.strip_prefix("Serial Number:") {
                slot.serial_num = val.trim().to_string();
            }
        }
    }

    if let Some(slot) = current {
        if slot.size > 0 {
            slots.push(slot);
        }
    }

    slots
}

#[cfg(target_os = "linux")]
fn read_gpu_info() -> Vec<GpuController> {
    // Parse lspci output
    let lspci_output = Command::new("lspci").output();

    match lspci_output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            let mut gpus = Vec::new();

            for line in text.lines() {
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                    // Extract the PCI slot address (first field)
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    if parts.len() >= 2 {
                        let description = parts[1];
                        // Try to extract meaningful info
                        let model = description
                            .split(':')
                            .last()
                            .unwrap_or(description)
                            .trim()
                            .to_string();

                        let vendor = if description.contains("NVIDIA") {
                            "NVIDIA".to_string()
                        } else if description.contains("AMD") || description.contains("ATI") {
                            "AMD".to_string()
                        } else if description.contains("Intel") {
                            "Intel".to_string()
                        } else {
                            String::new()
                        };

                        let bus = parts[0].to_string();

                        gpus.push(GpuController {
                            model,
                            vendor,
                            vram: 0, // Not easily available from lspci
                            bus,
                        });
                    }
                }
            }
            gpus
        }
        _ => Vec::new(),
    }
}

#[cfg(target_os = "linux")]
fn read_sysfs_file(path: &str) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_default()
        .trim()
        .to_string()
}

#[cfg(target_os = "linux")]
fn read_baseboard_info() -> BaseboardInfo {
    // Try sysfs first (no root needed)
    let manufacturer = read_sysfs_file("/sys/class/dmi/id/board_vendor");
    let model = read_sysfs_file("/sys/class/dmi/id/board_name");
    let version = read_sysfs_file("/sys/class/dmi/id/board_version");
    // serial requires root via sysfs too
    let serial = read_sysfs_file("/sys/class/dmi/id/board_serial");

    BaseboardInfo {
        manufacturer,
        model,
        version,
        serial,
    }
}

#[cfg(target_os = "linux")]
fn read_bios_info() -> BiosInfo {
    BiosInfo {
        vendor: read_sysfs_file("/sys/class/dmi/id/bios_vendor"),
        version: read_sysfs_file("/sys/class/dmi/id/bios_version"),
        release_date: read_sysfs_file("/sys/class/dmi/id/bios_date"),
    }
}

#[cfg(target_os = "linux")]
fn read_uuid_info() -> UuidInfo {
    let networks = Networks::new_with_refreshed_list();
    let mut macs: Vec<String> = Vec::new();

    for (name, _net) in networks.iter() {
        // Try reading the MAC from sysfs
        let path = format!("/sys/class/net/{}/address", name);
        let mac = read_sysfs_file(&path);
        if !mac.is_empty() && mac != "00:00:00:00:00:00" {
            macs.push(mac);
        }
    }

    UuidInfo { macs }
}




#[cfg(target_os = "windows")]
fn read_cpu_ids() -> (String, String, String) {
    (String::new(), String::new(), String::new())
}

#[cfg(target_os = "windows")]
fn read_cache_sizes() -> (u64, u64) {
    // Simplification: Return 0 or try to fetch from Win32_CacheMemory
    // For now, returning 0 to match safe default
    (0, 0) 
}

// ——— PowerShell Helper ———

#[cfg(target_os = "windows")]
fn exec_powershell<T: serde::de::DeserializeOwned>(script: &str) -> Option<T> {
    use std::os::windows::process::CommandExt;
    
    // CREATE_NO_WINDOW flag to prevent console popups
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = std::process::Command::new("powershell")
        .args(&["-NoProfile", "-Command", script])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;

    if !output.status.success() {
        eprintln!("PowerShell command failed: {}", String::from_utf8_lossy(&output.stderr));
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Handle single object vs array: ConvertTo-Json might return single obj if only 1 result.
    // We can force array in PS or handle it here.
    // Better to use `ConvertTo-Json -AsArray` if available (PS 7+), but standard PS 5.1 doesn't have it easily?
    // Actually, wrap in @() in PS: @(Get-CimInstance...)
    
    match serde_json::from_str::<T>(&stdout) {
        Ok(res) => Some(res),
        Err(e) => {
            eprintln!("Failed to parse JSON from PowerShell: {:?}\nOutput: {}", e, stdout);
            None
        }
    }
}

// ——— Wrapper Structs for PowerShell JSON ———

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PsGpu {
    Name: Option<String>,
    VideoProcessor: Option<String>,
    AdapterRAM: Option<u64>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PsMem {
    Capacity: Option<u64>,
    Speed: Option<u32>,
    Manufacturer: Option<String>,
    PartNumber: Option<String>,
    SerialNumber: Option<String>,
    FormFactor: Option<u16>,
    MemoryType: Option<u16>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PsBoard {
    Manufacturer: Option<String>,
    Product: Option<String>,
    Version: Option<String>,
    SerialNumber: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PsBios {
    Manufacturer: Option<String>,
    SMBIOSBIOSVersion: Option<String>,
    ReleaseDate: Option<String>, // PowerShell date format might be weird
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct PsNetAdapter {
    MACAddress: Option<String>,
    IPEnabled: Option<bool>,
}

// ——— Implementation ———

#[cfg(target_os = "windows")]
fn read_gpu_info() -> Vec<GpuController> {
    let script = "@(Get-CimInstance Win32_VideoController) | Select-Object Name, VideoProcessor, AdapterRAM | ConvertTo-Json -Compress";
    
    let gpus: Option<Vec<PsGpu>> = exec_powershell(script);
    let mut res = Vec::new();
    
    if let Some(ps_gpus) = gpus {
       for gpu in ps_gpus {
           res.push(GpuController {
               model: gpu.Name.unwrap_or_default(),
               vendor: gpu.VideoProcessor.clone().unwrap_or_else(|| "Unknown".to_string()),
               vram: gpu.AdapterRAM.unwrap_or(0),
               bus: "PCI".to_string(), 
           });
       }
    }
    res
}

#[cfg(target_os = "windows")]
fn read_memory_layout() -> Vec<MemorySlot> {
    let script = "@(Get-CimInstance Win32_PhysicalMemory) | Select-Object Capacity, Speed, Manufacturer, PartNumber, SerialNumber, FormFactor, MemoryType | ConvertTo-Json -Compress";
    let mems: Option<Vec<PsMem>> = exec_powershell(script);
    let mut slots = Vec::new();

    if let Some(ps_mems) = mems {
        for (i, mem) in ps_mems.iter().enumerate() {
            slots.push(MemorySlot {
                slot: i,
                size: mem.Capacity.unwrap_or(0),
                clock_speed: mem.Speed.unwrap_or(0) as u64,
                mem_type: format!("{}", mem.MemoryType.unwrap_or(0)),
                form_factor: format!("{}", mem.FormFactor.unwrap_or(0)),
                manufacturer: mem.Manufacturer.clone().unwrap_or_default(),
                part_num: mem.PartNumber.clone().unwrap_or_default(),
                serial_num: mem.SerialNumber.clone().unwrap_or_default(),
            });
        }
    }
    slots
}

#[cfg(target_os = "windows")]
fn read_baseboard_info() -> BaseboardInfo {
    let script = "@(Get-CimInstance Win32_BaseBoard) | Select-Object Manufacturer, Product, Version, SerialNumber | ConvertTo-Json -Compress";
    let boards: Option<Vec<PsBoard>> = exec_powershell(script);
    
    if let Some(boards) = boards {
        if let Some(b) = boards.first() {
            return BaseboardInfo {
                manufacturer: b.Manufacturer.clone().unwrap_or_default(),
                model: b.Product.clone().unwrap_or_default(),
                version: b.Version.clone().unwrap_or_default(),
                serial: b.SerialNumber.clone().unwrap_or_default(),
            };
        }
    }
    BaseboardInfo::default()
}

#[cfg(target_os = "windows")]
fn read_bios_info() -> BiosInfo {
    let script = "@(Get-CimInstance Win32_BIOS) | Select-Object Manufacturer, SMBIOSBIOSVersion, ReleaseDate | ConvertTo-Json -Compress";
    let bioses: Option<Vec<PsBios>> = exec_powershell(script);
    
    if let Some(list) = bioses {
        if let Some(b) = list.first() {
             // ReleaseDate in WMI is often YYYYMMDDHHMMSS... encoded string.
             // But ConvertTo-Json might output different structure?
             // Usually it's a string like "/Date(123456)/" or raw string.
             // We'll just take it as string for now.
             let date_str = b.ReleaseDate.clone().unwrap_or_default();
             return BiosInfo {
                 vendor: b.Manufacturer.clone().unwrap_or_default(),
                 version: b.SMBIOSBIOSVersion.clone().unwrap_or_default(),
                 release_date: date_str,
             };
        }
    }
    BiosInfo::default()
}


#[cfg(target_os = "windows")]
fn read_uuid_info() -> UuidInfo {
    // We need MAC addresses of IPEnabled adapters
    let script = "@(Get-CimInstance Win32_NetworkAdapterConfiguration -Filter 'IPEnabled=True') | Select-Object MACAddress | ConvertTo-Json -Compress";
    let adapters: Option<Vec<PsNetAdapter>> = exec_powershell(script);
    let mut macs = Vec::new();

    if let Some(list) = adapters {
        for a in list {
            if let Some(mac) = a.MACAddress {
                macs.push(mac);
            }
        }
    }
    UuidInfo { macs }
}



// ——— Common helpers ———

fn read_os_info(sys: &System) -> OsInfo {
    let hostname = System::host_name().unwrap_or_default();
    let kernel = System::kernel_version().unwrap_or_default();
    let name = System::name().unwrap_or_default();
    let os_version = System::os_version().unwrap_or_default();
    let long_os_version = System::long_os_version().unwrap_or_default();
    let arch = System::cpu_arch().unwrap_or_default();

    // Check for UEFI
    #[cfg(target_os = "linux")]
    let uefi = std::path::Path::new("/sys/firmware/efi").exists();
    #[cfg(not(target_os = "linux"))]
    let uefi = false; // Simplified

    let platform = if cfg!(target_os = "linux") {
        "linux".to_string()
    } else if cfg!(target_os = "windows") {
        "win32".to_string()
    } else if cfg!(target_os = "macos") {
        "darwin".to_string()
    } else {
        std::env::consts::OS.to_string()
    };

    let _ = sys; 

    OsInfo {
        platform,
        distro: long_os_version.clone(),
        release: os_version,
        hostname: hostname.clone(),
        kernel,
        arch,
        fqdn: name,
        uefi,
    }
}
