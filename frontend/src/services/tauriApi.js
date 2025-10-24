// @ts-check
import { invoke } from '@tauri-apps/api/tauri';

/**
 * API Service per comunicare con il backend Tauri
 */

// ============================================================================
// RECORDING API
// ============================================================================

/**
 * Avvia la registrazione di un nuovo workflow
 * @param {string} workflowName - Nome del workflow da registrare
 * @returns {Promise<string>}
 */
export async function startRecording(workflowName) {
  try {
    const result = await invoke('start_recording', { workflowName });
    return result;
  } catch (error) {
    console.error('Errore avvio registrazione:', error);
    throw error;
  }
}

/**
 * Ferma la registrazione corrente
 * @returns {Promise<string>}
 */
export async function stopRecording() {
  try {
    const result = await invoke('stop_recording');
    return result;
  } catch (error) {
    console.error('Errore arresto registrazione:', error);
    throw error;
  }
}

/**
 * Salva il workflow registrato
 * @param {string} fileName - Nome file per salvare il workflow
 * @returns {Promise<string>} - Path del file salvato
 */
export async function saveWorkflow(fileName) {
  try {
    const result = await invoke('save_workflow', { fileName });
    return result;
  } catch (error) {
    console.error('Errore salvataggio workflow:', error);
    throw error;
  }
}

/**
 * Verifica se una registrazione è in corso
 * @returns {Promise<boolean>}
 */
export async function isRecording() {
  try {
    const result = await invoke('is_recording');
    return result;
  } catch (error) {
    console.error('Errore verifica registrazione:', error);
    throw error;
  }
}

// ============================================================================
// WORKFLOW MANAGEMENT API
// ============================================================================

/**
 * Ottiene la lista di tutti i workflow salvati
 * @returns {Promise<Array<WorkflowInfo>>}
 */
export async function listWorkflows() {
  try {
    const result = await invoke('list_workflows');
    return result;
  } catch (error) {
    console.error('Errore caricamento workflow:', error);
    throw error;
  }
}

/**
 * Ottiene le informazioni dettagliate di un workflow
 * @param {string} workflowName - Nome del workflow
 * @returns {Promise<Object>}
 */
export async function getWorkflowInfo(workflowName) {
  try {
    const result = await invoke('get_workflow_info', { workflowName });
    return result;
  } catch (error) {
    console.error('Errore caricamento info workflow:', error);
    throw error;
  }
}

/**
 * Elimina un workflow
 * @param {string} workflowName - Nome del workflow da eliminare
 * @returns {Promise<string>}
 */
export async function deleteWorkflow(workflowName) {
  try {
    const result = await invoke('delete_workflow', { workflowName });
    return result;
  } catch (error) {
    console.error('Errore eliminazione workflow:', error);
    throw error;
  }
}

// ============================================================================
// EXECUTION API
// ============================================================================

/**
 * Esegue un workflow
 * @param {string} workflowName - Nome del workflow da eseguire
 * @param {number} repetitions - Numero di ripetizioni (default: 1)
 * @param {number} speed - Velocità esecuzione (default: 1.0)
 * @returns {Promise<string>}
 */
export async function executeWorkflow(workflowName, repetitions = 1, speed = 1.0) {
  try {
    const result = await invoke('execute_workflow', {
      workflowName,
      repetitions,
      speed,
    });
    return result;
  } catch (error) {
    console.error('Errore esecuzione workflow:', error);
    throw error;
  }
}

// ============================================================================
// LICENSE API
// ============================================================================

/**
 * Attiva una nuova licenza
 * @param {string} licenseKey - Chiave di licenza
 * @returns {Promise<License>}
 */
export async function activateLicense(licenseKey) {
  try {
    const result = await invoke('activate_license', { licenseKey });
    return result;
  } catch (error) {
    console.error('Errore attivazione licenza:', error);
    throw error;
  }
}

/**
 * Ottiene lo stato della licenza corrente
 * @returns {Promise<License|null>}
 */
export async function getLicenseStatus() {
  try {
    const result = await invoke('get_license_status');
    return result;
  } catch (error) {
    console.error('Errore caricamento stato licenza:', error);
    throw error;
  }
}

/**
 * Disattiva la licenza corrente
 * @returns {Promise<string>}
 */
export async function deactivateLicense() {
  try {
    const result = await invoke('deactivate_license');
    return result;
  } catch (error) {
    console.error('Errore disattivazione licenza:', error);
    throw error;
  }
}

// ============================================================================
// TYPES (JSDoc)
// ============================================================================

/**
 * @typedef {Object} WorkflowInfo
 * @property {string} name - Nome del workflow
 * @property {string} file_path - Path del file
 * @property {number} [created_at] - Timestamp creazione
 * @property {number} [duration_ms] - Durata in millisecondi
 * @property {number} event_count - Numero di eventi
 */
