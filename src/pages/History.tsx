import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Search, AlertCircle, FileText, Calendar, CheckCircle2, XCircle } from 'lucide-react';

export function History() {
  const [logs, setLogs] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedLog, setSelectedLog] = useState<any | null>(null);
  const [showLogs, setShowLogs] = useState(false);

  const [engineFilter, setEngineFilter] = useState('all');
  const [statusFilter, setStatusFilter] = useState('all');
  const [search, setSearch] = useState('');

  const loadLogs = async () => {
    setLoading(true);
    try {
      const data = await invoke('list_logs', { engine: engineFilter, status: statusFilter });
      setLogs(data as any[]);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  };

  useEffect(() => {
    loadLogs();
  }, [engineFilter, statusFilter]);

  const filteredLogs = logs.filter(log => 
    log.connection_name.toLowerCase().includes(search.toLowerCase())
  );

  const getEngineColor = (engine: string) => {
    switch(engine) {
      case 'postgres': return 'text-blue-400 bg-blue-400/10 border-blue-400/20';
      case 'mysql': return 'text-orange-400 bg-orange-400/10 border-orange-400/20';
      case 'sqlserver': return 'text-red-400 bg-red-400/10 border-red-400/20';
      case 'mongodb': return 'text-green-400 bg-green-400/10 border-green-400/20';
      default: return 'text-muted bg-muted/10 border-muted/20';
    }
  };

  const formatSize = (bytes: number) => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  };

  return (
    <div className="h-full flex flex-col relative overflow-hidden">
      <header className="flex flex-col sm:flex-row items-start sm:items-center justify-between px-6 py-4 border-b border-border flex-none gap-4">
        <div className="min-w-0">
          <h1 className="text-xl font-mono text-text truncate">Historial de Logs</h1>
          <p className="text-sm text-muted truncate">Registro completo de operaciones de backup y validación</p>
        </div>
        
        <div className="flex flex-wrap items-center gap-3 w-full sm:w-auto">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-muted" size={16} />
            <input 
              type="text" 
              placeholder="Buscar conexión..."
              className="pl-9 pr-4 py-1.5 bg-surface border border-border rounded text-sm focus:border-accent outline-none text-text"
              value={search}
              onChange={e => setSearch(e.target.value)}
            />
          </div>
          
          <select 
            className="bg-surface border border-border rounded px-3 py-1.5 text-sm text-text outline-none"
            value={engineFilter}
            onChange={e => setEngineFilter(e.target.value)}
          >
            <option value="all">Todos los Motores</option>
            <option value="postgres">PostgreSQL</option>
            <option value="mysql">MySQL</option>
            <option value="sqlserver">SQL Server</option>
            <option value="mongodb">MongoDB</option>
          </select>
          
          <select 
            className="bg-surface border border-border rounded px-3 py-1.5 text-sm text-text outline-none"
            value={statusFilter}
            onChange={e => setStatusFilter(e.target.value)}
          >
            <option value="all">Todos los Estados</option>
            <option value="OK">Exitosos (OK)</option>
            <option value="FAIL">Fallidos (FAIL)</option>
          </select>
        </div>
      </header>

      <main className="flex-1 overflow-auto p-6">
        <div className="bg-surface border border-border rounded-lg overflow-x-auto">
          <table className="w-full text-left border-collapse min-w-[800px]">
            <thead>
              <tr className="border-b border-border bg-bg/50">
                <th className="px-4 py-3 text-sm font-semibold text-muted">Conexión</th>
                <th className="px-4 py-3 text-sm font-semibold text-muted">Motor</th>
                <th className="px-4 py-3 text-sm font-semibold text-muted">Fecha</th>
                <th className="px-4 py-3 text-sm font-semibold text-muted">Duración</th>
                <th className="px-4 py-3 text-sm font-semibold text-muted">Tamaño</th>
                <th className="px-4 py-3 text-sm font-semibold text-muted">Estado</th>
              </tr>
            </thead>
            <tbody>
              {loading ? (
                <tr>
                  <td colSpan={6} className="text-center py-8 text-muted">Cargando historial...</td>
                </tr>
              ) : filteredLogs.length === 0 ? (
                <tr>
                  <td colSpan={6} className="text-center py-12 text-muted">
                    <div className="flex flex-col items-center">
                      <FileText size={48} className="mb-4 opacity-50" />
                      <p>No se encontraron registros</p>
                    </div>
                  </td>
                </tr>
              ) : (
                filteredLogs.map(log => (
                  <tr 
                    key={log.id} 
                    onClick={() => setSelectedLog(log)}
                    className="border-b border-border hover:bg-bg/50 cursor-pointer transition-colors"
                  >
                    <td className="px-4 py-3 font-medium text-text">{log.connection_name}</td>
                    <td className="px-4 py-3">
                      <span className={`text-xs px-2 py-1 rounded border uppercase font-mono ${getEngineColor(log.engine)}`}>
                        {log.engine}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-sm text-muted">{log.started_at}</td>
                    <td className="px-4 py-3 text-sm font-mono text-muted">{log.duration_seconds}s</td>
                    <td className="px-4 py-3 text-sm font-mono text-muted">{formatSize(log.file_size_bytes)}</td>
                    <td className="px-4 py-3">
                      {log.status === 'OK' ? (
                        <span className="flex items-center gap-1 text-success bg-success/10 px-2 py-1 rounded text-xs border border-success/20 w-fit">
                          <CheckCircle2 size={14} /> OK
                        </span>
                      ) : (
                        <span className="flex items-center gap-1 text-error bg-error/10 px-2 py-1 rounded text-xs border border-error/20 w-fit">
                          <XCircle size={14} /> FAIL
                        </span>
                      )}
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </main>

      {/* Modal de Detalles */}
      {selectedLog && (
        <div className="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4">
          <div className="bg-surface border border-border rounded-lg shadow-xl w-full max-w-2xl flex flex-col max-h-[90vh]">
            <div className="p-4 border-b border-border flex justify-between items-center">
              <h2 className="font-mono text-lg font-bold flex items-center gap-2">
                <FileText size={20} className="text-accent" /> Detalles del Log
              </h2>
              <button onClick={() => { setSelectedLog(null); setShowLogs(false); }} className="text-muted hover:text-text"><XCircle size={20} /></button>
            </div>
            
            <div className="p-6 overflow-auto space-y-6">
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">ID Operación</label>
                  <div className="font-mono text-sm">{selectedLog.id}</div>
                </div>
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Estado General</label>
                  <div className={`font-mono text-sm font-bold ${selectedLog.status === 'OK' ? 'text-success' : 'text-error'}`}>
                    {selectedLog.status}
                  </div>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Conexión</label>
                  <div className="text-sm">{selectedLog.connection_name}</div>
                </div>
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Motor</label>
                  <div className="text-sm uppercase font-mono">{selectedLog.engine}</div>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Inicio</label>
                  <div className="text-sm font-mono flex items-center gap-2"><Calendar size={14}/> {selectedLog.started_at}</div>
                </div>
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Fin</label>
                  <div className="text-sm font-mono flex items-center gap-2"><Calendar size={14}/> {selectedLog.finished_at}</div>
                </div>
              </div>

              <div>
                <label className="block text-xs text-muted uppercase tracking-wider mb-1">Ruta del Archivo Generado</label>
                <div className="bg-bg p-2 rounded border border-border font-mono text-xs break-all">
                  {selectedLog.file_path}
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Tamaño en disco</label>
                  <div className="font-mono text-sm">{formatSize(selectedLog.file_size_bytes)} ({selectedLog.file_size_bytes} bytes)</div>
                </div>
                <div>
                  <label className="block text-xs text-muted uppercase tracking-wider mb-1">Verificación Docker</label>
                  <div className="font-mono text-sm">{selectedLog.restore_verified ? 'Exitosa' : 'Fallida / No realizada'}</div>
                </div>
              </div>

              {selectedLog.error_message && (
                <div>
                  <label className="block text-xs text-error uppercase tracking-wider mb-1 flex items-center gap-1">
                    <AlertCircle size={14} /> Mensaje de Error
                  </label>
                  <div className="bg-error/10 border border-error/20 text-error p-3 rounded font-mono text-xs whitespace-pre-wrap">
                    {selectedLog.error_message}
                  </div>
                </div>
              )}

              {/* Logs en tiempo real */}
              {selectedLog.full_logs && (
                <div>
                  <button 
                    onClick={() => setShowLogs(!showLogs)} 
                    className="flex items-center gap-2 text-sm text-accent hover:underline mb-2 font-medium"
                  >
                    <FileText size={16} /> 
                    {showLogs ? "Ocultar Logs de Ejecución" : "Mostrar Logs de Ejecución"}
                  </button>
                  
                  {showLogs && (
                    <div className="bg-[#0f111a] p-4 rounded-lg border border-border/50 max-h-64 overflow-y-auto overflow-x-auto">
                      <pre className="text-gray-300 font-mono text-xs whitespace-pre-wrap">
                        {selectedLog.full_logs}
                      </pre>
                    </div>
                  )}
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
