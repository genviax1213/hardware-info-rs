import React from 'react';
import { 
  Info, 
  Monitor, 
  Cpu, 
  Database, 
  Layers, 
  HardDrive, 
  Music, 
  MousePointer2, 
  Network,
  Activity
} from 'lucide-react';

export type TabId = 
  | 'summary' 
  | 'os' 
  | 'cpu' 
  | 'ram' 
  | 'motherboard' 
  | 'graphics' 
  | 'storage' 
  | 'audio' 
  | 'peripherals' 
  | 'network';

interface Tab {
  id: TabId;
  label: string;
  icon: React.ElementType;
}

const tabs: Tab[] = [
  { id: 'summary', label: 'Summary', icon: Activity },
  { id: 'os', label: 'Operating System', icon: Monitor },
  { id: 'cpu', label: 'CPU', icon: Cpu },
  { id: 'ram', label: 'RAM', icon: Database },
  { id: 'motherboard', label: 'Motherboard', icon: Layers },
  { id: 'graphics', label: 'Graphics', icon: Monitor },
  { id: 'storage', label: 'Storage', icon: HardDrive },
  { id: 'audio', label: 'Audio', icon: Music },
  { id: 'peripherals', label: 'Peripherals', icon: MousePointer2 },
  { id: 'network', label: 'Network', icon: Network },
];

interface SidebarProps {
  activeTab: TabId;
  onTabChange: (id: TabId) => void;
}

const Sidebar: React.FC<SidebarProps> = ({ activeTab, onTabChange }) => {
  return (
    <div className="w-64 bg-gray-900 text-gray-300 h-screen overflow-y-auto border-r border-gray-800 flex flex-col">
      <div className="p-6 border-b border-gray-800">
        <h1 className="text-xl font-bold text-white flex items-center gap-2">
          <Info className="w-6 h-6 text-blue-500" />
          Hardware Info
        </h1>
      </div>
      <nav className="flex-1 py-4">
        {tabs.map((tab) => {
          const Icon = tab.icon;
          const isActive = activeTab === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => onTabChange(tab.id)}
              className={`w-full flex items-center gap-3 px-6 py-3 transition-colors ${
                isActive 
                  ? 'bg-blue-600/10 text-blue-400 border-r-2 border-blue-500' 
                  : 'hover:bg-gray-800 hover:text-white'
              }`}
            >
              <Icon size={18} />
              <span className="font-medium">{tab.label}</span>
            </button>
          );
        })}
      </nav>
      <div className="p-4 border-t border-gray-800 text-xs text-gray-500 text-center">
        v0.1.4
      </div>
    </div>
  );
};

export default Sidebar;
