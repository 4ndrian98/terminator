import React, { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route, Navigate, useNavigate, useLocation } from 'react-router-dom';
import { Key, AlertCircle } from 'lucide-react';
import { WorkflowProvider } from './contexts/WorkflowContext';
import { Dashboard } from './pages/Dashboard';
import { RecordWorkflow } from './pages/RecordWorkflow';
import { WorkflowDetails } from './pages/WorkflowDetails';
import { ExecuteWorkflow } from './pages/ExecuteWorkflow';
import { ActivateLicense } from './pages/ActivateLicense';
import { LicenseManagement } from './pages/LicenseManagement';
import { getLicenseStatus } from './services/tauriApi';

function LicenseChecker({ children }) {
  const [license, setLicense] = useState(null);
  const [loading, setLoading] = useState(true);
  const [checkComplete, setCheckComplete] = useState(false);

  useEffect(() => {
    checkLicense();
  }, []);

  const checkLicense = async () => {
    try {
      const licenseData = await getLicenseStatus();
      setLicense(licenseData);
    } catch (err) {
      console.log('Nessuna licenza trovata');
    } finally {
      setLoading(false);
      setCheckComplete(true);
    }
  };

  if (loading || !checkComplete) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <p className="text-gray-600">Caricamento...</p>
      </div>
    );
  }

  // Se non c'è licenza, mostra solo la pagina di attivazione
  if (!license) {
    return <ActivateLicense onActivated={(newLicense) => setLicense(newLicense)} />;
  }

  // Se c'è licenza, mostra l'app normale
  return children;
}

function AppHeader({ license }) {
  const navigate = useNavigate();
  const location = useLocation();

  const getDaysRemaining = (expirationDate) => {
    if (!expirationDate || expirationDate.length !== 8) return 0;
    
    const year = parseInt(expirationDate.substring(0, 4));
    const month = parseInt(expirationDate.substring(4, 6)) - 1;
    const day = parseInt(expirationDate.substring(6, 8));
    
    const expDate = new Date(year, month, day);
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    
    const diffTime = expDate - today;
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    
    return diffDays;
  };

  const daysRemaining = license ? getDaysRemaining(license.expiration_date) : 0;
  const isExpired = daysRemaining < 0;
  const isExpiringSoon = daysRemaining >= 0 && daysRemaining <= 7;

  return (
    <header className="bg-white shadow-sm border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-8 py-4">
        <div className="flex items-center justify-between">
          <div 
            className="flex items-center gap-3 cursor-pointer"
            onClick={() => navigate('/')}
          >
            <div className="w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center">
              <span className="text-white font-bold text-xl">W</span>
            </div>
            <div>
              <h1 className="text-xl font-bold text-gray-900">Workflow Automator</h1>
              <p className="text-xs text-gray-600">Automazione Intelligente</p>
            </div>
          </div>
          
          <div className="flex items-center gap-4">
            {/* License Status */}
            {license && (
              <button
                onClick={() => navigate('/license')}
                className={`flex items-center gap-2 px-3 py-2 rounded-lg transition-colors ${
                  isExpired 
                    ? 'bg-red-100 text-red-700 hover:bg-red-200' 
                    : isExpiringSoon 
                    ? 'bg-yellow-100 text-yellow-700 hover:bg-yellow-200'
                    : 'bg-green-100 text-green-700 hover:bg-green-200'
                }`}
              >
                {isExpired ? (
                  <AlertCircle className="w-4 h-4" />
                ) : (
                  <Key className="w-4 h-4" />
                )}
                <span className="text-sm font-medium">
                  {isExpired 
                    ? 'Licenza Scaduta' 
                    : isExpiringSoon
                    ? `${daysRemaining} giorni`
                    : 'Licenza Attiva'}
                </span>
              </button>
            )}
            <div className="text-sm text-gray-600">
              v1.0.0
            </div>
          </div>
        </div>
      </div>
    </header>
  );
}

function AppContent() {
  const [license, setLicense] = useState(null);

  useEffect(() => {
    loadLicense();
  }, []);

  const loadLicense = async () => {
    try {
      const licenseData = await getLicenseStatus();
      setLicense(licenseData);
    } catch (err) {
      console.log('Nessuna licenza');
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <AppHeader license={license} />
      
      <main>
        <Routes>
          <Route path="/" element={<Dashboard />} />
          <Route path="/record" element={<RecordWorkflow />} />
          <Route path="/workflow/:workflowName" element={<WorkflowDetails />} />
          <Route path="/execute/:workflowName" element={<ExecuteWorkflow />} />
          <Route path="/license" element={<LicenseManagement />} />
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </main>

      <footer className="mt-12 py-6 border-t border-gray-200 bg-white">
        <div className="max-w-7xl mx-auto px-8 text-center text-sm text-gray-600">
          <p>© 2025 Workflow Automator - Powered by Terminator & Tauri</p>
        </div>
      </footer>
    </div>
  );
}

function App() {
  return (
    <WorkflowProvider>
      <BrowserRouter>
        <LicenseChecker>
          <AppContent />
        </LicenseChecker>
      </BrowserRouter>
    </WorkflowProvider>
  );
}

export default App;
