use std::path::PathBuf;
use terminator_workflow_recorder::{WorkflowRecorder, WorkflowRecorderConfig};
use tokio::signal::ctrl_c;
use tokio_stream::StreamExt;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
// use std::panic::AssertUnwindSafe; // Not used due to async limitation

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[EARLY] Comprehensive workflow recorder started");
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
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
        record_mouse: true,
        record_keyboard: true,
        record_window: true,
        capture_ui_elements: true, // PERFORMANCE: Set to false for max speed if you don't need UI context

        // Advanced workflow features
        record_clipboard: true,
        record_text_selection: true,
        record_drag_drop: true,
        record_hotkeys: true,

        // High-level semantic events
        record_text_input_completion: true, // 🔥 NEW: High-level text input events
        text_input_completion_timeout_ms: 2000, // Complete typing after 2s pause OR focus change

        // UI Automation events
        record_ui_focus_changes: true,
        record_ui_structure_changes: true,
        record_ui_property_changes: true,

        // Configuration tuning
        max_clipboard_content_length: 2048, // 2KB max for clipboard content
        max_text_selection_length: 512,     // 512 chars max for text selections
        track_modifier_states: true,
        mouse_move_throttle_ms: 100, // PERFORMANCE: Increase throttle to reduce event spam
        min_drag_distance: 5.0,      // 5 pixels minimum for drag detection

        ..Default::default()
    };

    debug!("Comprehensive recorder config: {:?}", config);

    // Create the comprehensive workflow recorder
    let mut recorder =
        WorkflowRecorder::new("Comprehensive Workflow Recording".to_string(), config);

    debug!("Starting comprehensive recording...");
    let mut event_stream = recorder.event_stream();
    recorder
        .start()
        .await
        .expect("Failed to start comprehensive recorder");

    info!("📊 Comprehensive workflow recording started!");
    info!("🎯 Recording the following interactions:");
    info!("   • Mouse movements, clicks, and drags");
    info!("   • Keyboard input with modifier key tracking");
    info!("   • 🔥 HIGH-LEVEL TEXT INPUT COMPLETION (NEW!)");
    info!("     - Aggregates individual keystrokes into semantic 'text entered' events");
    info!("     - Captures final text value from UI elements after typing");
    info!("     - Detects typing vs pasting vs auto-fill methods");
    info!("   • Clipboard operations (copy/paste/cut)");
    info!("   • Text selection with mouse and keyboard");
    info!("   • Window management (focus, move, resize)");
    info!("   • UI element interactions with detailed context");
    info!("   • Hotkey combinations and shortcuts");
    info!("   • Scroll events and directions");
    info!("   • Drag and drop operations");
    info!("   • Menu and dialog interactions");
    info!("   • UI focus changes");
    info!("   • UI structure changes");
    info!("   • UI property changes");
    info!("");
    info!("💡 Interact with your desktop to see comprehensive event capture...");
    info!("");
    info!("🔥 TO TEST HIGH-LEVEL TEXT INPUT COMPLETION:");
    info!("   1. Open Notepad, a browser, or any app with text fields");
    info!("   2. Click in a text field and type something like 'john'");
    info!("   3. Click elsewhere or wait 2 seconds - you'll see a TextInputCompleted event!");
    info!("   4. Try typing vs pasting to see different input methods detected");
    info!("");
    info!("🛑 Press Ctrl+C to stop recording and save the workflow");

    // Process and display events from the stream
    let event_display_task = tokio::spawn(async move {
        let mut event_count = 0;
        while let Some(event) = event_stream.next().await {
            event_count += 1;

            // Display different event types with appropriate detail levels
            match &event {
                terminator_workflow_recorder::WorkflowEvent::Keyboard(kb_event) => {
                    if kb_event.is_key_down {
                        let modifiers = format!(
                            "{}{}{}{}",
                            if kb_event.ctrl_pressed { "Ctrl+" } else { "" },
                            if kb_event.alt_pressed { "Alt+" } else { "" },
                            if kb_event.shift_pressed { "Shift+" } else { "" },
                            if kb_event.win_pressed { "Win+" } else { "" }
                        );

                        if let Some(ch) = kb_event.character {
                            println!("⌨️  Keyboard {}: {}'{}'", event_count, modifiers, ch);
                        } else {
                            println!(
                                "⌨️  Keyboard {}: {}Key({})",
                                event_count, modifiers, kb_event.key_code
                            );
                        }

                        if let Some(ref ui_element) = kb_event.metadata.ui_element {
                            println!(
                                "     └─ Target: {} in {}",
                                ui_element.role(),
                                ui_element.application_name()
                            );

                            if let Some(ref name) = ui_element.name() {
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
                    println!(
                        "✨ Text Selection {}: {} chars selected",
                        event_count, selection_event.selection_length
                    );

                    let preview = if selection_event.selected_text.len() > 60 {
                        format!("{}...", &selection_event.selected_text[..60])
                    } else {
                        selection_event.selected_text.clone()
                    };

                    println!("     └─ Text: \"{}\"", preview);

                    println!(
                        "     └─ App: {}, Method: {:?}",
                        selection_event
                            .metadata
                            .ui_element
                            .as_ref()
                            .unwrap()
                            .application_name(),
                        selection_event.selection_method
                    );
                }
                terminator_workflow_recorder::WorkflowEvent::Hotkey(hotkey_event) => {
                    println!(
                        "🔥 Hotkey {}: {} -> {}",
                        event_count,
                        hotkey_event.combination,
                        hotkey_event
                            .action
                            .as_ref()
                            .unwrap_or(&"Unknown".to_string())
                    );
                }
                terminator_workflow_recorder::WorkflowEvent::DragDrop(drag_event) => {
                    println!(
                        "🎯 Drag & Drop {}: from ({}, {}) to ({}, {})",
                        event_count,
                        drag_event.start_position.x,
                        drag_event.start_position.y,
                        drag_event.end_position.x,
                        drag_event.end_position.y
                    );
                }
                terminator_workflow_recorder::WorkflowEvent::UiFocusChanged(focus_event) => {
                    println!(
                        "🎯 Focus changed to: {:?}",
                        focus_event
                            .metadata
                            .ui_element
                            .as_ref()
                            .unwrap()
                            .text(1)
                            .unwrap()
                    );
                }
                terminator_workflow_recorder::WorkflowEvent::UiPropertyChanged(property_event) => {
                    println!(
                        "🔧 Property changed: {:?}",
                        property_event
                            .metadata
                            .ui_element
                            .as_ref()
                            .unwrap()
                            .text(1)
                            .unwrap()
                    );
                }
                terminator_workflow_recorder::WorkflowEvent::TextInputCompleted(text_input_event) => {
                    println!(
                        "🔥 TEXT INPUT COMPLETED {}: \"{}\" ({} keystrokes in {}ms)",
                        event_count,
                        text_input_event.text_value,
                        text_input_event.keystroke_count,
                        text_input_event.typing_duration_ms
                    );
                    
                    // Show field details
                    if let Some(ref field_name) = text_input_event.field_name {
                        println!("     └─ Field: \"{}\" ({})", field_name, text_input_event.field_type);
                    } else {
                        println!("     └─ Field Type: {}", text_input_event.field_type);
                    }
                    
                    // Show input method
                    let method_icon = match text_input_event.input_method {
                        terminator_workflow_recorder::TextInputMethod::Typed => "⌨️ Typed",
                        terminator_workflow_recorder::TextInputMethod::Pasted => "📋 Pasted",
                        terminator_workflow_recorder::TextInputMethod::AutoFilled => "🤖 Auto-filled",
                        terminator_workflow_recorder::TextInputMethod::Mixed => "🔀 Mixed",
                    };
                    println!("     └─ Method: {}", method_icon);
                    
                    // Show application context
                    if let Some(ref ui_element) = text_input_event.metadata.ui_element {
                        println!("     └─ App: {}", ui_element.application_name());
                    }
                    
                    println!("     └─ 🎯 This is the high-level semantic event you wanted!");
                }
                _ => {
                    // Display other event types more briefly
                    // println!("📝 Event {}: {:?}", event_count, event);
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
    recorder
        .save(&output_path)
        .expect("Failed to save recording");

    info!(
        "✅ Comprehensive workflow recording saved to {:?}",
        output_path
    );
    info!("📊 The recording includes detailed interaction context and metadata");
    info!("🔍 You can analyze the JSON file to understand the complete workflow");

    Ok(())
}
