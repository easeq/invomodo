use leptos::prelude::*;
use leptos_use::watch_debounced;
use std::collections::HashSet;
use std::hash::Hash;

/// Trait for items that can be searched and displayed in the autocomplete
pub trait AutocompleteItem: Clone + PartialEq + Eq + Hash {
    type Id: Clone + PartialEq + Hash;

    fn id(&self) -> Self::Id;
    fn search_text(&self) -> String;
    fn display_text(&self) -> String;
}

/// Represents a group of autocomplete items
#[derive(Clone, Debug)]
pub struct AutocompleteGroup<T> {
    pub id: String,
    pub name: String,
    pub items: Vec<T>,
}

/// Configuration for the autocomplete component
pub struct AutocompleteConfig {
    pub min_query_length: usize,
    pub max_results_per_group: Option<usize>,
    pub debounce_ms: f64,
    pub case_sensitive: bool,
}

impl Default for AutocompleteConfig {
    fn default() -> Self {
        Self {
            min_query_length: 1,
            max_results_per_group: None,
            debounce_ms: 300.0,
            case_sensitive: false,
        }
    }
}

/// Data source trait for providing autocomplete data
pub trait AutocompleteDataSource<T: AutocompleteItem + 'static> {
    fn search(&self, query: &str) -> Vec<AutocompleteGroup<T>>;
}

/// Async data source trait
pub trait AsyncAutocompleteDataSource<T: AutocompleteItem + 'static> {
    fn search_async(
        &self,
        query: &str,
    ) -> impl std::future::Future<Output = Vec<AutocompleteGroup<T>>> + Send;
}

/// Simple in-memory data source
#[derive(Clone)]
pub struct StaticDataSource<T> {
    groups: Vec<AutocompleteGroup<T>>,
    case_sensitive: bool,
}

impl<T: AutocompleteItem> StaticDataSource<T> {
    pub fn new(groups: Vec<AutocompleteGroup<T>>, case_sensitive: bool) -> Self {
        Self {
            groups,
            case_sensitive,
        }
    }
}

impl<T: AutocompleteItem + 'static> AutocompleteDataSource<T> for StaticDataSource<T> {
    fn search(&self, query: &str) -> Vec<AutocompleteGroup<T>> {
        if query.is_empty() {
            return self.groups.clone();
        }

        let search_query = if self.case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        self.groups
            .iter()
            .filter_map(|group| {
                let filtered_items: Vec<T> = group
                    .items
                    .iter()
                    .filter(|item| {
                        let item_text = if self.case_sensitive {
                            item.search_text()
                        } else {
                            item.search_text().to_lowercase()
                        };
                        item_text.contains(&search_query)
                    })
                    .cloned()
                    .collect();

                if filtered_items.is_empty() {
                    None
                } else {
                    Some(AutocompleteGroup {
                        id: group.id.clone(),
                        name: group.name.clone(),
                        items: filtered_items,
                    })
                }
            })
            .collect()
    }
}

/// Signal-based data source for reactive data
pub struct SignalDataSource<T> {
    signal: ArcReadSignal<Vec<AutocompleteGroup<T>>>,
    case_sensitive: bool,
}

impl<T: AutocompleteItem + 'static> SignalDataSource<T> {
    pub fn new(signal: ArcReadSignal<Vec<AutocompleteGroup<T>>>, case_sensitive: bool) -> Self {
        Self {
            signal,
            case_sensitive,
        }
    }
}

impl<T: AutocompleteItem + 'static> AutocompleteDataSource<T> for SignalDataSource<T> {
    fn search(&self, query: &str) -> Vec<AutocompleteGroup<T>> {
        let groups = self.signal.get();
        StaticDataSource::new(groups, self.case_sensitive).search(query)
    }
}

/// Main autocomplete component props
#[derive(Clone)]
pub struct AutocompleteRenderProps<T: AutocompleteItem> {
    pub query: ReadSignal<String>,
    pub results: ReadSignal<Vec<AutocompleteGroup<T>>>,
    pub selected_items: ReadSignal<HashSet<T>>,
    pub is_open: ReadSignal<bool>,
    pub is_loading: ReadSignal<bool>,
    pub highlighted_index: Option<(usize, usize)>, // (group_index, item_index)
}

/// Actions available to the render function
#[derive(Clone)]
pub struct AutocompleteActions<T: AutocompleteItem + 'static> {
    pub set_query: WriteSignal<String>,
    pub select_item: Callback<T>,
    pub remove_item: Callback<T>,
    pub clear_all: Callback<()>,
    pub set_open: WriteSignal<bool>,
    pub set_highlighted: WriteSignal<Option<(usize, usize)>>,
    pub handle_key_down: Callback<web_sys::KeyboardEvent>,
}

/// The main headless autocomplete component
#[component]
pub fn GroupedAutocomplete<T, F, DS, R>(
    /// Data source for the autocomplete
    data_source: DS,
    /// Render function that receives props and actions
    render: F,
    /// Configuration options
    #[prop(optional)]
    config: Option<AutocompleteConfig>,
    /// Initial selected items
    #[prop(optional)]
    initial_selected: Option<HashSet<T>>,
    /// Callback when selection changes
    #[prop(optional)]
    on_selection_change: Option<Callback<HashSet<T>>>,
) -> impl IntoView
where
    T: AutocompleteItem + Send + Sync + 'static,
    F: Fn(AutocompleteRenderProps<T>, AutocompleteActions<T>) -> R + 'static,
    DS: AutocompleteDataSource<T> + Clone + 'static,
    R: IntoView + 'static,
{
    let config = config.unwrap_or_default();
    let initial_selected = initial_selected.unwrap_or_default();

    let (query, set_query) = signal(String::new());
    let (selected_items, set_selected_items) = signal(initial_selected);
    let (is_open, set_open) = signal(false);
    let (is_loading, set_loading) = signal(false);
    let (highlighted_index, set_highlighted) = signal(None::<(usize, usize)>);
    let (results, set_results) = signal(Vec::<AutocompleteGroup<T>>::new());

    // Debounced search effect
    let _ = watch_debounced(
        move || query.get(),
        move |current_query, _, _| {
            if current_query.len() < config.min_query_length {
                set_results.set(Vec::new());
                set_open.set(false);
                return;
            }

            set_loading.set(true);

            // Run the search
            let search_results = data_source.search(current_query);
            let limited_results = if let Some(max_per_group) = config.max_results_per_group {
                search_results
                    .into_iter()
                    .map(|mut group| {
                        group.items.truncate(max_per_group);
                        group
                    })
                    .collect()
            } else {
                search_results
            };

            set_results.set(limited_results);
            set_loading.set(false);
            set_open.set(true);
        },
        config.debounce_ms, // debounce delay in ms
    );

    // Handle selection changes
    Effect::new(move |_| {
        let items = selected_items.get();
        if let Some(callback) = &on_selection_change {
            callback.run(items);
            set_query.set("".to_string());
        }
    });

    let select_item = Callback::new(move |item: T| {
        set_selected_items.update(|items| {
            items.insert(item);
        });
        set_query.set(String::new());
        set_open.set(false);
        set_highlighted.set(None);
    });

    let remove_item = Callback::new(move |item: T| {
        set_selected_items.update(|items| {
            items.remove(&item.clone());
        });
    });

    let clear_all = Callback::new(move |_| {
        set_selected_items.set(HashSet::new());
    });

    let handle_key_down = Callback::new({
        move |event: web_sys::KeyboardEvent| {
            let key = event.key();
            let current_results = results.get();
            let current_highlighted = highlighted_index.get();

            match key.as_str() {
                "ArrowDown" => {
                    event.prevent_default();
                    let next_index = match current_highlighted {
                        None => Some((0, 0)),
                        Some((group_idx, item_idx)) => {
                            if group_idx < current_results.len() {
                                let current_group = &current_results[group_idx];
                                if item_idx + 1 < current_group.items.len() {
                                    Some((group_idx, item_idx + 1))
                                } else if group_idx + 1 < current_results.len() {
                                    Some((group_idx + 1, 0))
                                } else {
                                    Some((0, 0)) // Wrap to beginning
                                }
                            } else {
                                Some((0, 0))
                            }
                        }
                    };
                    set_highlighted.set(next_index);
                }
                "ArrowUp" => {
                    event.prevent_default();
                    let prev_index = match current_highlighted {
                        None => current_results.last().map(|last_group| {
                            (
                                current_results.len() - 1,
                                last_group.items.len().saturating_sub(1),
                            )
                        }),
                        Some((group_idx, item_idx)) => {
                            if item_idx > 0 {
                                Some((group_idx, item_idx - 1))
                            } else if group_idx > 0 {
                                let prev_group = &current_results[group_idx - 1];
                                Some((group_idx - 1, prev_group.items.len().saturating_sub(1)))
                            } else {
                                current_results.last().map(|last_group| {
                                    (
                                        current_results.len() - 1,
                                        last_group.items.len().saturating_sub(1),
                                    )
                                })
                            }
                        }
                    };
                    set_highlighted.set(prev_index);
                }
                "Enter" => {
                    event.prevent_default();
                    if let Some((group_idx, item_idx)) = current_highlighted {
                        if let Some(group) = current_results.get(group_idx) {
                            if let Some(item) = group.items.get(item_idx) {
                                select_item.run(item.clone());
                            }
                        }
                    }
                }
                "Escape" => {
                    event.prevent_default();
                    set_open.set(false);
                    set_highlighted.set(None);
                }
                _ => {}
            }
        }
    });

    let render_props = AutocompleteRenderProps {
        query,
        results,
        selected_items,
        is_open,
        is_loading,
        highlighted_index: highlighted_index.get(),
    };

    let actions = AutocompleteActions {
        set_query,
        select_item,
        remove_item,
        clear_all,
        set_open,
        set_highlighted,
        handle_key_down,
    };

    render(render_props, actions)
}
