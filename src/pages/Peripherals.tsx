import React from 'react';

import { HardwareInfo } from '../types';

interface PeripheralsProps {
  hardware: HardwareInfo;
}

const Peripherals: React.FC<PeripheralsProps> = ({ hardware }) => {
  const devices = hardware.peripherals.usbDevices || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Peripherals (USB)</h2>
        <p className="text-gray-400">USB devices, input devices, and external hardware detected via lsusb.</p>
      </header>

      {devices.length > 0 ? (
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <div className="overflow-x-auto">
            <table className="w-full text-left">
              <thead>
                <tr className="border-b border-gray-800 bg-gray-800/30">
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Bus/Dev</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">ID</th>
                  <th className="px-4 py-3 text-xs font-bold text-gray-400 uppercase tracking-wider">Device Name</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-800">
                {devices.map((usb, idx) => (
                  <tr key={idx} className="hover:bg-gray-800/20">
                    <td className="px-4 py-3 text-sm text-gray-400 font-mono">
                      {usb.bus}:{usb.device}
                    </td>
                    <td className="px-4 py-3 text-sm text-blue-400 font-mono">
                      {usb.vendorId}:{usb.productId}
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-200">
                      {usb.name}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      ) : (
        <div className="p-12 text-center bg-gray-900/50 rounded-xl border border-dashed border-gray-800 text-gray-500">
          No USB devices detected (lsusb).
        </div>
      )}
    </div>
  );
};

export default Peripherals;
