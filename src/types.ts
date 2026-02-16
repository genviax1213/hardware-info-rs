// TypeScript interfaces matching the Rust structs

export interface CpuCache {
  l2: number;
  l3: number;
}

export interface CpuInfo {
  brand: string;
  vendor: string;
  family: string;
  model: string;
  stepping: string;
  physicalCores: number;
  cores: number;
  speed: number;
  speedMax: number;
  cache: CpuCache;
}

export interface CpuCurrentSpeed {
  avg: number;
  min: number;
  max: number;
}

export interface CurrentLoad {
  currentLoad: number;
}

export interface CpuTemperature {
  main: number;
  max: number;
}

export interface MemorySlot {
  slot: number;
  size: number;
  clockSpeed: number;
  type: string;
  formFactor: string;
  manufacturer: string;
  partNum: string;
  serialNum: string;
}

export interface MemoryInfo {
  total: number;
  used: number;
  available: number;
  active: number;
  swaptotal: number;
  swapused: number;
  swapfree: number;
  layout: MemorySlot[];
}

export interface GpuController {
  model: string;
  vendor: string;
  vram: number;
  bus: string;
}

export interface GraphicsInfo {
  controllers: GpuController[];
}

export interface DiskLayoutEntry {
  name: string;
  type: string;
  size: number;
  interfaceType: string;
}

export interface FilesystemEntry {
  mount: string;
  type: string;
  used: number;
  size: number;
  use: number;
}

export interface StorageInfo {
  diskLayout: DiskLayoutEntry[];
  filesystems: FilesystemEntry[];
}

export interface NetworkInterface {
  iface: string;
  ip4: string;
  ip6: string;
  mac: string;
}

export interface NetworkInfo {
  interfaces: NetworkInterface[];
}

export interface AudioDevice {
  name: string;
  manufacturer: string;
  status: string;
}

export interface AudioInfo {
  devices: AudioDevice[];
}

export interface UsbDevice {
  name: string;
  vendor: string;
  vendorId: string;
  productId: string;
  bus: string;
  device: string;
}

export interface PeripheralInfo {
  usbDevices: UsbDevice[];
}

export interface OpticalDevice {
  name: string;
  model: string;
  vendor: string;
}

export interface OpticalInfo {
  devices: OpticalDevice[];
}

export interface BaseboardInfo {
  manufacturer: string;
  model: string;
  version: string;
  serial: string;
}

export interface BiosInfo {
  vendor: string;
  version: string;
  releaseDate: string;
}

export interface OsInfo {
  platform: string;
  distro: string;
  release: string;
  hostname: string;
  kernel: string;
  arch: string;
  fqdn: string;
  uefi: boolean;
}

export interface UuidInfo {
  macs: string[];
}

export interface VersionsInfo {
  node: string;
}

export interface StaticData {
  baseboard: BaseboardInfo;
  bios: BiosInfo;
  os: OsInfo;
  uuid: UuidInfo;
  versions: VersionsInfo;
}

export interface RuntimeInfo {
  uptime: number;
  current: number;
}

export interface HardwareInfo {
  staticData: StaticData;
  cpu: CpuInfo;
  cpuCurrentSpeed: CpuCurrentSpeed;
  currentLoad: CurrentLoad;
  cpuTemperature: CpuTemperature;
  graphics: GraphicsInfo;
  network: NetworkInfo;
  storage: StorageInfo;
  memory: MemoryInfo;
  audio: AudioInfo;
  peripherals: PeripheralInfo;
  optical: OpticalInfo;
  runtime: RuntimeInfo;
}

export interface LiveInfo {
  cpuCurrentSpeed: CpuCurrentSpeed;
  currentLoad: CurrentLoad;
  cpuTemperature: CpuTemperature;
  memory: MemoryInfo;
  runtime: RuntimeInfo;
}
