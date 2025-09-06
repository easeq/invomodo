use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, MediaQueryListEvent, window};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScreenSize {
    Mobile,  // < 768px (md breakpoint)
    Tablet,  // 768px - 1024px (md to lg)
    Desktop, // >= 1024px (lg breakpoint)
}

impl ScreenSize {
    pub fn is_mobile(&self) -> bool {
        matches!(self, ScreenSize::Mobile)
    }

    pub fn is_tablet(&self) -> bool {
        matches!(self, ScreenSize::Tablet)
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, ScreenSize::Desktop)
    }

    pub fn is_mobile_or_tablet(&self) -> bool {
        matches!(self, ScreenSize::Mobile | ScreenSize::Tablet)
    }
}

fn get_screen_size() -> ScreenSize {
    if let Some(window) = window() {
        if let Ok(width) = window.inner_width() {
            if let Some(width_num) = width.as_f64() {
                if width_num < 768.0 {
                    return ScreenSize::Mobile;
                } else if width_num < 1024.0 {
                    return ScreenSize::Tablet;
                } else {
                    return ScreenSize::Desktop;
                }
            }
        }
    }
    // Default fallback
    ScreenSize::Desktop
}

pub fn use_responsive() -> ReadSignal<ScreenSize> {
    let (screen_size, set_screen_size) = signal(get_screen_size());

    // Set up resize listener
    Effect::new(move |_| {
        let resize_closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
            set_screen_size.set(get_screen_size());
        }) as Box<dyn FnMut(_)>);

        if let Some(window) = window() {
            let _ = window.add_event_listener_with_callback(
                "resize",
                resize_closure.as_ref().unchecked_ref(),
            );
        }

        // Store closure to prevent it from being dropped
        std::mem::forget(resize_closure);
    });

    screen_size
}

// Helper hook for specific breakpoint queries

pub fn use_media_query(query: &'static str) -> ReadSignal<bool> {
    let (matches, set_matches) = signal(false);

    Effect::new(move |_| {
        if let Some(window) = window() {
            if let Ok(Some(mql)) = window.match_media(query) {
                set_matches.set(mql.matches());

                let change_closure = Closure::wrap(Box::new(move |event: MediaQueryListEvent| {
                    set_matches.set(event.matches());
                }) as Box<dyn FnMut(_)>);

                // Convert mql to EventTarget to use add_event_listener
                let target: &EventTarget = mql.as_ref();
                if target
                    .add_event_listener_with_callback(
                        "change",
                        change_closure.as_ref().unchecked_ref(),
                    )
                    .is_ok()
                {
                    // Prevent the closure from being dropped
                    change_closure.forget();
                }
            }
        }
    });

    matches
}

// Predefined media queries for common breakpoints
pub mod breakpoints {
    use super::use_media_query;
    use leptos::prelude::*;

    pub fn use_is_mobile() -> ReadSignal<bool> {
        use_media_query("(max-width: 767px)")
    }

    pub fn use_is_tablet() -> ReadSignal<bool> {
        use_media_query("(min-width: 768px) and (max-width: 1023px)")
    }

    pub fn use_is_desktop() -> ReadSignal<bool> {
        use_media_query("(min-width: 1024px)")
    }

    pub fn use_is_large_screen() -> ReadSignal<bool> {
        use_media_query("(min-width: 1280px)")
    }
}
