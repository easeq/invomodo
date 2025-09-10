use leptos::prelude::*;

#[component]
pub fn Builder() -> impl IntoView {
    view! {
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
    }
}
