import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { X, Folder, Shield, ShieldCheck, Database, Zap } from 'lucide-react';
import toast from 'react-hot-toast';

export function ConnectionForm({ 
  connection, 
  onClose, 
  onSave 
}: { 
  connection?: any, 
  onClose: () => void, 
  onSave: () => void 
}) {
  const [formData, setFormData] = useState({
    id: connection?.id || null,
    name: connection?.name || '',
    engine: connection?.engine || 'postgres',
    host: connection?.host || 'localhost',
    port: connection?.port || 5432,
    username: connection?.username || 'root',
    password: '',
    database_name: connection?.database_name || '',
    backup_path: connection?.backup_path || '',
  });

  const [testResult, setTestResult] = useState<'idle' | 'testing' | 'ok' | 'fail'>('idle');
  const [testMessage, setTestMessage] = useState('');

  const handleTest = async () => {
    setTestResult('testing');
    try {
      await invoke('test_connection', { 
        host: formData.host, 
        port: Number(formData.port) 
      });
      setTestResult('ok');
      setTestMessage('Conexión TCP exitosa');
    } catch (e) {
      setTestResult('fail');
      setTestMessage(String(e));
    }
  };

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Validaciones T-061
    if (!formData.name.trim()) return toast.error('El nombre es obligatorio');
    if (!formData.host.trim()) return toast.error('El host es obligatorio');
    if (!formData.username.trim()) return toast.error('El usuario es obligatorio');
    if (!formData.database_name.trim()) return toast.error('El nombre de la BD es obligatorio');
    if (!formData.backup_path.trim()) return toast.error('Debes seleccionar una carpeta de backup');
    if (formData.port <= 0 || formData.port > 65535) return toast.error('Puerto inválido');

    try {
      const payload = { ...formData, port: Number(formData.port) };
      if (formData.id) {
        await invoke('update_connection', { id: formData.id, conn: payload });
        toast.success('Conexión actualizada exitosamente');
      } else {
        await invoke('create_connection', { conn: payload });
        toast.success('Conexión creada exitosamente');
      }
      onSave();
    } catch (e) {
      toast.error(`Error al guardar: ${e}`);
    }
  };

  const selectFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      setFormData({ ...formData, backup_path: selected as string });
    }
  };

  const handleEngineChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newEngine = e.target.value;
    let defaultPort = 5432;
    if (newEngine === 'mysql') defaultPort = 3306;
    else if (newEngine === 'sqlserver') defaultPort = 1433;
    else if (newEngine === 'mongodb') defaultPort = 27017;

    setFormData({
      ...formData,
      engine: newEngine,
      port: defaultPort
    });
  };

  return (
    <div className="fixed inset-0 bg-black/50 z-50 flex justify-end">
      <div className="w-[450px] bg-surface h-full border-l border-border flex flex-col shadow-2xl animate-in slide-in-from-right">
        <div className="h-16 flex items-center justify-between px-6 border-b border-border">
          <h2 className="text-lg font-mono font-bold">{formData.id ? 'Editar Conexión' : 'Nueva Conexión'}</h2>
          <button onClick={onClose} className="text-muted hover:text-text"><X size={20} /></button>
        </div>

        <form onSubmit={handleSave} className="flex-1 overflow-auto p-6 space-y-4">
          <div>
            <label className="block text-sm text-muted mb-1">Nombre amigable *</label>
            <input required type="text" className="w-full bg-bg border border-border rounded px-3 py-2 text-text"
              value={formData.name} onChange={e => setFormData({...formData, name: e.target.value})} />
          </div>

          <div>
            <label className="block text-sm text-muted mb-1">Motor *</label>
            <select className="w-full bg-bg border border-border rounded px-3 py-2 text-text outline-none focus:border-accent transition-colors"
              value={formData.engine} onChange={handleEngineChange}>
              <option value="postgres">PostgreSQL</option>
              <option value="mysql">MySQL</option>
              <option value="sqlserver">SQL Server</option>
              <option value="mongodb">MongoDB</option>
            </select>
          </div>

          <div className="flex gap-4">
            <div className="flex-[2]">
              <label className="block text-sm text-muted mb-1">Host *</label>
              <input required type="text" className="w-full bg-bg border border-border rounded px-3 py-2 text-text"
                value={formData.host} onChange={e => setFormData({...formData, host: e.target.value})} />
            </div>
            <div className="flex-1">
              <label className="block text-sm text-muted mb-1">Puerto *</label>
              <input required type="number" className="w-full bg-bg border border-border rounded px-3 py-2 text-text"
                value={formData.port} onChange={e => setFormData({...formData, port: Number(e.target.value)})} />
            </div>
          </div>

          <div className="flex gap-4">
            <div className="flex-1">
              <label className="block text-sm text-muted mb-1">Usuario *</label>
              <input required type="text" className="w-full bg-bg border border-border rounded px-3 py-2 text-text"
                value={formData.username} onChange={e => setFormData({...formData, username: e.target.value})} />
            </div>
            <div className="flex-1">
              <label className="block text-sm text-muted mb-1">
                Contraseña {formData.id ? '(Dejar en blanco para no cambiar)' : '*'}
              </label>
              <input type="password" required={!formData.id} className="w-full bg-bg border border-border rounded px-3 py-2 text-text"
                value={formData.password} onChange={e => setFormData({...formData, password: e.target.value})} />
            </div>
          </div>

          <div>
            <label className="block text-sm text-muted mb-1">Nombre de la Base de Datos *</label>
            <input required type="text" className="w-full bg-bg border border-border rounded px-3 py-2 text-text"
              value={formData.database_name} onChange={e => setFormData({...formData, database_name: e.target.value})} />
          </div>

          <div>
            <label className="block text-sm text-muted mb-1">Carpeta de Backups *</label>
            <div className="flex gap-2">
              <input required type="text" className="flex-1 bg-bg border border-border rounded px-3 py-2 text-text text-sm"
                value={formData.backup_path} onChange={e => setFormData({...formData, backup_path: e.target.value})} placeholder="Selecciona o escribe una ruta..." />
              <button type="button" onClick={selectFolder} className="bg-border hover:bg-muted/30 px-3 rounded flex items-center justify-center">
                <Folder size={18} />
              </button>
            </div>
          </div>

          <div className="pt-4 border-t border-border mt-6">
            <button type="button" onClick={handleTest} disabled={testResult === 'testing'}
              className="w-full bg-bg border border-border hover:border-accent text-text py-2 rounded flex items-center justify-center gap-2 mb-2 transition-colors">
              {testResult === 'testing' ? <Zap size={18} className="animate-pulse text-accent" /> : <Database size={18} />}
              Probar Conexión (Ping TCP)
            </button>
            
            {testResult === 'ok' && (
              <div className="text-success text-sm flex items-center gap-2 bg-success/10 p-2 rounded">
                <ShieldCheck size={16} /> {testMessage}
              </div>
            )}
            {testResult === 'fail' && (
              <div className="text-error text-sm flex items-center gap-2 bg-error/10 p-2 rounded">
                <Shield size={16} /> {testMessage}
              </div>
            )}
          </div>
        </form>

        <div className="p-6 border-t border-border flex gap-4">
          <button onClick={onClose} className="flex-1 py-2 text-muted hover:text-text">Cancelar</button>
          <button onClick={handleSave} className="flex-1 bg-accent text-bg font-semibold rounded hover:bg-accent/90 transition-colors">Guardar</button>
        </div>
      </div>
    </div>
  );
}
