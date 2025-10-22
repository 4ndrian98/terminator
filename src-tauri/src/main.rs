// Prevents additional console window on Windows in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use terminator_workflow_recorder::{WorkflowRecorder, WorkflowRecorderConfig};
use tracing::{info, error};

// State per il recorder
struct RecorderState {
    recorder: Arc<Mutex<Option<WorkflowRecorder>>>,
    recording: Arc<Mutex<bool>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WorkflowInfo {
    name: String,
    file_path: String,
    created_at: Option<u64>,
    duration_ms: Option<u64>,
    event_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkflowStats {
    click_count: usize,
    keyboard_count: usize,
    hotkey_count: usize,
    text_input_count: usize,
    app_switch_count: usize,
}

// ============================================================================
// COMANDI TAURI - Registrazione Workflow
// ============================================================================

#[tauri::command]
async fn start_recording(
    workflow_name: String,
    state: State<'_, RecorderState>,
) -> Result<String, String> {
    info!("🎬 Avvio registrazione workflow: {}", workflow_name);
    
    let mut recording = state.recording.lock().map_err(|e| e.to_string())?;
    if *recording {
        return Err("Registrazione già in corso".to_string());
    }

    let config = WorkflowRecorderConfig::default();
    let recorder = WorkflowRecorder::new(workflow_name.clone(), config);
    
    let mut recorder_guard = state.recorder.lock().map_err(|e| e.to_string())?;
    *recorder_guard = Some(recorder);
    
    if let Some(ref mut rec) = *recorder_guard {
        rec.start().await.map_err(|e| e.to_string())?;
    }
    
    *recording = true;
    
    info!("✅ Registrazione avviata con successo");
    Ok("Registrazione avviata".to_string())
}

#[tauri::command]
async fn stop_recording(
    state: State<'_, RecorderState>,
) -> Result<String, String> {
    info!("🛑 Arresto registrazione");
    
    let mut recording = state.recording.lock().map_err(|e| e.to_string())?;
    if !*recording {
        return Err("Nessuna registrazione in corso".to_string());
    }

    let mut recorder_guard = state.recorder.lock().map_err(|e| e.to_string())?;
    
    if let Some(ref mut rec) = *recorder_guard {
        rec.stop().await.map_err(|e| e.to_string())?;
    }
    
    *recording = false;
    
    info!("✅ Registrazione arrestata");
    Ok("Registrazione arrestata".to_string())
}

#[tauri::command]
async fn save_workflow(
    file_name: String,
    state: State<'_, RecorderState>,
) -> Result<String, String> {
    info!("💾 Salvataggio workflow: {}", file_name);
    
    let mut recorder_guard = state.recorder.lock().map_err(|e| e.to_string())?;
    
    if let Some(ref mut rec) = *recorder_guard {
        let output_path = format!("workflows/{}.json", file_name);
        
        // Crea directory workflows se non esiste
        std::fs::create_dir_all("workflows").map_err(|e| e.to_string())?;
        
        rec.save(&output_path).map_err(|e| e.to_string())?;
        
        info!("✅ Workflow salvato: {}", output_path);
        Ok(output_path)
    } else {
        Err("Nessun recorder disponibile".to_string())
    }
}

#[tauri::command]
async fn is_recording(state: State<'_, RecorderState>) -> Result<bool, String> {
    let recording = state.recording.lock().map_err(|e| e.to_string())?;
    Ok(*recording)
}

// ============================================================================
// COMANDI TAURI - Gestione Workflow
// ============================================================================

#[tauri::command]
async fn list_workflows() -> Result<Vec<WorkflowInfo>, String> {
    info!("📋 Caricamento lista workflow");
    
    let workflows_dir = PathBuf::from("workflows");
    
    if !workflows_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut workflows = Vec::new();
    
    let entries = std::fs::read_dir(&workflows_dir).map_err(|e| e.to_string())?;
    
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                let file_path = path.to_string_lossy().to_string();
                
                // Leggi il file per ottenere info
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(workflow_data) = serde_json::from_str::<serde_json::Value>(&content) {
                        let start_time = workflow_data["start_time"].as_u64();
                        let end_time = workflow_data["end_time"].as_u64();
                        let events = workflow_data["events"].as_array();
                        
                        let duration_ms = if let (Some(start), Some(end)) = (start_time, end_time) {
                            Some(end - start)
                        } else {
                            None
                        };
                        
                        workflows.push(WorkflowInfo {
                            name: file_name.to_string(),
                            file_path,
                            created_at: start_time,
                            duration_ms,
                            event_count: events.map(|e| e.len()).unwrap_or(0),
                        });
                    }
                }
            }
        }
    }
    
    // Ordina per data creazione (più recenti prima)
    workflows.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    info!("✅ Trovati {} workflow", workflows.len());
    Ok(workflows)
}

#[tauri::command]
async fn get_workflow_info(workflow_name: String) -> Result<serde_json::Value, String> {
    info!("ℹ️  Caricamento info workflow: {}", workflow_name);
    
    let file_path = format!("workflows/{}.json", workflow_name);
    let content = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let workflow_data: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    Ok(workflow_data)
}

#[tauri::command]
async fn delete_workflow(workflow_name: String) -> Result<String, String> {
    info!("🗑️  Eliminazione workflow: {}", workflow_name);
    
    let file_path = format!("workflows/{}.json", workflow_name);
    std::fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    
    info!("✅ Workflow eliminato");
    Ok("Workflow eliminato con successo".to_string())
}

// ============================================================================
// COMANDI TAURI - Esecuzione Workflow
// ============================================================================

#[tauri::command]
async fn execute_workflow(
    workflow_name: String,
    repetitions: u32,
    speed: f32,
) -> Result<String, String> {
    info!("▶️  Esecuzione workflow: {} (ripetizioni: {}, velocità: {}x)", 
          workflow_name, repetitions, speed);
    
    // TODO: Implementare esecuzione reale usando terminator
    // Per ora ritorna successo simulato
    
    Ok(format!("Workflow {} eseguito {} volte a velocità {}x", 
               workflow_name, repetitions, speed))
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Avvio Workflow Automator GUI");

    // Crea state per il recorder
    let recorder_state = RecorderState {
        recorder: Arc::new(Mutex::new(None)),
        recording: Arc::new(Mutex::new(false)),
    };

    tauri::Builder::default()
        .manage(recorder_state)
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            save_workflow,
            is_recording,
            list_workflows,
            get_workflow_info,
            delete_workflow,
            execute_workflow,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
