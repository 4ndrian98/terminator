import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { WorkflowProvider } from './contexts/WorkflowContext';
import { Dashboard } from './pages/Dashboard';
import { RecordWorkflow } from './pages/RecordWorkflow';
import { WorkflowDetails } from './pages/WorkflowDetails';
import { ExecuteWorkflow } from './pages/ExecuteWorkflow';

function App() {
  return (
    <WorkflowProvider>
      <BrowserRouter>
        <div className="min-h-screen bg-gray-50">
          {/* Header */}
          <header className="bg-white shadow-sm border-b border-gray-200">
            <div className="max-w-7xl mx-auto px-8 py-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-700 rounded-lg flex items-center justify-center">
                    <span className="text-white font-bold text-xl">W</span>
                  </div>
                  <div>
                    <h1 className="text-xl font-bold text-gray-900">Workflow Automator</h1>
                    <p className="text-xs text-gray-600">Automazione Intelligente</p>
                  </div>
                </div>
                <div className="text-sm text-gray-600">
                  v1.0.0
                </div>
              </div>
            </div>
          </header>

          {/* Main Content */}
          <main>
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/record" element={<RecordWorkflow />} />
              <Route path="/workflow/:workflowName" element={<WorkflowDetails />} />
              <Route path="/execute/:workflowName" element={<ExecuteWorkflow />} />
              <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
          </main>

          {/* Footer */}
          <footer className="mt-12 py-6 border-t border-gray-200 bg-white">
            <div className="max-w-7xl mx-auto px-8 text-center text-sm text-gray-600">
              <p>© 2025 Workflow Automator - Powered by Terminator & Tauri</p>
            </div>
          </footer>
        </div>
      </BrowserRouter>
    </WorkflowProvider>
  );
}

export default App;
