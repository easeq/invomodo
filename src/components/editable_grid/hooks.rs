use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

/// Breakpoint enum for different screen sizes
#[derive(Clone, PartialEq, Debug)]
pub enum Breakpoint {
    Mobile,  // < 640px
    Tablet,  // >= 640px && < 1024px
    Desktop, // >= 1024px
}

/// Device type for layout optimization
#[derive(Clone, PartialEq, Debug)]
pub enum DeviceType {
    Mobile,
    Desktop,
}

impl DeviceType {
    pub fn is_mobile(&self) -> bool {
        matches!(self, DeviceType::Mobile)
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, DeviceType::Desktop)
    }
}

/// Responsive hook that tracks screen size changes
pub fn use_responsive() -> (ReadSignal<Breakpoint>, ReadSignal<DeviceType>) {
    let (breakpoint, set_breakpoint) = signal(Breakpoint::Desktop);
    let (device_type, set_device_type) = signal(DeviceType::Desktop);

    let update_breakpoint = move || {
        if let Some(window) = window() {
            let width = window.inner_width().unwrap().as_f64().unwrap_or(1024.0);

            let new_breakpoint = if width < 640.0 {
                Breakpoint::Mobile
            } else if width < 1024.0 {
                Breakpoint::Tablet
            } else {
                Breakpoint::Desktop
            };

            let new_device_type = if width < 1024.0 {
                DeviceType::Mobile
            } else {
                DeviceType::Desktop
            };

            set_breakpoint.set(new_breakpoint);
            set_device_type.set(new_device_type);
        }
    };

    // Initial setup
    Effect::new(move |_| {
        update_breakpoint();
    });

    // Set up media query listeners for responsive changes
    Effect::new(move |_| {
        if let Some(window) = window() {
            // Mobile breakpoint (max-width: 639px)
            if let Ok(Some(query)) = window.match_media("(max-width: 639px)") {
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    update_breakpoint();
                }) as Box<dyn Fn(_)>);

                let _ = query
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                closure.forget(); // Keep the closure alive
            }

            // Tablet breakpoint (min-width: 640px and max-width: 1023px)
            if let Ok(Some(query)) =
                window.match_media("(min-width: 640px) and (max-width: 1023px)")
            {
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    update_breakpoint();
                }) as Box<dyn Fn(_)>);

                let _ = query
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                closure.forget(); // Keep the closure alive
            }

            // Desktop breakpoint (min-width: 1024px)
            if let Ok(Some(query)) = window.match_media("(min-width: 1024px)") {
                let closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                    update_breakpoint();
                }) as Box<dyn Fn(_)>);

                let _ = query
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
                closure.forget(); // Keep the closure alive
            }
        }
    });

    (breakpoint, device_type)
}
