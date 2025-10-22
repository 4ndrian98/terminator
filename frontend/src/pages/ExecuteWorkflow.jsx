import React, { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ArrowLeft, Play, Loader2, CheckCircle } from 'lucide-react';
import { useWorkflows } from '../contexts/WorkflowContext';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { Input } from '../components/Input';

export function ExecuteWorkflow() {
  const { workflowName } = useParams();
  const navigate = useNavigate();
  const { executeWorkflow } = useWorkflows();
  const [repetitions, setRepetitions] = useState(1);
  const [speed, setSpeed] = useState(1.0);
  const [executing, setExecuting] = useState(false);
  const [completed, setCompleted] = useState(false);
  const [error, setError] = useState('');

  const handleExecute = async () => {
    setExecuting(true);
    setError('');
    
    try {
      await executeWorkflow(workflowName, repetitions, speed);
      setCompleted(true);
      setTimeout(() => {
        navigate('/');
      }, 2000);
    } catch (err) {
      setError(err.message || 'Errore durante l\'esecuzione');
      setExecuting(false);
    }
  };

  return (
    <div className="max-w-4xl mx-auto p-8">
      <div className="mb-8">
        <Button onClick={() => navigate('/')} variant="ghost" className="mb-4">
          <ArrowLeft className="w-4 h-4" />
          Torna alla Dashboard
        </Button>
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Esegui Workflow
        </h1>
        <p className="text-gray-600">
          Configura ed esegui <strong>{workflowName}</strong>
        </p>
      </div>

      {!executing && !completed && (
        <Card className="mb-8">
          <h2 className="text-xl font-semibold text-gray-900 mb-6">
            Configurazione Esecuzione
          </h2>
          
          <div className="space-y-6">
            {/* Repetitions */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Numero di Ripetizioni
              </label>
              <input
                type="number"
                min="1"
                max="100"
                value={repetitions}
                onChange={(e) => setRepetitions(parseInt(e.target.value) || 1)}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-primary-500"
              />
              <p className="mt-1 text-sm text-gray-600">
                Il workflow verrà eseguito {repetitions} {repetitions === 1 ? 'volta' : 'volte'}
              </p>
            </div>

            {/* Speed */}
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Velocità di Esecuzione: {speed}x
              </label>
              <input
                type="range"
                min="0.25"
                max="2.0"
                step="0.25"
                value={speed}
                onChange={(e) => setSpeed(parseFloat(e.target.value))}
                className="w-full"
              />
              <div className="flex justify-between text-xs text-gray-600 mt-1">
                <span>0.25x (Lento)</span>
                <span>1.0x (Normale)</span>
                <span>2.0x (Veloce)</span>
              </div>
            </div>

            {error && (
              <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
                <p className="text-red-700">{error}</p>
              </div>
            )}

            <Button
              onClick={handleExecute}
              variant="primary"
              className="w-full py-4 text-lg"
            >
              <Play className="w-5 h-5" />
              Avvia Esecuzione
            </Button>
          </div>
        </Card>
      )}

      {executing && !completed && (
        <Card className="text-center py-12">
          <Loader2 className="w-16 h-16 animate-spin text-primary-600 mx-auto mb-6" />
          <h2 className="text-2xl font-bold text-gray-900 mb-2">
            Esecuzione in Corso...
          </h2>
          <p className="text-gray-600 mb-4">
            Il workflow <strong>{workflowName}</strong> è in esecuzione
          </p>
          <p className="text-sm text-gray-500">
            Ripetizioni: {repetitions} | Velocità: {speed}x
          </p>
        </Card>
      )}

      {completed && (
        <Card className="text-center py-12">
          <div className="w-24 h-24 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-6">
            <CheckCircle className="w-12 h-12 text-green-600" />
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">
            ✅ Esecuzione Completata!
          </h2>
          <p className="text-gray-600 mb-4">
            Il workflow è stato eseguito con successo
          </p>
          <p className="text-sm text-gray-500">
            Reindirizzamento alla dashboard...
          </p>
        </Card>
      )}

      {/* Warning Box */}
      {!executing && !completed && (
        <Card className="bg-yellow-50 border-yellow-200">
          <h3 className="text-lg font-semibold text-gray-900 mb-3">
            ⚠️ Attenzione
          </h3>
          <ul className="space-y-2 text-gray-700">
            <li>• Assicurati che le finestre siano nella stessa posizione della registrazione</li>
            <li>• Non muovere il mouse durante l'esecuzione</li>
            <li>• Il workflow eseguirà le azioni automaticamente</li>
            <li>• Puoi premere ESC per interrompere (feature futura)</li>
          </ul>
        </Card>
      )}
    </div>
  );
}
