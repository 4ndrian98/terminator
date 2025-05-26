use tokio_stream::StreamExt;
use terminator_workflow_recorder::{WorkflowRecorder, WorkflowRecorderConfig};
use std::path::PathBuf;
use tokio::signal::ctrl_c;
use tracing::{info, debug, Level};
use tracing_subscriber::FmtSubscriber;
// use std::panic::AssertUnwindSafe; // Not used due to async limitation

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[EARLY] Comprehensive workflow recorder started");
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    info!("[LOG] Comprehensive workflow recorder initialized");

    info!("[LOG] Setting up comprehensive recording configuration");
    
    // Create a comprehensive configuration for maximum workflow capture
    let config = WorkflowRecorderConfig {
        // Basic input recording
        record_mouse: false,
        record_keyboard: true,
        record_window: true,
        capture_ui_elements: true,
        
        // Advanced workflow features
        record_clipboard: true,
        record_text_input: true,
        record_text_selection: true,
        record_applications: true,
        record_file_operations: true, // Can be very noisy, enable if needed
        record_menu_interactions: true,
        record_dialog_interactions: true,
        record_scroll: false,
        record_system_events: true, // Can be noisy, enable for comprehensive system monitoring
        record_drag_drop: true,
        record_hotkeys: true,
        
        // Configuration tuning
        max_clipboard_content_length: 2048, // 2KB max for clipboard content
        max_text_selection_length: 512, // 512 chars max for text selections
        record_window_geometry: true,
        track_modifier_states: true,
        detailed_scroll_tracking: false, // Set to true for detailed scroll analysis
        monitor_file_system: true, 
        file_system_watch_paths: vec![], // Add specific paths if file monitoring is enabled
        record_network_events: true, // Privacy-sensitive, enable if needed
        record_multimedia_events: true, // Can be noisy, enable if needed
        mouse_move_throttle_ms: 50, // 20 FPS max for mouse moves to reduce noise
        min_drag_distance: 5.0, // 5 pixels minimum for drag detection
    };
    
    debug!("Comprehensive recorder config: {:?}", config);
    
    // Create the comprehensive workflow recorder
    let mut recorder = WorkflowRecorder::new("Comprehensive Workflow Recording".to_string(), config);
    
    debug!("Starting comprehensive recording...");
    let mut event_stream = recorder.event_stream();
    recorder.start().await.expect("Failed to start comprehensive recorder");
    
    info!("📊 Comprehensive workflow recording started!");
    info!("🎯 Recording the following interactions:");
    info!("   • Mouse movements, clicks, and drags");
    info!("   • Keyboard input with modifier key tracking");
    info!("   • Clipboard operations (copy/paste/cut)");
    info!("   • Text selection with mouse and keyboard");
    info!("   • Window management (focus, move, resize)");
    info!("   • UI element interactions with detailed context");
    info!("   • Hotkey combinations and shortcuts");
    info!("   • Scroll events and directions");
    info!("   • Text input with UI element context");
    info!("   • Drag and drop operations");
    info!("   • Menu and dialog interactions");
    info!("");
    info!("💡 Interact with your desktop to see comprehensive event capture...");
    info!("🛑 Press Ctrl+C to stop recording and save the workflow");
    
    // Process and display events from the stream
    let event_display_task = tokio::spawn(async move {
        let mut event_count = 0;
        while let Some(event) = event_stream.next().await {
            event_count += 1;
            
            // Display different event types with appropriate detail levels
            match &event {
                terminator_workflow_recorder::WorkflowEvent::Mouse(mouse_event) => {
                    if mouse_event.event_type != terminator_workflow_recorder::MouseEventType::Move {
                        println!("🖱️  Mouse {}: {:?} at ({}, {})", 
                            event_count,
                            mouse_event.event_type, 
                            mouse_event.position.x, 
                            mouse_event.position.y);
                        
                        if let Some(ref ui_element) = mouse_event.ui_element {
                            if let Some(ref app) = ui_element.application_name {
                                println!("     └─ Target: {} in {}", 
                                    ui_element.control_type.as_ref().unwrap_or(&"Unknown".to_string()),
                                    app);
                            }
                        }
                    }
                }
                terminator_workflow_recorder::WorkflowEvent::Keyboard(kb_event) => {
                    if kb_event.is_key_down {
                        let modifiers = format!("{}{}{}{}",
                            if kb_event.ctrl_pressed { "Ctrl+" } else { "" },
                            if kb_event.alt_pressed { "Alt+" } else { "" },
                            if kb_event.shift_pressed { "Shift+" } else { "" },
                            if kb_event.win_pressed { "Win+" } else { "" }
                        );
                        
                        if let Some(ch) = kb_event.character {
                            println!("⌨️  Keyboard {}: {}'{}'", event_count, modifiers, ch);
                        } else {
                            println!("⌨️  Keyboard {}: {}Key({})", event_count, modifiers, kb_event.key_code);
                        }
                        
                        if let Some(ref ui_element) = kb_event.ui_element {
                            if let Some(ref app) = ui_element.application_name {
                                println!("     └─ Target: {} in {}", 
                                    ui_element.control_type.as_ref().unwrap_or(&"Unknown".to_string()),
                                    app);
                            }
                            if let Some(ref name) = ui_element.name {
                                if !name.is_empty() {
                                    println!("     └─ Element: \"{}\"", name);
                                }
                            }
                        }
                    }
                }
                terminator_workflow_recorder::WorkflowEvent::Clipboard(clip_event) => {
                    println!("📋 Clipboard {}: {:?}", event_count, clip_event.action);
                    if let Some(ref content) = clip_event.content {
                        let preview = if content.len() > 50 {
                            format!("{}...", &content[..50])
                        } else {
                            content.clone()
                        };
                        println!("     └─ Content: \"{}\"", preview);
                    }
                }
                terminator_workflow_recorder::WorkflowEvent::TextSelection(selection_event) => {
                    println!("✨ Text Selection {}: {} chars selected", 
                        event_count, 
                        selection_event.selection_length);
                    
                    let preview = if selection_event.selected_text.len() > 60 {
                        format!("{}...", &selection_event.selected_text[..60])
                    } else {
                        selection_event.selected_text.clone()
                    };
                    
                    println!("     └─ Text: \"{}\"", preview);
                    
                    if let Some(ref app) = selection_event.application {
                        println!("     └─ App: {}, Method: {:?}", app, selection_event.selection_method);
                    }
                }
                terminator_workflow_recorder::WorkflowEvent::Hotkey(hotkey_event) => {
                    println!("🔥 Hotkey {}: {} -> {}", 
                        event_count,
                        hotkey_event.combination,
                        hotkey_event.action.as_ref().unwrap_or(&"Unknown".to_string()));
                }
                terminator_workflow_recorder::WorkflowEvent::Scroll(scroll_event) => {
                    println!("📜 Scroll {}: {:?} delta({}, {}) at ({}, {})", 
                        event_count,
                        scroll_event.direction,
                        scroll_event.delta.0,
                        scroll_event.delta.1,
                        scroll_event.position.x,
                        scroll_event.position.y);
                }
                terminator_workflow_recorder::WorkflowEvent::DragDrop(drag_event) => {
                    println!("🎯 Drag & Drop {}: from ({}, {}) to ({}, {})", 
                        event_count,
                        drag_event.start_position.x,
                        drag_event.start_position.y,
                        drag_event.end_position.x,
                        drag_event.end_position.y);
                }
                terminator_workflow_recorder::WorkflowEvent::File(file_event) => {
                    println!("📁 File {}: {:?} -> {}", 
                        event_count,
                        file_event.action,
                        file_event.path);
                }
                _ => {
                    // Display other event types more briefly
                    println!("📝 Event {}: {:?}", event_count, event);
                }
            }
        }
    });
    
    debug!("Waiting for Ctrl+C signal...");
    ctrl_c().await.expect("Failed to wait for Ctrl+C");
    
    info!("🛑 Stop signal received, finalizing recording...");
    debug!("Sending stop signal to recorder...");
    recorder.stop().await.expect("Failed to stop recorder");
    
    // Cancel the event display task
    event_display_task.abort();
    
    let output_path = PathBuf::from("comprehensive_workflow_recording.json");
    debug!("Saving comprehensive recording to {:?}", output_path);
    recorder.save(&output_path).expect("Failed to save recording");
    
    info!("✅ Comprehensive workflow recording saved to {:?}", output_path);
    info!("📊 The recording includes detailed interaction context and metadata");
    info!("🔍 You can analyze the JSON file to understand the complete workflow");
    
    Ok(())
} 