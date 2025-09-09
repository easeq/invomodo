use leptos::prelude::*;

use crate::components::ui::{Tab, TabPanel, TabsProvider};

#[component]
pub fn InternalTab(name: String, label: String) -> impl IntoView {
    view! {
        <Tab
            name=name.clone()
            render=move |is_sel, select| {
                view! {
                    <button
                        class="px-4 py-3 text-sm font-semibold border-b-2 border-[var(--primary-color)] text-[var(--primary-color)]"
                        on:click=move |_| select()
                        style:cursor="pointer"
                        style:font-weight=move || { if is_sel.get() { "bold" } else { "normal" } }
                    >
                        {label.clone()}
                    </button>
                }
            }
        />
    }
}

#[component]
pub fn InvoiceBuilder() -> impl IntoView {
    view! {
        <div class="lg:col-span-2 space-y-6">
            <TabsProvider default="builder".to_string()>
                <div class="bg-[var(--background-secondary)] rounded-md border border-[var(--border-color)]">
                    <div class="border-b border-[var(--border-color)]">
                        <div class="flex px-4">
                            <InternalTab name="builder".to_string() label="Builder".to_string() />
                            <InternalTab name="taxes".to_string() label="Taxes".to_string() />
                            <InternalTab
                                name="discounts".to_string()
                                label="Discounts".to_string()
                            />
                            <InternalTab
                                name="other_charges".to_string()
                                label="Other Charges".to_string()
                            />
                            <InternalTab
                                name="custom_fields".to_string()
                                label="Custom Fields".to_string()
                            />
                            <InternalTab name="preview".to_string() label="Preview".to_string() />
                        </div>
                    </div>
                    <TabPanel name="builder".to_string()>
                        <div class="p-6 space-y-6">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div>
                                    <label class="form-label" for="template">
                                        Template
                                    </label>
                                    <select class="form-input" id="template" name="template">
                                        <option>Default Template</option>
                                        <option>Modern</option>
                                        <option>Classic</option>
                                    </select>
                                </div>
                                <div>
                                    <label class="form-label" for="locale">
                                        Locale
                                    </label>
                                    <select class="form-input" id="locale" name="locale">
                                        <option>English (US)</option>
                                        <option>Français (FR)</option>
                                        <option>Español (ES)</option>
                                    </select>
                                </div>
                            </div>
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                                <div>
                                    <label class="form-label" for="invoice-number">
                                        Invoice Number
                                    </label>
                                    <input
                                        class="form-input"
                                        id="invoice-number"
                                        name="invoice-number"
                                        type="text"
                                        value="INV-2024-001"
                                    />
                                </div>
                                <div>
                                    <label class="form-label" for="issue-date">
                                        Issue Date
                                    </label>
                                    <input
                                        class="form-input"
                                        id="issue-date"
                                        name="issue-date"
                                        type="date"
                                        value="2024-07-28"
                                    />
                                </div>
                                <div>
                                    <label class="form-label" for="due-date">
                                        Due Date
                                    </label>
                                    <input
                                        class="form-input"
                                        id="due-date"
                                        name="due-date"
                                        type="date"
                                        value="2024-08-27"
                                    />
                                </div>
                            </div>
                            <div>
                                <label class="form-label" for="reference-number">
                                    Reference Number
                                </label>
                                <input
                                    class="form-input"
                                    id="reference-number"
                                    name="reference-number"
                                    placeholder="Optional project code or PO number"
                                    type="text"
                                />
                            </div>
                        </div>
                    </TabPanel>
                    <TabPanel name="taxes".to_string()>
                        <section>taxes</section>
                    </TabPanel>
                    <TabPanel name="discounts".to_string()>
                        <section>discounts</section>
                    </TabPanel>
                    <TabPanel name="other_charges".to_string()>
                        <section>other_charges</section>
                    </TabPanel>
                    <TabPanel name="custom_fields".to_string()>
                        <section>custom_fields</section>
                    </TabPanel>
                    <TabPanel name="preview".to_string()>
                        <section>preview</section>
                    </TabPanel>
                </div>
            </TabsProvider>
        </div>
    }
}
