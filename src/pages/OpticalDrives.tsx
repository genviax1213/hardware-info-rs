import React from 'react';
import Row from '../components/Row';
import { HardwareInfo } from '../types';

interface OpticalDrivesProps {
  hardware: HardwareInfo;
}

const OpticalDrives: React.FC<OpticalDrivesProps> = ({ hardware }) => {
  const devices = hardware.optical.devices || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Optical Drives</h2>
        <p className="text-gray-400">CD, DVD, and Blu-ray drives detected on your system.</p>
      </header>

      {devices.length > 0 ? (
        devices.map((dev, idx) => (
          <section key={idx} className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
            <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
              Drive: {dev.name}
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12">
              <div className="space-y-1">
                <Row label="Device Path" value={`/dev/${dev.name}`} />
                <Row label="Model" value={dev.model} />
              </div>
              <div className="space-y-1">
                <Row label="Vendor" value={dev.vendor} />
                <Row label="Status" value="Ready" />
              </div>
            </div>
          </section>
        ))
      ) : (
        <div className="p-12 text-center bg-gray-900/50 rounded-xl border border-dashed border-gray-800 text-gray-500">
          No optical drives detected on this machine.
        </div>
      )}
    </div>
  );
};

export default OpticalDrives;
