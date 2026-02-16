import React from 'react';
import Row from '../components/Row';
import { formatSeconds } from '../utils';
import { HardwareInfo, LiveInfo } from '../types';

interface OperatingSystemProps {
  hardware: HardwareInfo;
  live: LiveInfo | null;
}

const OperatingSystem: React.FC<OperatingSystemProps> = ({ hardware, live }) => {
  const os = hardware.staticData.os;
  const runtime = live?.runtime || hardware.runtime;

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Operating System</h2>
        <p className="text-gray-400">Software environment and kernel details.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
            OS Information
          </h3>
          <div className="space-y-1">
            <Row label="Distribution" value={os.distro} />
            <Row label="Kernel Version" value={os.kernel} />
            <Row label="Architecture" value={os.arch} />
            <Row label="Hostname" value={os.hostname} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
            Status & Features
          </h3>
          <div className="space-y-1">
            <Row label="Uptime" value={formatSeconds(runtime.uptime)} />
            <Row label="UEFI Mode" value={os.uefi ? 'Enabled' : 'Disabled'} />
            <Row label="Platform" value={os.platform} />
            <Row label="FQDN" value={os.fqdn} />
          </div>
        </section>
      </div>

      <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
        <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
          Environment
        </h3>
        <p className="text-gray-500 text-sm">
          Current Date/Time: <span className="text-gray-200 font-mono text-xs">{new Date(runtime.current * 1000).toLocaleString()}</span>
        </p>
      </section>
    </div>
  );
};

export default OperatingSystem;
