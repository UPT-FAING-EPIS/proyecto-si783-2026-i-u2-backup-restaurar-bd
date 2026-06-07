import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { Play, Terminal, CheckCircle2, XCircle } from 'lucide-react';

interface LogEntry {
  id: number;
  time: string;
  message: string;
  level: string;
}

export function Backup() {
  const [connections, setConnections] = useState<any[]>([]);
  const [selectedConn, setSelectedConn] = useState<string>('');
  const [status, setStatus] = useState<'idle' | 'running' | 'success' | 'error'>('idle');
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [progress, setProgress] = useState(0);
  const logsEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    invoke('list_connections').then((data) => {
      setConnections(data as any[]);
      if ((data as any[]).length > 0) {
        setSelectedConn((data as any[])[0].id);
      }
    });

    const unlisten = listen('backup_log', (event: any) => {
      const payload = event.payload;
      const now = new Date();
      const timeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`;
      
      setLogs(prev => [...prev, {
        id: Date.now() + Math.random(),
        time: timeStr,
        message: payload.message,
        level: payload.level
      }]);
      
      // Simulate progress bar based on logs received (MVP approach)
      setProgress(p => Math.min(p + 15, 95));
    });

    return () => {
      unlisten.then(f => f());
    };
  }, []);

  useEffect(() => {
    // Auto-scroll
    logsEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  const handleBackup = async () => {
    if (!selectedConn) return;
    setStatus('running');
    setLogs([]);
    setProgress(10);
    
    try {
      const result: any = await invoke('generate_backup', { connectionId: selectedConn });
      if (result.verified) {
        setStatus('success');
      } else {
        setStatus('error');
      }
      setProgress(100);
    } catch (e) {
      setStatus('error');
      setLogs(prev => [...prev, {
        id: Date.now(),
        time: new Date().toLocaleTimeString(),
        message: `Error fatal: ${e}`,
        level: 'error'
      }]);
    }
  };

  const getLogColor = (level: string) => {
    switch (level) {
      case 'success': return 'text-success';
      case 'error': return 'text-error';
      default: return 'text-muted hover:text-text transition-colors';
    }
  };

  return (
    <div className="h-full flex flex-col">
      <header className="h-16 flex items-center px-6 border-b border-border flex-none">
        <div>
          <h1 className="text-xl font-mono text-text">Generar Backup</h1>
          <p className="text-sm text-muted">Ejecuta volcados manuales y verifica en tiempo real</p>
        </div>
      </header>

      <main className="flex-1 p-6 flex flex-col gap-6 overflow-hidden">
        {/* Controls */}
        <div className="bg-surface border border-border rounded-lg p-5 flex gap-4 items-end flex-none">
          <div className="flex-1">
            <label className="block text-sm text-muted mb-2">Seleccionar Conexión</label>
            <select 
              disabled={status === 'running'}
              className="w-full bg-bg border border-border rounded px-3 py-2 text-text focus:border-accent outline-none transition-colors"
              value={selectedConn}
              onChange={e => setSelectedConn(e.target.value)}
            >
              {connections.length === 0 && <option value="">No hay conexiones registradas</option>}
              {connections.map(c => (
                <option key={c.id} value={c.id}>{c.name} ({c.engine} - {c.database_name})</option>
              ))}
            </select>
          </div>
          
          <button 
            disabled={!selectedConn || status === 'running'}
            onClick={handleBackup}
            className="bg-accent text-bg hover:bg-accent/90 disabled:opacity-50 disabled:cursor-not-allowed px-6 py-2 rounded flex items-center justify-center gap-2 font-bold transition-colors"
          >
            {status === 'running' ? (
              <><span className="animate-spin text-bg">↻</span> Procesando...</>
            ) : (
              <><Play size={18} /> Iniciar Backup</>
            )}
          </button>
        </div>

        {/* Progress Bar */}
        {(status === 'running' || status === 'success' || status === 'error') && (
          <div className="flex-none">
            <div className="flex justify-between text-sm mb-2">
              <span className="text-muted font-mono">FASE ACTUAL: {status === 'running' ? 'Ejecutando proceso...' : status === 'success' ? 'Completado' : 'Fallido'}</span>
              <span className="text-accent font-mono">{progress}%</span>
            </div>
            <div className="h-2 w-full bg-surface rounded overflow-hidden border border-border">
              <div 
                className={`h-full transition-all duration-500 ease-out ${status === 'error' ? 'bg-error' : status === 'success' ? 'bg-success' : 'bg-accent'}`}
                style={{ width: `${progress}%` }}
              />
            </div>
          </div>
        )}

        {/* Console Logs */}
        <div className="flex-1 bg-bg border border-border rounded-lg flex flex-col overflow-hidden relative">
          <div className="h-10 border-b border-border flex items-center px-4 bg-surface gap-2 flex-none">
            <Terminal size={16} className="text-muted" />
            <span className="text-sm font-mono text-muted">LOG EN TIEMPO REAL</span>
          </div>
          
          <div className="flex-1 overflow-auto p-4 font-mono text-sm leading-relaxed">
            {logs.length === 0 && status === 'idle' ? (
              <div className="h-full flex items-center justify-center text-muted/50 select-none">
                Esperando inicio de operación...
              </div>
            ) : (
              logs.map((log) => (
                <div key={log.id} className="flex gap-3 mb-1">
                  <span className="text-accent/70 select-none shrink-0">[{log.time}]</span>
                  <span className={`break-all ${getLogColor(log.level)}`}>
                    {log.level === 'success' && '✓ '}
                    {log.level === 'error' && '✗ '}
                    {log.message}
                  </span>
                </div>
              ))
            )}
            <div ref={logsEndRef} />
          </div>
        </div>

        {/* Final Result Card */}
        {(status === 'success' || status === 'error') && (
          <div className={`flex-none p-4 rounded-lg border flex items-center gap-3 animate-in slide-in-from-bottom-4 ${
            status === 'success' ? 'bg-success/10 border-success/30 text-success' : 'bg-error/10 border-error/30 text-error'
          }`}>
            {status === 'success' ? <CheckCircle2 size={24} /> : <XCircle size={24} />}
            <div className="font-mono font-bold">
              {status === 'success' ? 'BACKUP VERIFICADO Y GUARDADO CORRECTAMENTE' : 'OPERACIÓN FALLIDA — VERIFICA EL LOG PARA MÁS DETALLES'}
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
