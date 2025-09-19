use leptos::prelude::*;

use crate::components::invoice::InvoiceBuilder as InvoiceBuilderView;

#[component]
pub fn InvoiceBuilder() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow-sm border-b border-gray-200">
                <div class="px-4 sm:px-6 lg:px-8">
                    <div class="flex h-16 items-center justify-between">
                        <div class="flex items-center space-x-8">
                            <span class="text-lg font-semibold text-gray-800">Invoice Builder</span>
                        // <nav class="flex space-x-4">
                        // <a href="#" class="text-sm text-gray-600 hover:text-gray-900">
                        // Dashboard
                        // </a>
                        // <a href="#" class="text-sm text-gray-600 hover:text-gray-900">
                        // Invoices
                        // </a>
                        // <a href="#" class="text-sm text-gray-600 hover:text-gray-900">
                        // Settings
                        // </a>
                        // </nav>
                        </div>

                    // <div>
                    // <a href="#" class="text-sm text-blue-600 hover:underline">
                    // Login
                    // </a>
                    // </div>
                    </div>
                </div>
            </header>

            <div class="lg:col-span-2 space-y-6 p-4">
                <InvoiceBuilderView />
            </div>
        </div>
    }
}
