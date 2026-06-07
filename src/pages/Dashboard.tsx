import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Database, ShieldCheck, ShieldAlert, HardDrive, Activity, CheckCircle2, XCircle } from 'lucide-react';

export function Dashboard() {
  const [stats, setStats] = useState<any>(null);

  useEffect(() => {
    invoke('get_dashboard_stats').then(data => setStats(data)).catch(console.error);
  }, []);

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  const getEngineColor = (engine: string) => {
    switch(engine) {
      case 'postgres': return 'text-blue-400 bg-blue-400/10 border-blue-400/20';
      case 'mysql': return 'text-orange-400 bg-orange-400/10 border-orange-400/20';
      case 'sqlserver': return 'text-red-400 bg-red-400/10 border-red-400/20';
      case 'mongodb': return 'text-green-400 bg-green-400/10 border-green-400/20';
      default: return 'text-muted bg-muted/10 border-muted/20';
    }
  };

  return (
    <div className="h-full flex flex-col relative overflow-auto">
      <header className="h-16 flex items-center justify-between px-6 border-b border-border flex-none">
        <div>
          <h1 className="text-xl font-mono text-text">Dashboard</h1>
          <p className="text-sm text-muted">Vista general del estado de tus backups</p>
        </div>
      </header>

      <main className="flex-1 p-6 space-y-6">
        {/* KPI Cards */}
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
          <div className="bg-surface border border-border rounded-lg p-5 flex items-center gap-4">
            <div className="w-12 h-12 rounded bg-accent/10 flex items-center justify-center text-accent">
              <Database size={24} />
            </div>
            <div>
              <div className="text-sm text-muted">Conexiones</div>
              <div className="text-2xl font-mono font-bold">{stats?.total_connections || 0}</div>
            </div>
          </div>

          <div className="bg-surface border border-border rounded-lg p-5 flex items-center gap-4">
            <div className="w-12 h-12 rounded bg-success/10 flex items-center justify-center text-success">
              <ShieldCheck size={24} />
            </div>
            <div>
              <div className="text-sm text-muted">Backups Exitosos</div>
              <div className="text-2xl font-mono font-bold text-success">{stats?.successful_backups || 0}</div>
            </div>
          </div>

          <div className="bg-surface border border-border rounded-lg p-5 flex items-center gap-4">
            <div className="w-12 h-12 rounded bg-error/10 flex items-center justify-center text-error">
              <ShieldAlert size={24} />
            </div>
            <div>
              <div className="text-sm text-muted">Backups Fallidos</div>
              <div className="text-2xl font-mono font-bold text-error">{stats?.failed_backups || 0}</div>
            </div>
          </div>

          <div className="bg-surface border border-border rounded-lg p-5 flex items-center gap-4">
            <div className="w-12 h-12 rounded bg-blue-500/10 flex items-center justify-center text-blue-500">
              <HardDrive size={24} />
            </div>
            <div>
              <div className="text-sm text-muted">Datos Resguardados</div>
              <div className="text-2xl font-mono font-bold">{stats ? formatSize(stats.total_bytes_backed_up) : '0 B'}</div>
            </div>
          </div>
        </div>

        {/* Actividad Reciente */}
        <div className="bg-surface border border-border rounded-lg overflow-hidden">
          <div className="px-5 py-4 border-b border-border flex items-center gap-2">
            <Activity size={18} className="text-accent" />
            <h2 className="font-semibold">Actividad Reciente</h2>
          </div>
          
          {(!stats?.recent_activity || stats.recent_activity.length === 0) ? (
            <div className="p-8 text-center text-muted">No hay actividad reciente. Ejecuta tu primer backup.</div>
          ) : (
            <div className="divide-y divide-border">
              {stats.recent_activity.map((log: any) => (
                <div key={log.id} className="p-4 flex items-center justify-between hover:bg-bg/50 transition-colors">
                  <div className="flex items-center gap-4">
                    {log.status === 'OK' ? (
                      <CheckCircle2 size={24} className="text-success shrink-0" />
                    ) : (
                      <XCircle size={24} className="text-error shrink-0" />
                    )}
                    <div>
                      <div className="font-medium">{log.connection_name}</div>
                      <div className="text-sm text-muted flex gap-3 mt-1">
                        <span className="font-mono">{log.started_at}</span>
                        <span>{formatSize(log.file_size_bytes)}</span>
                      </div>
                    </div>
                  </div>
                  <span className={`text-xs px-2 py-1 rounded border uppercase font-mono ${getEngineColor(log.engine)}`}>
                    {log.engine}
                  </span>
                </div>
              ))}
            </div>
          )}
        </div>
      </main>
    </div>
  );
}
