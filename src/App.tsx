import { useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';
import type { HardwareInfo, LiveInfo } from './types';

// Components
import Layout from './components/Layout';
import { TabId } from './components/Sidebar';

// Pages
import Summary from './pages/Summary';
import Cpu from './pages/Cpu';
import Ram from './pages/Ram';
import Motherboard from './pages/Motherboard';
import Graphics from './pages/Graphics';
import Storage from './pages/Storage';
import OperatingSystem from './pages/OperatingSystem';
import Network from './pages/Network';
import Audio from './pages/Audio';
import Peripherals from './pages/Peripherals';

function App() {
  const [hardwareInfo, setHardwareInfo] = useState<HardwareInfo | null>(null);
  const [liveInfo, setLiveInfo] = useState<LiveInfo | null>(null);
  const [activeTab, setActiveTab] = useState<TabId>('summary');
  const [error, setError] = useState<string>('');

  useEffect(() => {
    let timer: ReturnType<typeof setInterval>;

    const fetchInitial = async () => {
      try {
        const data = await invoke<HardwareInfo>('get_hardware_info');
        setHardwareInfo(data);
        setError('');
      } catch (fetchError) {
        setError('Unable to load hardware data.');
        console.error(fetchError);
      }
    };

    const fetchLive = async () => {
      try {
        const data = await invoke<LiveInfo>('get_hardware_live');
        setLiveInfo(data);
      } catch (fetchError) {
        console.error(fetchError);
      }
    };

    fetchInitial().then(fetchLive);
    timer = setInterval(fetchLive, 3000);
    return () => clearInterval(timer);
  }, []);

  const mergedHardware: HardwareInfo | null = useMemo(() => {
    if (!hardwareInfo) return null;
    return {
      ...hardwareInfo,
      memory: {
        ...hardwareInfo.memory,
        ...(liveInfo?.memory || {}),
        layout: hardwareInfo.memory.layout, // Keep static layout
      }
    };
  }, [hardwareInfo, liveInfo]);

  if (!mergedHardware) {
    return (
      <div className="loading">
        <div className="flex flex-col items-center gap-4">
          <div className="w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
          <p>{error || 'Initializing System Inspector...'}</p>
        </div>
      </div>
    );
  }

  const renderContent = () => {
    switch (activeTab) {
      case 'summary':
        return <Summary hardware={mergedHardware} live={liveInfo} />;
      case 'os':
        return <OperatingSystem hardware={mergedHardware} live={liveInfo} />;
      case 'cpu':
        return <Cpu hardware={mergedHardware} live={liveInfo} />;
      case 'ram':
        return <Ram hardware={mergedHardware} live={liveInfo} />;
      case 'motherboard':
        return <Motherboard hardware={mergedHardware} />;
      case 'graphics':
        return <Graphics hardware={mergedHardware} />;
      case 'storage':
        return <Storage hardware={mergedHardware} />;
      case 'network':
        return <Network hardware={mergedHardware} />;
      case 'audio':
        return <Audio hardware={mergedHardware} />;
      case 'peripherals':
        return <Peripherals hardware={mergedHardware} />;
      default:
        return <Summary hardware={mergedHardware} live={liveInfo} />;
    }
  };

  return (
    <Layout activeTab={activeTab} onTabChange={setActiveTab}>
      {renderContent()}
    </Layout>
  );
}

export default App;
