import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Circle, Square, Loader2, CheckCircle } from 'lucide-react';
import { useWorkflows } from '../contexts/WorkflowContext';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { Input } from '../components/Input';
import { Modal } from '../components/Modal';

export function RecordWorkflow() {
  const { startRecording, stopRecording, saveWorkflow, isRecording } = useWorkflows();
  const navigate = useNavigate();
  const [status, setStatus] = useState('idle'); // idle, recording, stopped, saving, saved
  const [workflowName, setWorkflowName] = useState('');
  const [tempWorkflowName, setTempWorkflowName] = useState('');
  const [elapsedTime, setElapsedTime] = useState(0);
  const [showSaveModal, setShowSaveModal] = useState(false);
  const [error, setError] = useState('');

  // Timer per tempo di registrazione
  useEffect(() => {
    let interval;
    if (status === 'recording') {
      interval = setInterval(() => {
        setElapsedTime(prev => prev + 1);
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [status]);

  const handleStartRecording = async () => {
    if (!tempWorkflowName.trim()) {
      setError('Inserisci un nome per il workflow');
      return;
    }

    setError('');
    setWorkflowName(tempWorkflowName);
    setStatus('recording');
    setElapsedTime(0);

    try {
      await startRecording(tempWorkflowName);
    } catch (err) {
      setStatus('idle');
      setError(err.message || 'Errore avvio registrazione');
    }
  };

  const handleStopRecording = async () => {
    setStatus('stopped');
    try {
      await stopRecording();
      setShowSaveModal(true);
    } catch (err) {
      setError(err.message || 'Errore arresto registrazione');
      setStatus('recording');
    }
  };

  const handleSave = async () => {
    setStatus('saving');
    try {
      await saveWorkflow(workflowName);
      setStatus('saved');
      setTimeout(() => {
        navigate('/');
      }, 1500);
    } catch (err) {
      setError(err.message || 'Errore salvataggio workflow');
      setStatus('stopped');
      setShowSaveModal(false);
    }
  };

  const formatTime = (seconds) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  return (
    <div className="max-w-4xl mx-auto p-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Registra Nuovo Workflow
        </h1>
        <p className="text-gray-600">
          Registra le tue azioni per creare un workflow automatizzato
        </p>
      </div>

      {/* Status Display */}
      <Card className="mb-8">
        {status === 'idle' && (
          <div className="text-center py-12">
            <div className="w-24 h-24 bg-primary-100 rounded-full flex items-center justify-center mx-auto mb-6">
              <Circle className="w-12 h-12 text-primary-600" />
            </div>
            <h2 className="text-2xl font-bold text-gray-900 mb-4">
              Pronto per Registrare
            </h2>
            <p className="text-gray-600 mb-8 max-w-md mx-auto">
              Inserisci un nome per il workflow e premi "Avvia Registrazione".
              Tutte le tue azioni verranno registrate.
            </p>
            <div className="max-w-md mx-auto mb-6">
              <Input
                label="Nome Workflow"
                placeholder="es: inserimento_ordini"
                value={tempWorkflowName}
                onChange={(e) => setTempWorkflowName(e.target.value)}
                error={error}
              />
            </div>
            <Button
              onClick={handleStartRecording}
              variant="primary"
              className="px-8 py-3 text-lg"
            >
              <Circle className="w-5 h-5" />
              Avvia Registrazione
            </Button>
          </div>
        )}

        {status === 'recording' && (
          <div className="text-center py-12">
            <div className="w-24 h-24 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-6 animate-pulse">
              <Circle className="w-12 h-12 text-red-600 fill-red-600" />
            </div>
            <h2 className="text-2xl font-bold text-gray-900 mb-2">
              🎬 Registrazione in Corso...
            </h2>
            <p className="text-xl font-mono text-red-600 mb-4">
              {formatTime(elapsedTime)}
            </p>
            <p className="text-gray-600 mb-8 max-w-md mx-auto">
              Esegui le azioni che vuoi automatizzare.
              Ogni click, digitazione e azione viene registrata.
            </p>
            <Button
              onClick={handleStopRecording}
              variant="danger"
              className="px-8 py-3 text-lg"
            >
              <Square className="w-5 h-5" />
              Ferma Registrazione
            </Button>
          </div>
        )}

        {status === 'stopped' && (
          <div className="text-center py-12">
            <div className="w-24 h-24 bg-yellow-100 rounded-full flex items-center justify-center mx-auto mb-6">
              <Square className="w-12 h-12 text-yellow-600" />
            </div>
            <h2 className="text-2xl font-bold text-gray-900 mb-2">
              Registrazione Fermata
            </h2>
            <p className="text-xl font-mono text-gray-900 mb-4">
              Durata: {formatTime(elapsedTime)}
            </p>
            <p className="text-gray-600 max-w-md mx-auto">
              Salvataggio del workflow in corso...
            </p>
          </div>
        )}

        {status === 'saving' && (
          <div className="text-center py-12">
            <Loader2 className="w-16 h-16 animate-spin text-primary-600 mx-auto mb-6" />
            <h2 className="text-2xl font-bold text-gray-900 mb-2">
              Salvataggio in corso...
            </h2>
            <p className="text-gray-600">
              Attendere prego
            </p>
          </div>
        )}

        {status === 'saved' && (
          <div className="text-center py-12">
            <div className="w-24 h-24 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-6">
              <CheckCircle className="w-12 h-12 text-green-600" />
            </div>
            <h2 className="text-2xl font-bold text-gray-900 mb-2">
              ✅ Workflow Salvato!
            </h2>
            <p className="text-gray-600">
              Reindirizzamento alla dashboard...
            </p>
          </div>
        )}
      </Card>

      {/* Instructions */}
      {status === 'idle' && (
        <Card className="bg-blue-50 border-blue-200">
          <h3 className="text-lg font-semibold text-gray-900 mb-3">
            💡 Consigli per una Buona Registrazione
          </h3>
          <ul className="space-y-2 text-gray-700">
            <li>✓ Esegui le azioni <strong>lentamente</strong> e con precisione</li>
            <li>✓ Aspetta che i programmi si carichino completamente</li>
            <li>✓ Evita movimenti inutili del mouse</li>
            <li>✓ Il workflow registrerà ESATTAMENTE quello che fai</li>
          </ul>
        </Card>
      )}

      {/* Save Modal */}
      <Modal
        isOpen={showSaveModal}
        onClose={() => {}}
        title="Salvataggio Workflow"
        footer={
          <Button variant="primary" onClick={handleSave} className="w-full" loading={status === 'saving'}>
            Salva Workflow
          </Button>
        }
      >
        <div className="text-center">
          <p className="text-gray-700 mb-4">
            Il workflow <strong>{workflowName}</strong> è stato registrato con successo.
          </p>
          <p className="text-gray-600">
            Durata: {formatTime(elapsedTime)}
          </p>
        </div>
      </Modal>
    </div>
  );
}
