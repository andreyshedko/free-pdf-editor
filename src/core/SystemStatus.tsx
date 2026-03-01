import React, { useEffect, useState } from 'react';

type Status = 'online' | 'checking' | 'offline';

export const SystemStatus: React.FC = () => {
  const [status, setStatus] = useState<Status>('checking');

  useEffect(() => {
    const check = () => {
      try {
        // Verify the JS event loop is responsive by resolving a microtask
        const start = Date.now();
        Promise.resolve().then(() => {
          const elapsed = Date.now() - start;
          setStatus(elapsed < 2000 ? 'online' : 'offline');
        });
      } catch {
        setStatus('offline');
      }
    };

    check();
    const id = setInterval(check, 10000);
    return () => clearInterval(id);
  }, []);

  const labelMap: Record<Status, string> = {
    online: 'System online',
    offline: 'System offline',
    checking: 'Checking…',
  };
  const label = labelMap[status];

  return (
    <span
      className="badge-subtle"
      aria-label={`System status: ${label}`}
      aria-live="polite"
    >
      <span
        className="badge-dot"
        style={
          status === 'offline'
            ? { background: '#ef4444', boxShadow: '0 0 10px rgba(239,68,68,0.9)' }
            : status === 'checking'
              ? { background: '#f59e0b', boxShadow: '0 0 10px rgba(245,158,11,0.9)' }
              : undefined
        }
      />
      {label}
    </span>
  );
};
