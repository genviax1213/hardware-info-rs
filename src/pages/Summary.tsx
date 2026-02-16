import React from 'react';
import Row from '../components/Row';
import { formatBytes, formatGHz, formatSeconds } from '../utils';
import { HardwareInfo, LiveInfo } from '../types';
import { ShieldCheck, Activity, Cpu, Database, Monitor } from 'lucide-react';

interface SummaryProps {
  hardware: HardwareInfo;
  live: LiveInfo | null;
}

const Summary: React.FC<SummaryProps> = ({ hardware, live }) => {
  const os = hardware.staticData.os;
  const cpu = hardware.cpu;
  const mem = live?.memory || hardware.memory;
  const gpu = hardware.graphics.controllers[0];

  return (
    <div className="space-y-6">
      <header className="mb-8 flex justify-between items-start">
        <div>
          <h2 className="text-2xl font-bold text-white mb-2">System Summary</h2>
          <p className="text-gray-400">Overview of your hardware and operating system.</p>
        </div>
        <div className="flex items-center gap-2 bg-green-500/10 text-green-400 px-4 py-2 rounded-lg border border-green-500/20">
          <ShieldCheck size={18} />
          <span className="font-semibold text-sm">System Health: Good</span>
        </div>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm relative overflow-hidden group">
          <div className="absolute top-0 left-0 w-1 h-full bg-blue-500/50" />
          <h3 className="text-blue-400 font-semibold mb-4 flex items-center gap-2">
            <Monitor size={18} />
            Operating System
          </h3>
          <div className="space-y-1">
            <Row label="Distribution" value={os.distro} />
            <Row label="Version" value={os.release} />
            <Row label="Kernel" value={os.kernel} />
            <Row label="Uptime" value={formatSeconds(live?.runtime.uptime || hardware.runtime.uptime)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm relative overflow-hidden group">
          <div className="absolute top-0 left-0 w-1 h-full bg-purple-500/50" />
          <h3 className="text-blue-400 font-semibold mb-4 flex items-center gap-2">
            <Cpu size={18} />
            Processor
          </h3>
          <div className="space-y-1">
            <Row label="Model" value={cpu.brand} />
            <Row label="Cores / Threads" value={`${cpu.physicalCores} / ${cpu.cores}`} />
            <Row label="Base Speed" value={formatGHz(cpu.speed)} />
            <Row label="Max Speed" value={formatGHz(cpu.speedMax)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm relative overflow-hidden group">
          <div className="absolute top-0 left-0 w-1 h-full bg-emerald-500/50" />
          <h3 className="text-blue-400 font-semibold mb-4 flex items-center gap-2">
            <Database size={18} />
            Memory
          </h3>
          <div className="space-y-1">
            <Row label="Total RAM" value={formatBytes(mem.total)} />
            <Row label="Used" value={formatBytes(mem.used)} />
            <Row label="Available" value={formatBytes(mem.available)} />
            <Row label="Active" value={formatBytes(mem.active)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm relative overflow-hidden group">
          <div className="absolute top-0 left-0 w-1 h-full bg-orange-500/50" />
          <h3 className="text-blue-400 font-semibold mb-4 flex items-center gap-2">
            <Activity size={18} />
            Graphics
          </h3>
          <div className="space-y-1">
            <Row label="GPU" value={gpu?.model || 'Generic Graphics'} />
            <Row label="Vendor" value={gpu?.vendor || 'Unknown'} />
            <Row label="VRAM" value={gpu?.vram ? `${gpu.vram} MB` : 'Shared'} />
          </div>
        </section>
      </div>
    </div>
  );
};

export default Summary;
