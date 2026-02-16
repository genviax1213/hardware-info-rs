import React from 'react';

import { formatBytes, formatPercent } from '../utils';
import { HardwareInfo } from '../types';

interface StorageProps {
  hardware: HardwareInfo;
}

const Storage: React.FC<StorageProps> = ({ hardware }) => {
  const layout = hardware.storage.diskLayout || [];
  const filesystems = hardware.storage.filesystems || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Storage (Drives)</h2>
        <p className="text-gray-400">Physical disks and mounted file systems.</p>
      </header>

      <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
        <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
          Physical Disk Layout
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-left">
            <thead>
              <tr className="border-b border-gray-800 bg-gray-800/30">
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Device</th>
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Type</th>
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Interface</th>
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider text-right">Size</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-800">
              {layout.map((disk, idx) => (
                <tr key={idx} className="hover:bg-gray-800/20">
                  <td className="px-4 py-3 text-sm text-gray-200">{disk.name}</td>
                  <td className="px-4 py-3 text-sm text-gray-200 uppercase">{disk.type}</td>
                  <td className="px-4 py-3 text-sm text-gray-200">{disk.interfaceType || 'SATA/NVMe'}</td>
                  <td className="px-4 py-3 text-sm text-gray-200 text-right font-mono">{formatBytes(disk.size)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
        <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
          File Systems (Mount Points)
        </h3>
        <div className="overflow-x-auto">
          <table className="w-full text-left">
            <thead>
              <tr className="border-b border-gray-800 bg-gray-800/30">
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Mount</th>
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Type</th>
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Usage</th>
                <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider text-right">Free / Total</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-800">
              {filesystems.map((fs, idx) => (
                <tr key={idx} className="hover:bg-gray-800/20">
                  <td className="px-4 py-3 text-sm text-gray-200 font-mono">{fs.mount}</td>
                  <td className="px-4 py-3 text-sm text-gray-200">{fs.type}</td>
                  <td className="px-4 py-3 text-sm text-gray-200">
                    <div className="flex items-center gap-3">
                      <div className="flex-1 bg-gray-800 h-1.5 rounded-full overflow-hidden min-w-[60px]">
                        <div 
                          className={`h-full rounded-full ${fs.use > 90 ? 'bg-red-500' : 'bg-blue-500'}`}
                          style={{ width: `${fs.use}%` }}
                        />
                      </div>
                      <span className="text-xs font-bold">{formatPercent(fs.use)}</span>
                    </div>
                  </td>
                  <td className="px-4 py-3 text-sm text-gray-200 text-right font-mono text-xs">
                    {formatBytes(fs.size - fs.used)} / {formatBytes(fs.size)}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
};

export default Storage;
