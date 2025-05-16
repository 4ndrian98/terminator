use crate::{
    KeyboardEvent, MouseButton, MouseEvent, MouseEventType, Position, UiElement, WindowEvent,
    WorkflowEvent, WorkflowRecorderError, Result, WorkflowRecorderConfig
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, info, warn};

#[cfg(target_os = "windows")]
use {
    std::ffi::OsString,
    std::os::windows::ffi::OsStringExt,
    uiautomation::{UIAutomation, UIElement as WinUIElement},
    windows::{
        Win32::Foundation::{HWND, LPARAM, POINT, WPARAM},
        Win32::UI::WindowsAndMessaging::{
            GetWindowTextW, GetWindowThreadProcessId, SetWindowsHookExW, UnhookWindowsHookEx,
            CallNextHookEx, HC_ACTION, WH_KEYBOARD_LL, WH_MOUSE_LL, KBDLLHOOKSTRUCT,
            MSLLHOOKSTRUCT, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_RBUTTONDOWN,
            WM_RBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL,
        },
        Win32::System::Threading::{
            GetCurrentProcessId, GetCurrentThreadId, OpenProcess, PROCESS_QUERY_INFORMATION,
            PROCESS_VM_READ,
        },
        Win32::UI::Accessibility::{
            AccessibleObjectFromPoint, IAccessible,
        },
    },
};

/// The Windows-specific recorder
pub struct WindowsRecorder {
    /// The UI Automation instance
    automation: Arc<UIAutomation>,
    
    /// The keyboard hook handle
    keyboard_hook: Option<isize>,
    
    /// The mouse hook handle
    mouse_hook: Option<isize>,
    
    /// The event sender
    event_tx: UnboundedSender<WorkflowEvent>,
    
    /// The configuration
    config: WorkflowRecorderConfig,
    
    /// The last mouse position
    last_mouse_pos: Arc<Mutex<Option<POINT>>>,
}

#[cfg(target_os = "windows")]
impl WindowsRecorder {
    /// Create a new Windows recorder
    pub fn new(
        config: WorkflowRecorderConfig,
        event_tx: UnboundedSender<WorkflowEvent>,
    ) -> Result<Self> {
        // Create UI Automation instance
        let automation = Arc::new(
            UIAutomation::new().map_err(|e| {
                WorkflowRecorderError::InitializationError(format!(
                    "Failed to initialize UI Automation: {}",
                    e
                ))
            })?,
        );
        
        let last_mouse_pos = Arc::new(Mutex::new(None));
        
        let mut recorder = Self {
            automation,
            keyboard_hook: None,
            mouse_hook: None,
            event_tx,
            config,
            last_mouse_pos,
        };
        
        // Set up hooks
        recorder.setup_hooks()?;
        
        Ok(recorder)
    }
    
    /// Set up the Windows hooks
    fn setup_hooks(&mut self) -> Result<()> {
        // Set up keyboard hook if enabled
        if self.config.record_keyboard {
            self.setup_keyboard_hook()?;
        }
        
        // Set up mouse hook if enabled
        if self.config.record_mouse {
            self.setup_mouse_hook()?;
        }
        
        Ok(())
    }
    
    /// Set up the keyboard hook
    fn setup_keyboard_hook(&mut self) -> Result<()> {
        let event_tx = self.event_tx.clone();
        
        // Define the keyboard hook procedure
        unsafe extern "system" fn keyboard_hook_proc(
            code: i32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> isize {
            if code < 0 || code != HC_ACTION {
                return CallNextHookEx(None, code, wparam, lparam);
            }
            
            let hook_struct = *(lparam.0 as *const KBDLLHOOKSTRUCT);
            let key_code = hook_struct.vkCode;
            
            // Check if key down or up
            let is_key_down = wparam.0 == WM_KEYDOWN as usize;
            let is_key_up = wparam.0 == WM_KEYUP as usize;
            
            if is_key_down || is_key_up {
                // Get modifier key states
                let ctrl_pressed = (hook_struct.flags & 0x8) != 0 || key_code == 17;
                let alt_pressed = (hook_struct.flags & 0x20) != 0 || key_code == 18;
                let shift_pressed = (hook_struct.flags & 0x1) != 0 || key_code == 16;
                let win_pressed = key_code == 91 || key_code == 92;
                
                // Create keyboard event
                let keyboard_event = KeyboardEvent {
                    key_code,
                    is_key_down,
                    ctrl_pressed,
                    alt_pressed,
                    shift_pressed,
                    win_pressed,
                };
                
                // Send event
                let _ = EVENT_TX.as_ref().unwrap().send(WorkflowEvent::Keyboard(keyboard_event));
            }
            
            CallNextHookEx(None, code, wparam, lparam)
        }
        
        // Store the event sender in a thread-local static
        thread_local! {
            static EVENT_TX: std::cell::RefCell<Option<UnboundedSender<WorkflowEvent>>> = std::cell::RefCell::new(None);
        }
        
        EVENT_TX.with(|tx| {
            *tx.borrow_mut() = Some(event_tx);
        });
        
        // Set the keyboard hook
        unsafe {
            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_proc),
                None,
                0,
            );
            
            if hook.is_null() {
                return Err(WorkflowRecorderError::InitializationError(
                    "Failed to set keyboard hook".to_string(),
                ));
            }
            
            self.keyboard_hook = Some(hook.0);
        }
        
        Ok(())
    }
    
    /// Set up the mouse hook
    fn setup_mouse_hook(&mut self) -> Result<()> {
        let event_tx = self.event_tx.clone();
        let automation = Arc::clone(&self.automation);
        let last_mouse_pos = Arc::clone(&self.last_mouse_pos);
        let capture_ui_elements = self.config.capture_ui_elements;
        
        // Define the mouse hook procedure
        unsafe extern "system" fn mouse_hook_proc(
            code: i32,
            wparam: WPARAM,
            lparam: LPARAM,
        ) -> isize {
            if code < 0 || code != HC_ACTION {
                return CallNextHookEx(None, code, wparam, lparam);
            }
            
            let hook_struct = *(lparam.0 as *const MSLLHOOKSTRUCT);
            let x = hook_struct.pt.x;
            let y = hook_struct.pt.y;
            
            // Store the current mouse position
            if let Some(last_pos) = LAST_MOUSE_POS.as_ref() {
                if let Ok(mut last_pos) = last_pos.lock() {
                    *last_pos = Some(POINT { x, y });
                }
            }
            
            // Determine the mouse event type and button
            let (event_type, button) = match wparam.0 as u32 {
                WM_LBUTTONDOWN => (MouseEventType::Down, MouseButton::Left),
                WM_LBUTTONUP => (MouseEventType::Up, MouseButton::Left),
                WM_RBUTTONDOWN => (MouseEventType::Down, MouseButton::Right),
                WM_RBUTTONUP => (MouseEventType::Up, MouseButton::Right),
                WM_MBUTTONDOWN => (MouseEventType::Down, MouseButton::Middle),
                WM_MBUTTONUP => (MouseEventType::Up, MouseButton::Middle),
                WM_MOUSEMOVE => (MouseEventType::Move, MouseButton::Left),
                WM_MOUSEWHEEL => (MouseEventType::Wheel, MouseButton::Middle),
                _ => return CallNextHookEx(None, code, wparam, lparam),
            };
            
            // Skip mouse move events unless it's a significant movement
            if event_type == MouseEventType::Move {
                // Only process every 10th mouse move event to reduce noise
                static mut MOVE_COUNTER: u32 = 0;
                MOVE_COUNTER += 1;
                if MOVE_COUNTER % 10 != 0 {
                    return CallNextHookEx(None, code, wparam, lparam);
                }
            }
            
            // Create position
            let position = Position { x, y };
            
            // Get UI element under mouse if needed
            let mut ui_element = None;
            if CAPTURE_UI_ELEMENTS && (event_type == MouseEventType::Down || event_type == MouseEventType::Up) {
                if let Some(automation) = AUTOMATION.as_ref() {
                    ui_element = get_ui_element_at_point(automation, x, y);
                }
            }
            
            // Create mouse event
            let mouse_event = MouseEvent {
                event_type,
                button,
                position,
                ui_element,
            };
            
            // Send event
            if let Some(tx) = EVENT_TX.as_ref() {
                let _ = tx.send(WorkflowEvent::Mouse(mouse_event));
            }
            
            CallNextHookEx(None, code, wparam, lparam)
        }
        
        // Store the necessary data in thread-local statics
        thread_local! {
            static EVENT_TX: std::cell::RefCell<Option<UnboundedSender<WorkflowEvent>>> = std::cell::RefCell::new(None);
            static AUTOMATION: std::cell::RefCell<Option<Arc<UIAutomation>>> = std::cell::RefCell::new(None);
            static LAST_MOUSE_POS: std::cell::RefCell<Option<Arc<Mutex<Option<POINT>>>>> = std::cell::RefCell::new(None);
            static CAPTURE_UI_ELEMENTS: bool = false;
        }
        
        EVENT_TX.with(|tx| {
            *tx.borrow_mut() = Some(event_tx);
        });
        
        AUTOMATION.with(|auto| {
            *auto.borrow_mut() = Some(automation);
        });
        
        LAST_MOUSE_POS.with(|pos| {
            *pos.borrow_mut() = Some(last_mouse_pos);
        });
        
        CAPTURE_UI_ELEMENTS.with(|capture| {
            *capture = capture_ui_elements;
        });
        
        // Set the mouse hook
        unsafe {
            let hook = SetWindowsHookExW(
                WH_MOUSE_LL,
                Some(mouse_hook_proc),
                None,
                0,
            );
            
            if hook.is_null() {
                return Err(WorkflowRecorderError::InitializationError(
                    "Failed to set mouse hook".to_string(),
                ));
            }
            
            self.mouse_hook = Some(hook.0);
        }
        
        Ok(())
    }
    
    /// Stop recording
    pub fn stop(&self) -> Result<()> {
        // Unhook the keyboard hook
        if let Some(hook) = self.keyboard_hook {
            unsafe {
                if UnhookWindowsHookEx(HWND(hook)).is_err() {
                    warn!("Failed to unhook keyboard hook");
                }
            }
        }
        
        // Unhook the mouse hook
        if let Some(hook) = self.mouse_hook {
            unsafe {
                if UnhookWindowsHookEx(HWND(hook)).is_err() {
                    warn!("Failed to unhook mouse hook");
                }
            }
        }
        
        Ok(())
    }
}

/// Get the UI element at the given point
#[cfg(target_os = "windows")]
fn get_ui_element_at_point(automation: &UIAutomation, x: i32, y: i32) -> Option<UiElement> {
    // Try to get the UI element at the point using UI Automation
    match automation.element_from_point(x as f64, y as f64) {
        Ok(element) => {
            // Get element properties
            let name = element.get_name().ok();
            let automation_id = element.get_automation_id().ok();
            let class_name = element.get_classname().ok();
            let control_type = element.get_control_type_name().ok();
            let process_id = element.get_process_id().ok().map(|pid| pid as u32);
            
            // Get window title and application name
            let (window_title, application_name) = if let Some(pid) = process_id {
                get_window_info_for_process(pid)
            } else {
                (None, None)
            };
            
            Some(UiElement {
                name,
                automation_id,
                class_name,
                control_type,
                process_id,
                application_name,
                window_title,
            })
        }
        Err(_) => None,
    }
}

/// Get window information for the given process ID
#[cfg(target_os = "windows")]
fn get_window_info_for_process(process_id: u32) -> (Option<String>, Option<String>) {
    let mut window_title = None;
    let mut application_name = None;
    
    unsafe {
        // Open the process to get its name
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        );
        
        if !process_handle.is_invalid() {
            // TODO: Get application name from process handle
            // This would require using GetModuleFileNameEx or similar
            // For simplicity, we'll leave it as None for now
        }
        
        // Find the main window for this process
        // This is a simplified approach; in a real implementation,
        // you would enumerate all windows and find the one matching the process ID
        // For simplicity, we'll leave window_title as None for now
    }
    
    (window_title, application_name)
} 