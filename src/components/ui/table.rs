use leptos::html::*;
use leptos::prelude::*;

// Table Components
#[component]
pub fn Table(#[prop(into, optional)] class: Option<String>, children: Children) -> impl IntoView {
    view! {
        <div class="relative w-full overflow-auto">
            <table class=format!("w-full caption-bottom text-sm {}", class.unwrap_or_default())>
                {children()}
            </table>
        </div>
    }
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    view! {
        <thead class="[&_tr]:border-b">
            {children()}
        </thead>
    }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! {
        <tbody class="[&_tr:last-child]:border-0">
            {children()}
        </tbody>
    }
}

#[component]
pub fn TableRow(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr class=format!("border-b transition-colors hover:bg-gray-50/50 data-[state=selected]:bg-gray-50 {}", class.unwrap_or_default())>
            {children()}
        </tr>
    }
}

#[component]
pub fn TableHead(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <th class=format!("h-12 px-4 text-left align-middle font-medium text-gray-500 [&:has([role=checkbox])]:pr-0 {}", class.unwrap_or_default())>
            {children()}
        </th>
    }
}

#[component]
pub fn TableCell(
    #[prop(into, optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <td class=format!("p-4 align-middle [&:has([role=checkbox])]:pr-0 {}", class.unwrap_or_default())>
            {children()}
        </td>
    }
}
