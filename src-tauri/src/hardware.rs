use serde::{Deserialize, Serialize};
use sysinfo::{Components, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

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

// ——— WMI Structs (Windows only) ———

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32VideoController {
    name: String,
    video_processor: Option<String>,
    adapter_r_a_m: Option<u64>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32PhysicalMemory {
    capacity: Option<u64>,
    speed: Option<u32>,
    manufacturer: Option<String>,
    part_number: Option<String>,
    serial_number: Option<String>,
    form_factor: Option<u16>,
    memory_type: Option<u16>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32BaseBoard {
    manufacturer: Option<String>,
    product: Option<String>, // Model
    version: Option<String>,
    serial_number: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32BIOS {
    manufacturer: Option<String>,
    #[serde(rename = "SMBIOSBIOSVersion")]
    smbios_bios_version: Option<String>,
    release_date: Option<String>,
}

#[cfg(target_os = "windows")]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Win32NetworkAdapterConfiguration {
    #[serde(rename = "MACAddress")]
    mac_address: Option<String>,
    #[serde(rename = "IPEnabled")]
    ip_enabled: Option<bool>,
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


// ——— Platform-specific helpers (Windows) ———

#[cfg(target_os = "windows")]
fn get_wmi() -> Result<WMIConnection, wmi::WMIError> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con)?;
    Ok(wmi_con)
}

#[cfg(target_os = "windows")]
fn read_cache_sizes() -> (u64, u64) {
    // Simplification: Return 0 or try to fetch from Win32_CacheMemory
    // For now, returning 0 to match safe default
    (0, 0) 
}

#[cfg(target_os = "windows")]
fn read_cpu_ids() -> (String, String, String) {
    (String::new(), String::new(), String::new())
}

#[cfg(target_os = "windows")]
fn read_memory_layout() -> Vec<MemorySlot> {
    let mut slots = Vec::new();
    if let Ok(wmi_con) = get_wmi() {
        if let Ok(results) = wmi_con.query::<Win32PhysicalMemory>() {
            for (index, mem) in results.iter().enumerate() {
                slots.push(MemorySlot {
                    slot: index,
                    size: mem.capacity.unwrap_or(0),
                    clock_speed: mem.speed.unwrap_or(0) as u64,
                    mem_type: format!("{:?}", mem.memory_type.unwrap_or(0)),
                    form_factor: format!("{:?}", mem.form_factor.unwrap_or(0)),
                    manufacturer: mem.manufacturer.clone().unwrap_or_default(),
                    part_num: mem.part_number.clone().unwrap_or_default(),
                    serial_num: mem.serial_number.clone().unwrap_or_default(),
                });
            }
        }
    }
    slots
}

#[cfg(target_os = "windows")]
fn read_gpu_info() -> Vec<GpuController> {
    let mut gpus = Vec::new();
    if let Ok(wmi_con) = get_wmi() {
        if let Ok(results) = wmi_con.query::<Win32VideoController>() {
            for gpu in results {
                gpus.push(GpuController {
                    model: gpu.name.clone(),
                    vendor: gpu.video_processor.clone().unwrap_or_default(),
                    vram: gpu.adapter_r_a_m.unwrap_or(0),
                    bus: "PCI".to_string(), // Simplified
                });
            }
        }
    }
    gpus
}

#[cfg(target_os = "windows")]
fn read_baseboard_info() -> BaseboardInfo {
    let mut info = BaseboardInfo::default();
    if let Ok(wmi_con) = get_wmi() {
        if let Ok(results) = wmi_con.query::<Win32BaseBoard>() {
            if let Some(board) = results.first() {
                info.manufacturer = board.manufacturer.clone().unwrap_or_default();
                info.model = board.product.clone().unwrap_or_default();
                info.version = board.version.clone().unwrap_or_default();
                info.serial = board.serial_number.clone().unwrap_or_default();
            }
        }
    }
    info
}

#[cfg(target_os = "windows")]
fn read_bios_info() -> BiosInfo {
    let mut info = BiosInfo::default();
    if let Ok(wmi_con) = get_wmi() {
        if let Ok(results) = wmi_con.query::<Win32BIOS>() {
            if let Some(bios) = results.first() {
                info.vendor = bios.manufacturer.clone().unwrap_or_default();
                info.version = bios.smbios_bios_version.clone().unwrap_or_default();
                info.release_date = bios.release_date.clone().unwrap_or_default();
            }
        }
    }
    info
}

#[cfg(target_os = "windows")]
fn read_uuid_info() -> UuidInfo {
    let mut macs = Vec::new();
    if let Ok(wmi_con) = get_wmi() {
        if let Ok(results) = wmi_con.query::<Win32NetworkAdapterConfiguration>() {
            for adapter in results {
                if adapter.ip_enabled.unwrap_or(false) {
                    if let Some(mac) = adapter.mac_address {
                        macs.push(mac);
                    }
                }
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
