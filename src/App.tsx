import { useState } from 'react';
import { Sidebar } from './components/Sidebar';
import { Dashboard } from './pages/Dashboard';
import { Connections } from './pages/Connections';
import { Backup } from './pages/Backup';
import { History } from './pages/History';
import { Toaster } from 'react-hot-toast';
import "./App.css";

function App() {
  const [currentView, setCurrentView] = useState('dashboard');

  return (
    <div className="flex h-screen w-full bg-bg text-text overflow-hidden">
      <Toaster position="bottom-right" toastOptions={{
        style: {
          background: '#1F2937', // bg-surface
          color: '#E5E7EB', // text-text
          border: '1px solid #374151', // border-border
        }
      }} />
      <Sidebar currentView={currentView} setView={setCurrentView} />
      
      <div className="flex-1 flex flex-col relative">
        {currentView === 'dashboard' && <Dashboard />}
        {currentView === 'connections' && <Connections />}
        {currentView === 'backup' && <Backup />}
        {currentView === 'history' && <History />}
      </div>
    </div>
  );
}

export default App;
