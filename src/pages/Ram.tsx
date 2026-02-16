import React from 'react';
import Row from '../components/Row';
import { formatBytes } from '../utils';
import { HardwareInfo, LiveInfo } from '../types';

interface RamProps {
  hardware: HardwareInfo;
  live: LiveInfo | null;
}

const Ram: React.FC<RamProps> = ({ hardware, live }) => {
  const mem = live?.memory || hardware.memory;
  const layout = hardware.memory.layout || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Memory (RAM)</h2>
        <p className="text-gray-400">Total capacity, current usage, and physical layout.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
            Total Memory Usage
          </h3>
          <div className="space-y-1">
            <Row label="Total Physical" value={formatBytes(mem.total)} />
            <Row label="Used Physical" value={formatBytes(mem.used)} />
            <Row label="Available Physical" value={formatBytes(mem.available)} />
            <Row label="Active" value={formatBytes(mem.active)} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
            Virtual Memory (Swap)
          </h3>
          <div className="space-y-1">
            <Row label="Total Swap" value={formatBytes(mem.swaptotal)} />
            <Row label="Used Swap" value={formatBytes(mem.swapused)} />
            <Row label="Free Swap" value={formatBytes(mem.swapfree)} />
          </div>
        </section>
      </div>

      <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
        <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
          Physical Slots (SPD)
        </h3>
        {layout.length > 0 ? (
          <div className="overflow-x-auto">
            <table className="w-full text-left">
              <thead>
                <tr className="border-b border-gray-800 bg-gray-800/30">
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Slot</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Size</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Type</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Speed</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Manufacturer</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Part Number</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-800">
                {layout.map((slot) => (
                  <tr key={slot.slot} className="hover:bg-gray-800/20">
                    <td className="px-4 py-3 text-sm text-gray-200">#{slot.slot}</td>
                    <td className="px-4 py-3 text-sm text-gray-200">{formatBytes(slot.size)}</td>
                    <td className="px-4 py-3 text-sm text-gray-200">{slot.type}</td>
                    <td className="px-4 py-3 text-sm text-gray-200">{slot.clockSpeed} MHz</td>
                    <td className="px-4 py-3 text-sm text-gray-200">{slot.manufacturer}</td>
                    <td className="px-4 py-3 text-sm text-gray-200 font-mono text-xs">{slot.partNum}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="p-4 text-center text-gray-500 italic">
            No slot information available (requires root/dmidecode)
          </div>
        )}
      </section>
    </div>
  );
};

export default Ram;
