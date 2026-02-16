import React from 'react';
import Row from '../components/Row';
import { HardwareInfo } from '../types';

interface AudioProps {
  hardware: HardwareInfo;
}

const Audio: React.FC<AudioProps> = ({ hardware }) => {
  const devices = hardware.audio.devices || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Audio Devices</h2>
        <p className="text-gray-400">Sound cards, speakers, and microphones detected on Linux.</p>
      </header>

      {devices.length > 0 ? (
        devices.map((dev, idx) => (
          <section key={idx} className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
            <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
              Sound Card #{idx}: {dev.name}
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12">
              <div className="space-y-1">
                <Row label="Device Name" value={dev.name} />
                <Row label="Manufacturer" value={dev.manufacturer} />
              </div>
              <div className="space-y-1">
                <Row label="Status" value={dev.status} />
              </div>
            </div>
          </section>
        ))
      ) : (
        <div className="p-12 text-center bg-gray-900/50 rounded-xl border border-dashed border-gray-800 text-gray-500">
          No audio devices detected (aplay -l).
        </div>
      )}
    </div>
  );
};

export default Audio;
