const dash = '—';

export function formatBytes(value?: number) {
  if (value === undefined || value === null || value < 0) return dash;
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  let size = value;
  let idx = 0;
  while (size >= 1024 && idx < units.length - 1) {
    size /= 1024;
    idx += 1;
  }
  return `${size.toFixed(idx > 1 ? 1 : 0)} ${units[idx]}`;
}

export function formatPercent(value?: number) {
  if (value === undefined || value === null || Number.isNaN(value)) return dash;
  return `${value.toFixed(1)}%`;
}

export function formatGHz(value?: number) {
  if (value === undefined || value === null || Number.isNaN(value) || value <= 0) return dash;
  return `${value.toFixed(2)} GHz`;
}

export function formatSeconds(value?: number) {
  if (!value || value < 0) return dash;
  const hrs = Math.floor(value / 3600);
  const mins = Math.floor((value % 3600) / 60);
  return `${hrs}h ${mins}m`;
}
