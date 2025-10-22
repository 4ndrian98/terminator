import React, { createContext, useContext, useState, useEffect } from 'react';
import * as tauriApi from '../services/tauriApi';

const WorkflowContext = createContext();

export function useWorkflows() {
  const context = useContext(WorkflowContext);
  if (!context) {
    throw new Error('useWorkflows must be used within WorkflowProvider');
  }
  return context;
}

export function WorkflowProvider({ children }) {
  const [workflows, setWorkflows] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [isRecording, setIsRecording] = useState(false);

  // Carica i workflow all'avvio
  useEffect(() => {
    loadWorkflows();
    checkRecordingStatus();
  }, []);

  const loadWorkflows = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await tauriApi.listWorkflows();
      setWorkflows(result);
    } catch (err) {
      setError(err.message || 'Errore nel caricamento dei workflow');
      console.error('Errore caricamento workflow:', err);
    } finally {
      setLoading(false);
    }
  };

  const checkRecordingStatus = async () => {
    try {
      const recording = await tauriApi.isRecording();
      setIsRecording(recording);
    } catch (err) {
      console.error('Errore verifica stato registrazione:', err);
    }
  };

  const startRecording = async (workflowName) => {
    try {
      await tauriApi.startRecording(workflowName);
      setIsRecording(true);
      return true;
    } catch (err) {
      setError(err.message || 'Errore avvio registrazione');
      throw err;
    }
  };

  const stopRecording = async () => {
    try {
      await tauriApi.stopRecording();
      setIsRecording(false);
      return true;
    } catch (err) {
      setError(err.message || 'Errore arresto registrazione');
      throw err;
    }
  };

  const saveWorkflow = async (fileName) => {
    try {
      const result = await tauriApi.saveWorkflow(fileName);
      await loadWorkflows(); // Ricarica la lista
      return result;
    } catch (err) {
      setError(err.message || 'Errore salvataggio workflow');
      throw err;
    }
  };

  const deleteWorkflow = async (workflowName) => {
    try {
      await tauriApi.deleteWorkflow(workflowName);
      await loadWorkflows(); // Ricarica la lista
      return true;
    } catch (err) {
      setError(err.message || 'Errore eliminazione workflow');
      throw err;
    }
  };

  const executeWorkflow = async (workflowName, repetitions = 1, speed = 1.0) => {
    try {
      const result = await tauriApi.executeWorkflow(workflowName, repetitions, speed);
      return result;
    } catch (err) {
      setError(err.message || 'Errore esecuzione workflow');
      throw err;
    }
  };

  const getWorkflowInfo = async (workflowName) => {
    try {
      const info = await tauriApi.getWorkflowInfo(workflowName);
      return info;
    } catch (err) {
      setError(err.message || 'Errore caricamento info workflow');
      throw err;
    }
  };

  const value = {
    workflows,
    loading,
    error,
    isRecording,
    loadWorkflows,
    startRecording,
    stopRecording,
    saveWorkflow,
    deleteWorkflow,
    executeWorkflow,
    getWorkflowInfo,
    clearError: () => setError(null),
  };

  return (
    <WorkflowContext.Provider value={value}>
      {children}
    </WorkflowContext.Provider>
  );
}
