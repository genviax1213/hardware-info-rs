import React from 'react';
import Sidebar, { TabId } from './Sidebar';

interface LayoutProps {
  children: React.ReactNode;
  activeTab: TabId;
  onTabChange: (id: TabId) => void;
}

const Layout: React.FC<LayoutProps> = ({ children, activeTab, onTabChange }) => {
  return (
    <div className="flex bg-gray-950 text-gray-100 min-h-screen">
      <Sidebar activeTab={activeTab} onTabChange={onTabChange} />
      <main className="flex-1 h-screen overflow-y-auto custom-scrollbar">
        <div className="p-8 max-w-6xl mx-auto">
          {children}
        </div>
      </main>
    </div>
  );
};

export default Layout;
