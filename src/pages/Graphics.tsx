import React from 'react';
import Row from '../components/Row';
import { HardwareInfo } from '../types';

interface GraphicsProps {
  hardware: HardwareInfo;
}

const Graphics: React.FC<GraphicsProps> = ({ hardware }) => {
  const controllers = hardware.graphics.controllers || [];

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Graphics (GPU)</h2>
        <p className="text-gray-400">Information about your graphics adapters and displays.</p>
      </header>

      {controllers.length > 0 ? (
        controllers.map((gpu, index) => (
          <section key={index} className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
            <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
              Adapter #{index + 1}: {gpu.model}
            </h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12">
              <div className="space-y-1">
                <Row label="Model" value={gpu.model} />
                <Row label="Vendor" value={gpu.vendor} />
              </div>
              <div className="space-y-1">
                <Row label="VRAM" value={gpu.vram ? `${gpu.vram} MB` : 'Shared'} />
                <Row label="Bus Interface" value={gpu.bus} />
              </div>
            </div>
          </section>
        ))
      ) : (
        <div className="p-12 text-center bg-gray-900/50 rounded-xl border border-dashed border-gray-800 text-gray-500">
          No graphics controllers detected.
        </div>
      )}

      <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
        <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
          Monitor / Display
        </h3>
        <p className="text-gray-500 text-sm italic italic">
          Extended display information coming in next update.
        </p>
      </section>
    </div>
  );
};

export default Graphics;
