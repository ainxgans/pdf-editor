use pdfium_render::prelude::*;

#[tauri::command]
pub fn merge_pdfs(paths: Vec<String>, out_path: String) -> Result<String, String> {
    let pdfium = Pdfium::default();
    let mut doc = pdfium.create_new_pdf().map_err(|e| e.to_string())?;

    for path in paths {
        let src = pdfium.load_pdf_from_file(&path, None).map_err(|e| e.to_string())?;
        doc.pages_mut().append(&src).map_err(|e| e.to_string())?;
    }

    doc.save_to_file(&out_path).map_err(|e| e.to_string())?;
    Ok(out_path)
}
