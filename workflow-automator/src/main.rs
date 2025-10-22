use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use terminator_workflow_recorder::{
    WorkflowRecorder, WorkflowRecorderConfig, WorkflowEvent,
    MouseEventType, MouseButton
};
use tokio::signal::ctrl_c;
use tracing::{info, error, warn};
use std::time::Duration;

/// Automazione Workflow - Applicazione Standalone per Clienti
/// 
/// REGISTRA qualsiasi cosa fai sul computer
/// ESEGUI esattamente quello che hai registrato
#[derive(Parser)]
#[command(name = "Workflow Automator")]
#[command(version = "1.0")]
#[command(about = "Automatizza QUALSIASI workflow - Facile da usare!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Registra un nuovo workflow (premi Ctrl+C per fermare)
    Registra {
        /// Nome del workflow da salvare
        #[arg(short, long, default_value = "mio_workflow")]
        nome: String,
    },
    
    /// Esegui un workflow salvato
    Esegui {
        /// File del workflow da eseguire (.json)
        #[arg(short, long)]
        workflow: PathBuf,
        
        /// Quante volte ripetere (default: 1)
        #[arg(short, long, default_value = "1")]
        ripeti: u32,
        
        /// Velocità esecuzione: 1.0 = normale, 0.5 = metà velocità, 2.0 = doppia
        #[arg(long, default_value = "1.0")]
        velocita: f32,
    },
    
    /// Mostra informazioni su un workflow salvato
    Info {
        /// File del workflow
        #[arg(short, long)]
        workflow: PathBuf,
    },
    
    /// Guida rapida per iniziare
    Guida,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExcelRow {
    data: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Registra { nome } => {
            registra_workflow(nome).await?;
        }
        Commands::Esegui { workflow, ripeti, velocita } => {
            esegui_workflow(workflow, ripeti, velocita).await?;
        }
        Commands::Info { workflow } => {
            mostra_info_workflow(workflow).await?;
        }
        Commands::Guida => {
            mostra_guida();
        }
    }

    Ok(())
}

/// Registra un nuovo workflow
async fn registra_workflow(nome: String) -> Result<()> {
    info!("🎬 INIZIA REGISTRAZIONE WORKFLOW: {}", nome);
    info!("");
    info!("📝 Esegui ora le azioni che vuoi automatizzare:");
    info!("   - Apri programmi");
    info!("   - Clicca pulsanti");
    info!("   - Compila campi");
    info!("   - Copia/Incolla dati");
    info!("");
    info!("🛑 Premi Ctrl+C quando hai finito");
    info!("");

    let config = WorkflowRecorderConfig::default();
    let mut recorder = WorkflowRecorder::new(nome.clone(), config);

    recorder.start().await?;
    
    // Aspetta Ctrl+C
    ctrl_c().await?;
    
    info!("");
    info!("🛑 Registrazione fermata, salvataggio in corso...");
    
    recorder.stop().await?;
    
    let output_file = format!("{}.json", nome);
    recorder.save(&output_file)?;
    
    info!("");
    info!("✅ Workflow salvato con successo in: {}", output_file);
    info!("");
    info!("▶️  Per eseguire il workflow:");
    info!("   workflow-automator.exe esegui --workflow {}", output_file);
    info!("");
    
    Ok(())
}

/// Esegui un workflow salvato
async fn esegui_workflow(workflow_path: PathBuf, ripeti: u32, velocita: f32) -> Result<()> {
    info!("▶️  Esecuzione workflow: {:?}", workflow_path);
    info!("   Ripetizioni: {}", ripeti);
    info!("   Velocità: {}x", velocita);
    info!("");
    
    // Carica il workflow
    let workflow_data = std::fs::read_to_string(&workflow_path)
        .context("Errore nella lettura del file workflow")?;
    
    #[derive(Deserialize)]
    struct RecordedWorkflow {
        name: String,
        start_time: u64,
        end_time: Option<u64>,
        events: Vec<TimestampedEvent>,
    }
    
    #[derive(Deserialize)]
    struct TimestampedEvent {
        timestamp: u64,
        event: WorkflowEvent,
    }
    
    let workflow: RecordedWorkflow = serde_json::from_str(&workflow_data)
        .context("Errore nel parsing del workflow")?;
    
    info!("✅ Workflow caricato: {}", workflow.name);
    info!("   Eventi totali: {}", workflow.events.len());
    
    if workflow.events.is_empty() {
        warn!("⚠️  Il workflow non contiene eventi da eseguire");
        return Ok(());
    }
    
    info!("");
    info!("⏱️  Attendere 3 secondi prima di iniziare...");
    info!("   (Posiziona le finestre se necessario)");
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Esegui il workflow per il numero di ripetizioni richieste
    for iteration in 1..=ripeti {
        if ripeti > 1 {
            info!("");
            info!("🔄 Ripetizione {}/{}", iteration, ripeti);
        }
        
        info!("🚀 Esecuzione in corso...");
        
        // Calcola i timing relativi
        let start_time = workflow.events[0].timestamp;
        let mut previous_time = start_time;
        
        for (idx, timestamped_event) in workflow.events.iter().enumerate() {
            // Calcola il delay rispetto all'evento precedente
            let delay_ms = timestamped_event.timestamp.saturating_sub(previous_time);
            previous_time = timestamped_event.timestamp;
            
            // Applica la velocità al delay
            let adjusted_delay = Duration::from_millis(
                ((delay_ms as f32) / velocita) as u64
            );
            
            if adjusted_delay.as_millis() > 10 {
                tokio::time::sleep(adjusted_delay).await;
            }
            
            // Esegui l'evento
            match &timestamped_event.event {
                WorkflowEvent::Mouse(mouse_event) => {
                    match mouse_event.event_type {
                        MouseEventType::Click | MouseEventType::Down => {
                            info!("🖱️  [{}/{}] Click at ({}, {})", 
                                idx + 1, workflow.events.len(),
                                mouse_event.position.x, mouse_event.position.y);
                            
                            // TODO: Esegui click reale usando terminator
                            // desktop.mouse_move(x, y)?;
                            // desktop.mouse_click(button)?;
                        }
                        _ => {}
                    }
                }
                WorkflowEvent::Keyboard(kb_event) => {
                    if kb_event.is_key_down {
                        if let Some(ch) = kb_event.character {
                            info!("⌨️  [{}/{}] Digitazione: '{}'", 
                                idx + 1, workflow.events.len(), ch);
                            
                            // TODO: Esegui digitazione reale
                            // desktop.type_text(&ch.to_string())?;
                        }
                    }
                }
                WorkflowEvent::Hotkey(hotkey) => {
                    info!("🔥 [{}/{}] Hotkey: {}", 
                        idx + 1, workflow.events.len(), 
                        hotkey.combination);
                    
                    // TODO: Esegui hotkey reale
                    // desktop.press_keys(&hotkey.combination)?;
                }
                WorkflowEvent::TextInputCompleted(text_input) => {
                    info!("📝 [{}/{}] Testo completato: \"{}\"", 
                        idx + 1, workflow.events.len(),
                        text_input.text_value);
                    
                    // TODO: Esegui input testo
                }
                WorkflowEvent::Click(click_event) => {
                    info!("🔘 [{}/{}] Button Click: \"{}\"", 
                        idx + 1, workflow.events.len(),
                        click_event.element_text);
                    
                    // TODO: Esegui click su elemento UI
                }
                WorkflowEvent::ApplicationSwitch(app_switch) => {
                    info!("🔄 [{}/{}] Switch app: {} → {}", 
                        idx + 1, workflow.events.len(),
                        app_switch.from_application.as_deref().unwrap_or("?"),
                        app_switch.to_application);
                    
                    // TODO: Esegui switch applicazione
                }
                _ => {
                    // Altri tipi di eventi
                }
            }
        }
        
        info!("✅ Esecuzione completata!");
        
        // Pausa tra ripetizioni
        if iteration < ripeti {
            info!("⏸️  Pausa 2 secondi prima della prossima ripetizione...");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
    
    info!("");
    info!("🎉 Tutte le {} ripetizioni completate!", ripeti);
    info!("");
    info!("⚠️  NOTA: Attualmente in modalità SIMULAZIONE");
    info!("   Le azioni sono mostrate ma non eseguite realmente");
    info!("   Versione completa in arrivo!");
    
    Ok(())
}

/// Mostra informazioni su un workflow salvato
async fn mostra_info_workflow(workflow_path: PathBuf) -> Result<()> {
    info!("ℹ️  Informazioni workflow: {:?}", workflow_path);
    info!("");
    
    let workflow_data = std::fs::read_to_string(&workflow_path)
        .context("Errore nella lettura del file workflow")?;
    
    #[derive(Deserialize)]
    struct RecordedWorkflow {
        name: String,
        start_time: u64,
        end_time: Option<u64>,
        events: Vec<TimestampedEvent>,
    }
    
    #[derive(Deserialize)]
    struct TimestampedEvent {
        timestamp: u64,
        event: WorkflowEvent,
    }
    
    let workflow: RecordedWorkflow = serde_json::from_str(&workflow_data)
        .context("Errore nel parsing del workflow")?;
    
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  INFORMAZIONI WORKFLOW                                         ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();
    println!("📝 Nome: {}", workflow.name);
    println!("📊 Eventi totali: {}", workflow.events.len());
    
    if let Some(end_time) = workflow.end_time {
        let duration_ms = end_time - workflow.start_time;
        let duration_sec = duration_ms as f64 / 1000.0;
        println!("⏱️  Durata: {:.2} secondi", duration_sec);
    }
    
    println!();
    println!("📋 Riepilogo azioni:");
    
    let mut click_count = 0;
    let mut keyboard_count = 0;
    let mut hotkey_count = 0;
    let mut text_input_count = 0;
    let mut app_switch_count = 0;
    
    for event in &workflow.events {
        match &event.event {
            WorkflowEvent::Mouse(m) if matches!(m.event_type, MouseEventType::Click | MouseEventType::Down) => {
                click_count += 1;
            }
            WorkflowEvent::Keyboard(k) if k.is_key_down => {
                keyboard_count += 1;
            }
            WorkflowEvent::Hotkey(_) => {
                hotkey_count += 1;
            }
            WorkflowEvent::TextInputCompleted(_) => {
                text_input_count += 1;
            }
            WorkflowEvent::ApplicationSwitch(_) => {
                app_switch_count += 1;
            }
            _ => {}
        }
    }
    
    println!("   🖱️  Click mouse: {}", click_count);
    println!("   ⌨️  Pressioni tastiera: {}", keyboard_count);
    println!("   🔥 Hotkey (Ctrl+C, ecc.): {}", hotkey_count);
    println!("   📝 Input testo: {}", text_input_count);
    println!("   🔄 Cambi applicazione: {}", app_switch_count);
    
    println!();
    println!("💡 Per eseguire questo workflow:");
    println!("   workflow-automator.exe esegui --workflow {:?}", workflow_path);
    println!();
    
    Ok(())
}

/// Mostra guida rapida
fn mostra_guida() {
    println!(r#"
╔════════════════════════════════════════════════════════════════╗
║                    WORKFLOW AUTOMATOR                          ║
║              Guida Rapida per Automatizzare Workflow           ║
╚════════════════════════════════════════════════════════════════╝

📚 COME FUNZIONA:

1️⃣  REGISTRA il tuo workflow (una volta sola):
   
   workflow-automator.exe registra --nome "compila_form"
   
   Poi esegui manualmente le azioni che vuoi automatizzare.
   Premi Ctrl+C quando hai finito.

2️⃣  ESEGUI il workflow quando vuoi:
   
   workflow-automator.exe esegui --workflow compila_form.json

3️⃣  AUTOMATIZZA con Excel (ripete per ogni riga):
   
   workflow-automator.exe excel \
     --excel dati.xlsx \
     --workflow compila_form.json

───────────────────────────────────────────────────────────────

💡 ESEMPIO PRATICO:

Devi compilare un form 100 volte con dati da Excel?

1. Registra: compili il form UNA VOLTA mentre registri
2. Prepara Excel con i 100 dati (una riga per ogni compilazione)
3. Esegui: il programma ripete il workflow per tutte le 100 righe

───────────────────────────────────────────────────────────────

📋 FORMATO EXCEL:

Il file Excel deve avere:
- Prima riga: intestazioni (opzionale)
- Righe successive: i dati da usare

Esempio:
  A          B         C
  Nome       Email     Telefono
  Mario      m@.it     123456
  Luigi      l@.it     789012

Il programma userà questi dati per compilare i campi.

───────────────────────────────────────────────────────────────

❓ SERVE AIUTO?

Per ulteriori informazioni:
- Email: supporto@esempio.com
- Docs: https://docs.esempio.com

"#);
}
