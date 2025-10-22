import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ArrowLeft, Play, Trash2, Loader2, Calendar, Clock, Activity } from 'lucide-react';
import { useWorkflows } from '../contexts/WorkflowContext';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { Modal } from '../components/Modal';

export function WorkflowDetails() {
  const { workflowName } = useParams();
  const navigate = useNavigate();
  const { getWorkflowInfo, deleteWorkflow } = useWorkflows();
  const [workflowData, setWorkflowData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [deleteModal, setDeleteModal] = useState(false);
  const [deleting, setDeleting] = useState(false);

  useEffect(() => {
    loadWorkflowData();
  }, [workflowName]);

  const loadWorkflowData = async () => {
    setLoading(true);
    try {
      const data = await getWorkflowInfo(workflowName);
      setWorkflowData(data);
    } catch (error) {
      console.error('Errore caricamento workflow:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async () => {
    setDeleting(true);
    try {
      await deleteWorkflow(workflowName);
      navigate('/');
    } catch (error) {
      console.error('Errore eliminazione:', error);
      setDeleting(false);
    }
  };

  const formatDuration = (ms) => {
    const seconds = Math.floor(ms / 1000);
    const minutes = Math.floor(seconds / 60);
    const remainingSeconds = seconds % 60;
    return minutes > 0 ? `${minutes}m ${remainingSeconds}s` : `${seconds}s`;
  };

  const formatDate = (timestamp) => {
    if (!timestamp) return 'N/A';
    const date = new Date(timestamp);
    return date.toLocaleString('it-IT');
  };

  const getEventTypeCounts = () => {
    if (!workflowData?.events) return {};
    
    const counts = {
      mouse: 0,
      keyboard: 0,
      hotkey: 0,
      text: 0,
      appSwitch: 0,
      other: 0,
    };

    workflowData.events.forEach(({ event }) => {
      if (event.Mouse) counts.mouse++;
      else if (event.Keyboard) counts.keyboard++;
      else if (event.Hotkey) counts.hotkey++;
      else if (event.TextInputCompleted) counts.text++;
      else if (event.ApplicationSwitch) counts.appSwitch++;
      else counts.other++;
    });

    return counts;
  };

  if (loading) {
    return (
      <div className="max-w-7xl mx-auto p-8">
        <Card className="flex items-center justify-center py-12">
          <Loader2 className="w-8 h-8 animate-spin text-primary-600" />
          <p className="ml-3 text-gray-600">Caricamento workflow...</p>
        </Card>
      </div>
    );
  }

  if (!workflowData) {
    return (
      <div className="max-w-7xl mx-auto p-8">
        <Card className="text-center py-12">
          <h2 className="text-xl font-semibold text-gray-900 mb-2">Workflow non trovato</h2>
          <Button onClick={() => navigate('/')} variant="primary" className="mt-4">
            <ArrowLeft className="w-4 h-4" />
            Torna alla Dashboard
          </Button>
        </Card>
      </div>
    );
  }

  const eventCounts = getEventTypeCounts();
  const duration = workflowData.end_time && workflowData.start_time
    ? workflowData.end_time - workflowData.start_time
    : null;

  return (
    <div className="max-w-7xl mx-auto p-8">
      {/* Header */}
      <div className="mb-8">
        <Button onClick={() => navigate('/')} variant="ghost" className="mb-4">
          <ArrowLeft className="w-4 h-4" />
          Torna alla Dashboard
        </Button>
        <div className="flex items-start justify-between">
          <div>
            <h1 className="text-3xl font-bold text-gray-900 mb-2">
              {workflowData.name}
            </h1>
            <p className="text-gray-600">
              Dettagli e statistiche del workflow
            </p>
          </div>
          <div className="flex gap-3">
            <Button
              onClick={() => navigate(`/execute/${workflowName}`)}
              variant="primary"
            >
              <Play className="w-4 h-4" />
              Esegui Workflow
            </Button>
            <Button
              onClick={() => setDeleteModal(true)}
              variant="danger"
            >
              <Trash2 className="w-4 h-4" />
              Elimina
            </Button>
          </div>
        </div>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-3 gap-6 mb-8">
        <Card>
          <div className="flex items-center gap-3 mb-2">
            <Activity className="w-5 h-5 text-primary-600" />
            <p className="text-sm text-gray-600">Azioni Totali</p>
          </div>
          <p className="text-3xl font-bold text-gray-900">
            {workflowData.events?.length || 0}
          </p>
        </Card>

        <Card>
          <div className="flex items-center gap-3 mb-2">
            <Clock className="w-5 h-5 text-primary-600" />
            <p className="text-sm text-gray-600">Durata</p>
          </div>
          <p className="text-3xl font-bold text-gray-900">
            {duration ? formatDuration(duration) : 'N/A'}
          </p>
        </Card>

        <Card>
          <div className="flex items-center gap-3 mb-2">
            <Calendar className="w-5 h-5 text-primary-600" />
            <p className="text-sm text-gray-600">Creato il</p>
          </div>
          <p className="text-lg font-semibold text-gray-900">
            {formatDate(workflowData.start_time)}
          </p>
        </Card>
      </div>

      {/* Event Breakdown */}
      <Card className="mb-8">
        <h2 className="text-xl font-semibold text-gray-900 mb-6">Riepilogo Azioni</h2>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
          <div className="p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600 mb-1">🖊️ Click Mouse</p>
            <p className="text-2xl font-bold text-gray-900">{eventCounts.mouse}</p>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600 mb-1">⌨️ Tastiera</p>
            <p className="text-2xl font-bold text-gray-900">{eventCounts.keyboard}</p>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600 mb-1">🔥 Hotkey</p>
            <p className="text-2xl font-bold text-gray-900">{eventCounts.hotkey}</p>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600 mb-1">📝 Input Testo</p>
            <p className="text-2xl font-bold text-gray-900">{eventCounts.text}</p>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600 mb-1">🔄 Cambi App</p>
            <p className="text-2xl font-bold text-gray-900">{eventCounts.appSwitch}</p>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <p className="text-sm text-gray-600 mb-1">🔹 Altro</p>
            <p className="text-2xl font-bold text-gray-900">{eventCounts.other}</p>
          </div>
        </div>
      </Card>

      {/* Info Box */}
      <Card className="bg-blue-50 border-blue-200">
        <h3 className="text-lg font-semibold text-gray-900 mb-3">
          💡 Come Usare Questo Workflow
        </h3>
        <ol className="space-y-2 text-gray-700 list-decimal list-inside">
          <li>Clicca su <strong>"Esegui Workflow"</strong> per avviarlo</li>
          <li>Scegli quante volte vuoi ripeterlo</li>
          <li>Regola la velocità di esecuzione se necessario</li>
          <li>Il workflow eseguirà automaticamente tutte le azioni registrate</li>
        </ol>
      </Card>

      {/* Delete Modal */}
      <Modal
        isOpen={deleteModal}
        onClose={() => setDeleteModal(false)}
        title="Conferma Eliminazione"
        footer={
          <>
            <Button variant="secondary" onClick={() => setDeleteModal(false)} className="flex-1">
              Annulla
            </Button>
            <Button variant="danger" onClick={handleDelete} loading={deleting} className="flex-1">
              Elimina
            </Button>
          </>
        }
      >
        <p className="text-gray-700">
          Sei sicuro di voler eliminare il workflow <strong>{workflowName}</strong>?
          Questa azione non può essere annullata.
        </p>
      </Modal>
    </div>
  );
}
