import { Database, LayoutDashboard, History, Settings2, Play } from 'lucide-react';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

export function Sidebar({ currentView, setView }: { currentView: string, setView: (v: string) => void }) {
  const [dockerOk, setDockerOk] = useState<boolean>(false);

  useEffect(() => {
    invoke('check_docker').then((res) => setDockerOk(res as boolean)).catch(() => setDockerOk(false));
  }, []);

  const navItems = [
    { id: 'dashboard', icon: LayoutDashboard, title: 'Dashboard' },
    { id: 'connections', icon: Database, title: 'Conexiones' },
    { id: 'backup', icon: Play, title: 'Generar Backup' },
    { id: 'history', icon: History, title: 'Historial' },
  ];

  return (
    <div className="w-16 flex-none bg-surface border-r border-border flex flex-col items-center py-4">
      <div className="w-8 h-8 rounded bg-accent mb-8 flex items-center justify-center text-bg font-bold font-mono">
        SB
      </div>
      
      <div className="flex-1 flex flex-col gap-4 w-full">
        {navItems.map(item => {
          const Icon = item.icon;
          const isActive = currentView === item.id;
          return (
            <button
              key={item.id}
              onClick={() => setView(item.id)}
              title={item.title}
              className={`w-full aspect-square flex items-center justify-center transition-colors relative
                ${isActive ? 'text-accent' : 'text-muted hover:text-text'}`}
            >
              {isActive && <div className="absolute left-0 top-0 bottom-0 w-1 bg-accent rounded-r" />}
              <Icon size={24} />
            </button>
          );
        })}
      </div>

      <div 
        title={dockerOk ? "Docker está corriendo" : "Docker no está disponible"}
        className="mt-auto pt-4 flex flex-col items-center justify-center"
      >
        <Settings2 size={20} className="text-muted mb-4" />
        <div className={`w-3 h-3 rounded-full ${dockerOk ? 'bg-success animate-pulse' : 'bg-muted'}`} />
      </div>
    </div>
  );
}
