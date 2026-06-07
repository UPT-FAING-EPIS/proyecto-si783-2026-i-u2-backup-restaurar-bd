import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Plus, Edit2, Trash2, Database } from 'lucide-react';
import { ConnectionForm } from '../components/ConnectionForm';
import toast from 'react-hot-toast';

export function Connections() {
  const [connections, setConnections] = useState<any[]>([]);
  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingConn, setEditingConn] = useState<any>(null);

  const loadConnections = async () => {
    try {
      const data = await invoke('list_connections');
      setConnections(data as any[]);
    } catch (e) {
      console.error(e);
    }
  };

  useEffect(() => {
    loadConnections();
  }, []);

  const handleEdit = (conn: any) => {
    setEditingConn(conn);
    setIsFormOpen(true);
  };

  const handleDelete = async (id: string) => {
    if (confirm('¿Estás seguro de eliminar esta conexión?')) {
      try {
        await invoke('delete_connection', { id });
        toast.success('Conexión eliminada');
        loadConnections();
      } catch (e) {
        toast.error(`Error: ${e}`);
      }
    }
  };

  const engineColors: Record<string, string> = {
    postgres: 'bg-blue-900/40 text-blue-400 border-blue-800',
    mysql: 'bg-orange-900/40 text-orange-400 border-orange-800',
    sqlserver: 'bg-red-900/40 text-red-400 border-red-800',
    mongodb: 'bg-green-900/40 text-green-400 border-green-800'
  };

  return (
    <div className="h-full flex flex-col">
      <header className="h-16 flex items-center justify-between px-6 border-b border-border">
        <div>
          <h1 className="text-xl font-mono text-text">Conexiones</h1>
          <p className="text-sm text-muted">Gestiona tus credenciales de base de datos</p>
        </div>
        <button 
          onClick={() => { setEditingConn(null); setIsFormOpen(true); }}
          className="bg-accent text-bg hover:bg-accent/90 px-4 py-2 rounded flex items-center gap-2 font-semibold transition-colors"
        >
          <Plus size={18} /> Nueva Conexión
        </button>
      </header>

      <main className="flex-1 p-6 overflow-auto">
        {connections.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-muted">
            <Database size={48} className="mb-4 opacity-50" />
            <h2 className="text-lg mb-2">No hay conexiones registradas</h2>
            <p>Agrega tu primera conexión para empezar a generar backups.</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-4">
            {connections.map(conn => (
              <div key={conn.id} className="bg-surface border border-border rounded-lg p-5 flex flex-col gap-4">
                <div className="flex justify-between items-start">
                  <div>
                    <h3 className="font-semibold text-lg text-text">{conn.name}</h3>
                    <p className="text-sm text-muted font-mono mt-1">{conn.host}:{conn.port}</p>
                  </div>
                  <span className={`text-xs px-2 py-1 rounded border font-mono uppercase ${engineColors[conn.engine] || 'bg-border text-muted'}`}>
                    {conn.engine}
                  </span>
                </div>
                
                <div className="text-sm">
                  <p><span className="text-muted">BD:</span> <span className="text-text">{conn.database_name}</span></p>
                  <p><span className="text-muted">User:</span> <span className="text-text">{conn.username}</span></p>
                </div>

                <div className="flex gap-2 mt-auto pt-4 border-t border-border">
                  <button onClick={() => handleEdit(conn)} className="flex-1 py-1.5 border border-border hover:border-accent text-muted hover:text-accent rounded flex items-center justify-center gap-2 transition-colors">
                    <Edit2 size={16} /> Editar
                  </button>
                  <button onClick={() => handleDelete(conn.id)} className="flex-1 py-1.5 border border-border hover:border-error text-muted hover:text-error rounded flex items-center justify-center gap-2 transition-colors">
                    <Trash2 size={16} /> Eliminar
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </main>

      {isFormOpen && (
        <ConnectionForm 
          connection={editingConn} 
          onClose={() => setIsFormOpen(false)} 
          onSave={() => { setIsFormOpen(false); loadConnections(); }} 
        />
      )}
    </div>
  );
}
