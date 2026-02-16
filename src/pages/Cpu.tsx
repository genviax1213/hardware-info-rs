import React from 'react';
import Row from '../components/Row';
import { formatBytes, formatGHz, formatPercent } from '../utils';
import { HardwareInfo, LiveInfo } from '../types';

interface CpuProps {
  hardware: HardwareInfo;
  live: LiveInfo | null;
}

const Cpu: React.FC<CpuProps> = ({ hardware, live }) => {
  const cpu = hardware.cpu;
  const speed = live?.cpuCurrentSpeed || hardware.cpuCurrentSpeed;
  const load = live?.currentLoad || hardware.currentLoad;
  const temp = live?.cpuTemperature || hardware.cpuTemperature;

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Central Processing Unit (CPU)</h2>
        <p className="text-gray-400">Detailed information about your processor.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Manufacturer</h3>
          <div className="space-y-1">
            <Row label="Name" value={cpu.brand} />
            <Row label="Vendor ID" value={cpu.vendor} />
            <Row label="Family" value={cpu.family} />
            <Row label="Model" value={cpu.model} />
            <Row label="Stepping" value={cpu.stepping} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Core Topology</h3>
          <div className="space-y-1">
            <Row label="Physical Cores" value={cpu.physicalCores} />
            <Row label="Logical Processors" value={cpu.cores} />
            <Row label="L2 Cache" value={formatBytes(cpu.cache.l2)} />
            <Row label="L3 Cache" value={formatBytes(cpu.cache.l3)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Clock Speeds</h3>
          <div className="space-y-1">
            <Row label="Base Frequency" value={formatGHz(cpu.speed)} />
            <Row label="Max Frequency" value={formatGHz(cpu.speedMax)} />
            <Row label="Current Frequency" value={formatGHz(speed.avg)} />
            <Row label="Core Voltage" value="N/A (Linux)" />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4">Live Metrics</h3>
          <div className="space-y-1">
            <Row label="Total CPU Load" value={formatPercent(load.currentLoad)} />
            <Row label="Package Temperature" value={temp.main ? `${temp.main}°C` : 'N/A'} />
            <Row label="Max Core Temp" value={temp.max ? `${temp.max}°C` : 'N/A'} />
          </div>
        </section>
      </div>
    </div>
  );
};

export default Cpu;
