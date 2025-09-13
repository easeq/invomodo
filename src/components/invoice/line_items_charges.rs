use super::{ChargeItem, DiscountItem, TaxItem};
use crate::components::ui::{
    AutocompleteConfig, AutocompleteGroup, AutocompleteItem, GroupedAutocomplete, StaticDataSource,
};

use leptos::prelude::*;
use phosphor_leptos::{Icon, MAGNIFYING_GLASS, X};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LineChargeItemKind {
    Tax(TaxItem),
    Discount(DiscountItem),
    Charge(ChargeItem),
}

impl AutocompleteItem for LineChargeItemKind {
    type Id = String;

    fn id(&self) -> Self::Id {
        match self {
            LineChargeItemKind::Tax(tax) => tax.id(),
            LineChargeItemKind::Discount(discount) => discount.id(),
            LineChargeItemKind::Charge(charge) => charge.id(),
        }
    }

    fn search_text(&self) -> String {
        match self {
            LineChargeItemKind::Tax(tax) => tax.search_text(),
            LineChargeItemKind::Discount(discount) => discount.search_text(),
            LineChargeItemKind::Charge(charge) => charge.search_text(),
        }
    }

    fn display_text(&self) -> String {
        match self {
            LineChargeItemKind::Tax(tax) => tax.display_text(),
            LineChargeItemKind::Discount(discount) => discount.display_text(),
            LineChargeItemKind::Charge(charge) => charge.display_text(),
        }
    }
}

/// Example component using the autocomplete
#[component]
pub fn LineItemCharges(
    taxes: ReadSignal<Vec<TaxItem>>,
    discounts: ReadSignal<Vec<DiscountItem>>,
    charges: ReadSignal<Vec<ChargeItem>>,
    on_select: Callback<LineChargeItemKind>,
    on_remove: Callback<LineChargeItemKind>,
) -> impl IntoView {
    let groups = vec![
        AutocompleteGroup {
            id: "taxes".to_string(),
            name: "Taxes".to_string(),
            items: taxes
                .get()
                .into_iter()
                .map(LineChargeItemKind::Tax)
                .collect::<Vec<_>>(),
        },
        AutocompleteGroup {
            id: "discounts".to_string(),
            name: "Discounts".to_string(),
            items: discounts
                .get()
                .into_iter()
                .map(LineChargeItemKind::Discount)
                .collect::<Vec<_>>(),
        },
        AutocompleteGroup {
            id: "other".to_string(),
            name: "Other Charges".to_string(),
            items: charges
                .get()
                .into_iter()
                .map(LineChargeItemKind::Charge)
                .collect::<Vec<_>>(),
        },
    ];

    let data_source = StaticDataSource::new(groups, false);
    let config = AutocompleteConfig {
        min_query_length: 1,
        max_results_per_group: Some(5),
        debounce_ms: 200.0,
        case_sensitive: false,
    };

    view! {
        <GroupedAutocomplete
            data_source=data_source
            config=config
            on_select=on_select
            on_remove=on_remove
            render=move |props, actions| {
                view! {
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-1" for="charges">
                            "Taxes, Discounts, Other Charges"
                        </label>
                        <div class="relative">
                            <input
                                class="form-input"
                                id="charges"
                                placeholder="Search for taxes, discounts, or other charges"
                                type="text"
                                prop:value=move || props.query.get()
                                on:input=move |ev| {
                                    actions.set_query.set(event_target_value(&ev));
                                }
                                on:focus=move |_| actions.set_open.set(true)
                                on:keydown=move |ev| actions.handle_key_down.run(ev)
                            />
                            <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
                                <span class="material-symbols-outlined text-gray-400">
                                    <Icon icon=MAGNIFYING_GLASS />
                                </span>
                            </div>

                            // Dropdown results
                            <Show when=move || {
                                props.is_open.get() && !props.results.get().is_empty()
                            }>
                                <div class="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none">
                                    <For
                                        each=move || props.results.get().into_iter().enumerate()
                                        key=|(_, group)| group.id.clone()
                                        children=move |(group_idx, group)| {
                                            view! {
                                                <div>
                                                    <div class="px-4 py-2 text-xs font-semibold text-gray-500 uppercase tracking-wide bg-gray-50">
                                                        {group.name.clone()}
                                                    </div>
                                                    <For
                                                        each={
                                                            let group_items = group.items.clone();
                                                            move || group_items.clone().into_iter().enumerate()
                                                        }
                                                        key=|(_, item)| item.id()
                                                        children=move |(item_idx, item)| {
                                                            let is_highlighted = props.highlighted_index
                                                                == Some((group_idx, item_idx));
                                                            let item_clone = item.clone();

                                                            view! {
                                                                <div
                                                                    class=format!(
                                                                        "px-4 py-2 cursor-pointer hover:bg-gray-100 {}",
                                                                        if is_highlighted {
                                                                            "bg-indigo-100 text-indigo-900"
                                                                        } else {
                                                                            "text-gray-900"
                                                                        },
                                                                    )
                                                                    on:click=move |_| {
                                                                        actions.select_item.run(item_clone.clone())
                                                                    }
                                                                >
                                                                    <div class="font-medium">{item.display_text()}</div>
                                                                </div>
                                                            }
                                                        }
                                                    />
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </Show>

                            // Loading indicator
                            <Show when=move || props.is_loading.get()>
                                <div class="absolute right-3 top-1/2 transform -translate-y-1/2">
                                    <div class="animate-spin h-4 w-4 border-2 border-indigo-500 border-t-transparent rounded-full"></div>
                                </div>
                            </Show>
                        </div>

                        // Selected items (tags)
                        <div class="mt-2 flex flex-wrap gap-2">
                            <For
                                each=move || props.selected_items.get()
                                key=|item| item.id()
                                children=move |item| {
                                    view! {
                                        <span class="bg-indigo-100 text-indigo-800 text-sm font-medium px-3 py-1 rounded-full flex items-center">
                                            {item.display_text()}
                                            <button
                                                class="ml-2 text-indigo-800 hover:text-indigo-900"
                                                on:click=move |_| actions.remove_item.run(item.clone())
                                            >
                                                <span class="material-symbols-outlined text-sm">
                                                    <Icon icon=X />
                                                </span>
                                            </button>
                                        </span>
                                    }
                                }
                            />
                        </div>
                    </div>
                }
                    .into_any()
            }
        />
    }
}
