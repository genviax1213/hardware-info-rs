import React from 'react';
import Row from '../components/Row';
import { formatBytes, formatGHz, formatSeconds } from '../utils';
import { HardwareInfo, LiveInfo } from '../types';

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
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">System Summary</h2>
        <p className="text-gray-400">Overview of your hardware and operating system.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 flex items-center gap-2">
            Operating System
          </h3>
          <div className="space-y-1">
            <Row label="Windows Edition" value={os.distro} />
            <Row label="Version" value={os.release} />
            <Row label="Build" value={os.kernel} />
            <Row label="Uptime" value={formatSeconds(live?.runtime.uptime || hardware.runtime.uptime)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Processor</h3>
          <div className="space-y-1">
            <Row label="Model" value={cpu.brand} />
            <Row label="Cores / Threads" value={`${cpu.physicalCores} / ${cpu.cores}`} />
            <Row label="Total Speed" value={formatGHz(cpu.speedMax)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Memory</h3>
          <div className="space-y-1">
            <Row label="Total RAM" value={formatBytes(mem.total)} />
            <Row label="Used" value={formatBytes(mem.used)} />
            <Row label="Available" value={formatBytes(mem.available)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Graphics</h3>
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
