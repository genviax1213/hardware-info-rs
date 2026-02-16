import React from 'react';
import Row from '../components/Row';
import { HardwareInfo } from '../types';

interface MotherboardProps {
  hardware: HardwareInfo;
}

const Motherboard: React.FC<MotherboardProps> = ({ hardware }) => {
  const baseboard = hardware.staticData.baseboard;
  const bios = hardware.staticData.bios;
  const os = hardware.staticData.os;

  return (
    <div className="space-y-6">
      <header className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Motherboard & BIOS</h2>
        <p className="text-gray-400">System board details and firmware version.</p>
      </header>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
            Motherboard
          </h3>
          <div className="space-y-1">
            <Row label="Manufacturer" value={baseboard.manufacturer} />
            <Row label="Model" value={baseboard.model} />
            <Row label="Version" value={baseboard.version} />
            <Row label="Serial Number" value={baseboard.serial} />
          </div>
        </section>

        <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
          <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
            BIOS / Firmware
          </h3>
          <div className="space-y-1">
            <Row label="Vendor" value={bios.vendor} />
            <Row label="Version" value={bios.version} />
            <Row label="Release Date" value={bios.releaseDate} />
            <Row label="UEFI Capable" value={os.uefi ? 'Yes' : 'No'} />
          </div>
        </section>
      </div>

      <section className="bg-gray-900/50 rounded-xl p-6 border border-gray-800 backdrop-blur-sm">
        <h3 className="text-blue-400 font-semibold mb-4 text-sm uppercase tracking-wider">
          System Information
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12">
          <div className="space-y-1">
            <Row label="System Name" value={os.hostname} />
            <Row label="Machine UUID" value={hardware.staticData.uuid.macs[0] || 'N/A'} />
          </div>
          <div className="space-y-1">
            <Row label="Chassis Type" value="Desktop (Static)" />
            <Row label="Secure Boot" value="Unknown" />
          </div>
        </div>
      </section>
    </div>
  );
};

export default Motherboard;
