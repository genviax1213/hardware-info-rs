import { useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';
import type {
  HardwareInfo,
  LiveInfo,
  CpuInfo,
  CpuCurrentSpeed,
  CurrentLoad,
  CpuTemperature,
  MemoryInfo,
  GraphicsInfo,
  StorageInfo,
  NetworkInfo,
  StaticData,
  MemorySlot,
  GpuController,
  DiskLayoutEntry,
  FilesystemEntry,
} from './types';

type TabKey =
  | 'cpu'
  | 'mainboard'
  | 'memory'
  | 'spd'
  | 'graphics'
  | 'storage'
  | 'sensors'
  | 'system'
  | 'about';

const tabs: { key: TabKey; label: string }[] = [
  { key: 'cpu', label: 'CPU' },
  { key: 'mainboard', label: 'Mainboard' },
  { key: 'memory', label: 'Memory' },
  { key: 'spd', label: 'SPD' },
  { key: 'graphics', label: 'Graphics' },
  { key: 'storage', label: 'Storage' },
  { key: 'sensors', label: 'Sensors' },
  { key: 'system', label: 'System' },
  { key: 'about', label: 'About' },
];

const dash = '—';

function formatBytes(value?: number) {
  if (!value || value < 0) return dash;
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = value;
  let idx = 0;
  while (size >= 1024 && idx < units.length - 1) {
    size /= 1024;
    idx += 1;
  }
  return `${size.toFixed(idx > 1 ? 1 : 0)} ${units[idx]}`;
}

function formatPercent(value?: number) {
  if (value === undefined || value === null || Number.isNaN(value)) return dash;
  return `${value.toFixed(1)}%`;
}

function formatGHz(value?: number) {
  if (value === undefined || value === null || Number.isNaN(value) || value <= 0) return dash;
  return `${value.toFixed(2)} GHz`;
}

function formatSeconds(value?: number) {
  if (!value || value < 0) return dash;
  const hrs = Math.floor(value / 3600);
  const mins = Math.floor((value % 3600) / 60);
  return `${hrs}h ${mins}m`;
}

function Row({ label, value }: { label: string; value: string | number }) {
  return (
    <div className="spec-row">
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

// Merged type for the combined hardware + live data
interface MergedData {
  staticData: StaticData;
  cpu: CpuInfo;
  cpuCurrentSpeed: CpuCurrentSpeed;
  currentLoad: CurrentLoad;
  cpuTemperature: CpuTemperature;
  graphics: GraphicsInfo;
  network: NetworkInfo;
  storage: StorageInfo;
  memory: MemoryInfo;
  runtime: { uptime: number; current: number };
}

function App() {
  const [hardwareInfo, setHardwareInfo] = useState<HardwareInfo | null>(null);
  const [liveInfo, setLiveInfo] = useState<LiveInfo | null>(null);
  const [activeTab, setActiveTab] = useState<TabKey>('cpu');
  const [error, setError] = useState<string>('');

  useEffect(() => {
    let timer: ReturnType<typeof setInterval>;

    const fetchInitial = async () => {
      try {
        const data = await invoke<HardwareInfo>('get_hardware_info');
        setHardwareInfo(data);
        setError('');
      } catch (fetchError) {
        setError('Unable to load hardware data.');
        console.error(fetchError);
      }
    };

    const fetchLive = async () => {
      try {
        const data = await invoke<LiveInfo>('get_hardware_live');
        setLiveInfo(data);
      } catch (fetchError) {
        console.error(fetchError);
      }
    };

    fetchInitial().then(fetchLive);
    timer = setInterval(fetchLive, 3000);
    return () => clearInterval(timer);
  }, []);

  const merged: MergedData | null = useMemo(() => {
    if (!hardwareInfo) return null;
    return {
      ...hardwareInfo,
      ...(liveInfo || {}),
      memory: {
        ...(hardwareInfo.memory || {}),
        ...((liveInfo && liveInfo.memory) || {}),
        // Preserve layout from static data
        layout: hardwareInfo.memory?.layout || [],
      },
      runtime: {
        ...(hardwareInfo.runtime || {}),
        ...((liveInfo && liveInfo.runtime) || {}),
      },
    } as MergedData;
  }, [hardwareInfo, liveInfo]);

  if (!merged) {
    return (
      <div className="dashboard-shell">
        <div className="loading">{error || 'Loading hardware info...'}</div>
      </div>
    );
  }

  const cpu = merged.cpu;
  const staticData = merged.staticData;
  const memory = merged.memory;
  const graphics = merged.graphics;
  const storage = merged.storage;
  const network = merged.network;
  const temp = merged.cpuTemperature;
  const speed = merged.cpuCurrentSpeed;
  const load = merged.currentLoad;

  const renderCpu = () => (
    <div className="tab-grid">
      <section className="spec-card">
        <h3>Processor</h3>
        <Row label="Name" value={cpu.brand || dash} />
        <Row label="Vendor" value={cpu.vendor || dash} />
        <Row label="Family / Model / Stepping" value={`${cpu.family || dash} / ${cpu.model || dash} / ${cpu.stepping || dash}`} />
        <Row label="Cores / Threads" value={`${cpu.physicalCores || dash} / ${cpu.cores || dash}`} />
        <Row label="Cache L2 / L3" value={`${formatBytes(cpu.cache?.l2)} / ${formatBytes(cpu.cache?.l3)}`} />
      </section>
      <section className="spec-card">
        <h3>Clocks & Load</h3>
        <Row label="Base Speed" value={formatGHz(cpu.speed)} />
        <Row label="Boost Speed" value={formatGHz(cpu.speedMax)} />
        <Row label="Current Speed" value={formatGHz(speed.avg)} />
        <Row label="Current Load" value={formatPercent(load.currentLoad)} />
      </section>
    </div>
  );

  const renderMainboard = () => (
    <div className="tab-grid">
      <section className="spec-card">
        <h3>Mainboard</h3>
        <Row label="Manufacturer" value={staticData.baseboard?.manufacturer || dash} />
        <Row label="Model" value={staticData.baseboard?.model || dash} />
        <Row label="Version" value={staticData.baseboard?.version || dash} />
        <Row label="Serial" value={staticData.baseboard?.serial || dash} />
      </section>
      <section className="spec-card">
        <h3>BIOS</h3>
        <Row label="Vendor" value={staticData.bios?.vendor || dash} />
        <Row label="Version" value={staticData.bios?.version || dash} />
        <Row label="Release Date" value={staticData.bios?.releaseDate || dash} />
        <Row label="UEFI" value={String(staticData.os?.uefi ?? dash)} />
      </section>
    </div>
  );

  const renderMemory = () => (
    <div className="tab-grid">
      <section className="spec-card">
        <h3>Memory Overview</h3>
        <Row label="Total" value={formatBytes(memory.total)} />
        <Row label="Used" value={formatBytes(memory.used)} />
        <Row label="Available" value={formatBytes(memory.available)} />
        <Row label="Active" value={formatBytes(memory.active)} />
        <Row label="Swap Used / Total" value={`${formatBytes(memory.swapused)} / ${formatBytes(memory.swaptotal)}`} />
      </section>
      <section className="spec-card">
        <h3>Runtime</h3>
        <Row label="Uptime" value={formatSeconds(merged.runtime?.uptime)} />
        <Row label="Kernel" value={staticData.os?.kernel || dash} />
        <Row label="Architecture" value={staticData.os?.arch || dash} />
        <Row label="Backend" value="Rust (Tauri)" />
      </section>
    </div>
  );

  const renderSpd = () => (
    <section className="spec-card full">
      <h3>SPD Slots</h3>
      <div className="table-wrap">
        <table>
          <thead>
            <tr>
              <th>Slot</th>
              <th>Size</th>
              <th>Type</th>
              <th>Clock</th>
              <th>Manufacturer</th>
              <th>Part #</th>
            </tr>
          </thead>
          <tbody>
            {(memory.layout || []).map((slot: MemorySlot) => (
              <tr key={slot.slot}>
                <td>{slot.slot}</td>
                <td>{formatBytes(slot.size)}</td>
                <td>{slot.type || dash}</td>
                <td>{slot.clockSpeed ? `${slot.clockSpeed} MHz` : dash}</td>
                <td>{slot.manufacturer || dash}</td>
                <td>{slot.partNum || dash}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );

  const renderGraphics = () => (
    <section className="spec-card full">
      <h3>Graphics Controllers</h3>
      <div className="table-wrap">
        <table>
          <thead>
            <tr>
              <th>Model</th>
              <th>Vendor</th>
              <th>VRAM</th>
              <th>Bus</th>
            </tr>
          </thead>
          <tbody>
            {(graphics.controllers || []).map((gpu: GpuController, index: number) => (
              <tr key={`${gpu.model || 'gpu'}-${index}`}>
                <td>{gpu.model || dash}</td>
                <td>{gpu.vendor || dash}</td>
                <td>{gpu.vram ? `${gpu.vram} MB` : dash}</td>
                <td>{gpu.bus || dash}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );

  const renderStorage = () => (
    <div className="tab-grid">
      <section className="spec-card full">
        <h3>Disk Layout</h3>
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>Name</th>
                <th>Type</th>
                <th>Size</th>
                <th>Interface</th>
              </tr>
            </thead>
            <tbody>
              {(storage.diskLayout || []).map((disk: DiskLayoutEntry, index: number) => (
                <tr key={`${disk.name || 'disk'}-${index}`}>
                  <td>{disk.name || dash}</td>
                  <td>{disk.type || dash}</td>
                  <td>{formatBytes(disk.size)}</td>
                  <td>{disk.interfaceType || dash}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
      <section className="spec-card full">
        <h3>File Systems</h3>
        <div className="table-wrap">
          <table>
            <thead>
              <tr>
                <th>Mount</th>
                <th>Type</th>
                <th>Used</th>
                <th>Size</th>
                <th>Use %</th>
              </tr>
            </thead>
            <tbody>
              {(storage.filesystems || []).map((fs: FilesystemEntry, index: number) => (
                <tr key={`${fs.mount || 'fs'}-${index}`}>
                  <td>{fs.mount || dash}</td>
                  <td>{fs.type || dash}</td>
                  <td>{formatBytes(fs.used)}</td>
                  <td>{formatBytes(fs.size)}</td>
                  <td>{formatPercent(fs.use)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );

  const renderSensors = () => (
    <div className="tab-grid">
      <section className="spec-card">
        <h3>Temperatures</h3>
        <Row label="Package" value={temp.main ? `${temp.main} °C` : dash} />
        <Row label="Max Core" value={temp.max ? `${temp.max} °C` : dash} />
      </section>
      <section className="spec-card">
        <h3>Live Metrics</h3>
        <Row label="CPU Load" value={formatPercent(load.currentLoad)} />
        <Row label="Current Clock" value={formatGHz(speed.avg)} />
        <Row label="Memory Used" value={formatBytes(memory.used)} />
        <Row label="Uptime" value={formatSeconds(merged.runtime?.uptime)} />
      </section>
    </div>
  );

  const renderSystem = () => (
    <div className="tab-grid">
      <section className="spec-card">
        <h3>Operating System</h3>
        <Row label="Platform" value={staticData.os?.platform || dash} />
        <Row label="Distro" value={staticData.os?.distro || dash} />
        <Row label="Release" value={staticData.os?.release || dash} />
        <Row label="Hostname" value={staticData.os?.hostname || dash} />
      </section>
      <section className="spec-card">
        <h3>Network</h3>
        <Row label="Interfaces" value={(network.interfaces || []).length} />
        <Row label="Primary MAC" value={staticData.uuid?.macs?.[0] || dash} />
        <Row label="FQDN" value={staticData.os?.fqdn || dash} />
        <Row label="Kernel" value={staticData.os?.kernel || dash} />
      </section>
    </div>
  );

  const renderAbout = () => (
    <section className="spec-card about-card full">
      <h3>About</h3>
      <p>Rolando L. Lanugon - Chief Engineer and Founder DreamArchers, Genviax and Scinettek</p>
      <p className="about-tech">Powered by Rust + Tauri v2</p>
    </section>
  );

  const contentByTab: Record<TabKey, () => JSX.Element> = {
    cpu: renderCpu,
    mainboard: renderMainboard,
    memory: renderMemory,
    spd: renderSpd,
    graphics: renderGraphics,
    storage: renderStorage,
    sensors: renderSensors,
    system: renderSystem,
    about: renderAbout,
  };

  return (
    <div className="dashboard-shell cpuz-shell">
      <header className="cpuz-header">
        <h1>Hardware Info Dashboard</h1>
        <p>CPU-Z style hardware inspector &bull; Powered by Rust &amp; Tauri</p>
      </header>
      <nav className="tab-bar">
        {tabs.map((tab) => (
          <button
            key={tab.key}
            className={activeTab === tab.key ? 'tab active' : 'tab'}
            onClick={() => setActiveTab(tab.key)}
            type="button"
          >
            {tab.label}
          </button>
        ))}
      </nav>
      <main className="tab-panel">{contentByTab[activeTab]()}</main>
    </div>
  );
}

export default App;
