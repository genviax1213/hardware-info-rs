import React from 'react';

interface RowProps {
  label: string;
  value: string | number;
}

const Row: React.FC<RowProps> = ({ label, value }) => {
  return (
    <div className="flex justify-between py-2 border-b border-gray-800/50 last:border-0 hover:bg-gray-800/20 px-2 transition-colors">
      <span className="text-gray-400 text-sm font-medium">{label}</span>
      <strong className="text-gray-200 text-sm">{value || '—'}</strong>
    </div>
  );
};

export default Row;
