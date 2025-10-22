import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Play, Trash2, Info, Plus, Loader2 } from 'lucide-react';
import { useWorkflows } from '../contexts/WorkflowContext';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { Modal } from '../components/Modal';

export function Dashboard() {
  const { workflows, loading, deleteWorkflow, loadWorkflows } = useWorkflows();
  const navigate = useNavigate();
  const [deleteModal, setDeleteModal] = useState(null);
  const [deleting, setDeleting] = useState(false);

  const handleDelete = async () => {
    if (!deleteModal) return;
    
    setDeleting(true);
    try {
      await deleteWorkflow(deleteModal);
      setDeleteModal(null);
    } catch (error) {
      console.error('Errore eliminazione:', error);
    } finally {
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
    return date.toLocaleDateString('it-IT', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  return (
    <div className="max-w-7xl mx-auto p-8">
      {/* Header */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          I Miei Workflow
        </h1>
        <p className="text-gray-600">
          Gestisci e esegui i tuoi workflow automatizzati
        </p>
      </div>

      {/* Action Card - Registra Nuovo */}
      <Card className="mb-8 bg-gradient-to-br from-primary-50 to-primary-100 border-primary-200 hover:shadow-md transition-shadow cursor-pointer"
            onClick={() => navigate('/record')}>
        <div className="flex items-center gap-6">
          <div className="w-16 h-16 bg-primary-600 rounded-full flex items-center justify-center">
            <Plus className="w-8 h-8 text-white" />
          </div>
          <div className="flex-1">
            <h2 className="text-2xl font-bold text-gray-900 mb-1">
              Registra Nuovo Workflow
            </h2>
            <p className="text-gray-700">
              Avvia la registrazione per creare un nuovo workflow automatizzato
            </p>
          </div>
          <Button variant="primary" className="px-8 py-3 text-lg">
            Inizia →
          </Button>
        </div>
      </Card>

      {/* Statistics */}
      {workflows.length > 0 && (
        <div className="grid grid-cols-3 gap-6 mb-8">
          <Card>
            <p className="text-sm text-gray-600 mb-1">Totale Workflow</p>
            <p className="text-3xl font-bold text-gray-900">{workflows.length}</p>
          </Card>
          <Card>
            <p className="text-sm text-gray-600 mb-1">Ultimo Creato</p>
            <p className="text-lg font-semibold text-gray-900">
              {workflows[0]?.name || 'N/A'}
            </p>
          </Card>
          <Card>
            <p className="text-sm text-gray-600 mb-1">Azioni Totali</p>
            <p className="text-3xl font-bold text-gray-900">
              {workflows.reduce((sum, w) => sum + w.event_count, 0)}
            </p>
          </Card>
        </div>
      )}

      {/* Workflows List */}
      <div>
        <h2 className="text-xl font-semibold text-gray-900 mb-4">
          Workflow Salvati ({workflows.length})
        </h2>

        {loading ? (
          <Card className="flex items-center justify-center py-12">
            <Loader2 className="w-8 h-8 animate-spin text-primary-600" />
            <p className="ml-3 text-gray-600">Caricamento workflow...</p>
          </Card>
        ) : workflows.length === 0 ? (
          <Card className="text-center py-12">
            <div className="w-20 h-20 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <Plus className="w-10 h-10 text-gray-400" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              Nessun workflow trovato
            </h3>
            <p className="text-gray-600 mb-6">
              Crea il tuo primo workflow per iniziare l'automazione
            </p>
            <Button onClick={() => navigate('/record')} variant="primary">
              <Plus className="w-5 h-5" />
              Registra Primo Workflow
            </Button>
          </Card>
        ) : (
          <div className="grid gap-4">
            {workflows.map((workflow) => (
              <Card key={workflow.name} className="hover:shadow-md transition-shadow">
                <div className="flex items-center gap-4">
                  <div className="flex-1">
                    <h3 className="text-lg font-semibold text-gray-900 mb-1">
                      {workflow.name}
                    </h3>
                    <div className="flex gap-4 text-sm text-gray-600">
                      <span>📊 {workflow.event_count} azioni</span>
                      {workflow.duration_ms && (
                        <span>⏱️ {formatDuration(workflow.duration_ms)}</span>
                      )}
                      <span>📅 {formatDate(workflow.created_at)}</span>
                    </div>
                  </div>
                  <div className="flex gap-2">
                    <Button
                      variant="ghost"
                      onClick={() => navigate(`/workflow/${workflow.name}`)}
                    >
                      <Info className="w-4 h-4" />
                      Dettagli
                    </Button>
                    <Button
                      variant="primary"
                      onClick={() => navigate(`/execute/${workflow.name}`)}
                    >
                      <Play className="w-4 h-4" />
                      Esegui
                    </Button>
                    <Button
                      variant="danger"
                      onClick={() => setDeleteModal(workflow.name)}
                    >
                      <Trash2 className="w-4 h-4" />
                    </Button>
                  </div>
                </div>
              </Card>
            ))}
          </div>
        )}
      </div>

      {/* Delete Modal */}
      <Modal
        isOpen={deleteModal !== null}
        onClose={() => setDeleteModal(null)}
        title="Conferma Eliminazione"
        footer={
          <>
            <Button variant="secondary" onClick={() => setDeleteModal(null)} className="flex-1">
              Annulla
            </Button>
            <Button variant="danger" onClick={handleDelete} loading={deleting} className="flex-1">
              Elimina
            </Button>
          </>
        }
      >
        <p className="text-gray-700">
          Sei sicuro di voler eliminare il workflow <strong>{deleteModal}</strong>?
          Questa azione non può essere annullata.
        </p>
      </Modal>
    </div>
  );
}
