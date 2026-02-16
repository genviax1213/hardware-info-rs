import React from 'react';
import Row from '../components/Row';
import { HardwareInfo } from '../types';

interface NetworkProps {
  hardware: HardwareInfo;
}

const Network: React.FC<NetworkProps> = ({ hardware }) => {
  const interfaces = hardware.network.interfaces || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Network Interfaces</h2>
        <p className="text-gray-400">Ethernet, Wireless, and other network connections.</p>
      </header>

      {interfaces.length > 0 ? (
        interfaces.map((iface, idx) => (
          <section key={idx} className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
            <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
              {iface.iface}
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12">
              <div className="space-y-1">
                <Row label="IPv4 Address" value={iface.ip4} />
                <Row label="IPv6 Address" value={iface.ip6} />
              </div>
              <div className="space-y-1">
                <Row label="MAC Address" value={iface.mac} />
                <Row label="Status" value="Active" />
              </div>
            </div>
          </section>
        ))
      ) : (
        <div className="p-12 text-center bg-gray-900/50 rounded-xl border border-dashed border-gray-800 text-gray-500">
          No network interfaces detected.
        </div>
      )}
    </div>
  );
};

export default Network;
