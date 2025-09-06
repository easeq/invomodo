use base64::Engine;
use base64::engine::general_purpose;
use leptos::prelude::*;

use typst::foundations::Bytes;
use typst::text::Font;
use typst_pdf::PdfOptions;

use crate::shared::fonts::FONTS;
use crate::shared::typst::InMemoryWorld;

#[component]
fn PdfViewer(content: String) -> impl IntoView {
    let fonts: Vec<Font> = FONTS
        .iter()
        .filter_map(|data| Font::new(Bytes::new(*data), 0))
        .collect();

    let world = InMemoryWorld::new(content, fonts);

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Output to pdf
    let pdf_data = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    // Encode the PDF's binary data using the standard Base64 engine.
    let encoded = general_purpose::STANDARD.encode(pdf_data);

    // Create the data URL.
    let data_url = format!("data:application/pdf;base64,{}", encoded);

    // In a framework like Leptos, this is the correct way to build the element.
    view! {
        <div>
            <iframe src=data_url style="width:100%; height:90vh; border:none;" />
        </div>
    };
}
