use pdfium_render::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use base64::prelude::*;

#[derive(Serialize)]
pub struct PageInfo {
    id: String,
    file_path: String,
    page_index: u16,
    thumbnail: String,
}

#[derive(Deserialize)]
pub struct PageInstruction {
    file_path: String,
    page_index: u16,
    rotation: i32,
}

fn get_pdfium() -> Result<Pdfium, String> {
    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library())
        .map_err(|e| format!("Gagal memuat PDFium: {}", e))?;
    Ok(Pdfium::new(bindings))
}

#[tauri::command]
pub fn load_pdf_pages(paths: Vec<String>) -> Result<Vec<PageInfo>, String> {
    let pdfium = get_pdfium()?;
    let mut results = Vec::new();

    for path in paths {
        let doc = pdfium.load_pdf_from_file(&path, None).map_err(|e| e.to_string())?;
        for (i, page) in doc.pages().iter().enumerate() {
            let render_config = PdfRenderConfig::new()
                .set_target_width(150)
                .set_maximum_height(200)
                .set_clear_color(PdfColor::WHITE);
            
            let mut thumb_base64 = String::new();
            if let Ok(bitmap) = page.render_with_config(&render_config) {
                if let Ok(image) = bitmap.as_image() {
                    let mut buf = Cursor::new(Vec::new());
                    if image.write_to(&mut buf, image::ImageFormat::Jpeg).is_ok() {
                        thumb_base64 = BASE64_STANDARD.encode(buf.into_inner());
                    }
                }
            }

            results.push(PageInfo {
                id: format!("{}_{}", path, i),
                file_path: path.clone(),
                page_index: i as u16,
                thumbnail: format!("data:image/jpeg;base64,{}", thumb_base64),
            });
        }
    }
    Ok(results)
}

#[tauri::command]
pub fn process_pdf(instructions: Vec<PageInstruction>, out_path: String) -> Result<String, String> {
    let pdfium = get_pdfium()?;
    let mut new_doc = pdfium.create_new_pdf().map_err(|e| e.to_string())?;

    for (i, inst) in instructions.iter().enumerate() {
        let mut doc = pdfium.load_pdf_from_file(&inst.file_path, None).map_err(|e| e.to_string())?;
        
        if let Ok(mut page) = doc.pages_mut().get(inst.page_index.into()) {
            let current_rot = page.rotation().map(|r| r.as_degrees()).unwrap_or(0.0);
            let new_rot = (current_rot as i32 + inst.rotation) % 360;
            let final_rot = match new_rot {
                90 | -270 => PdfPageRenderRotation::Degrees90,
                180 | -180 => PdfPageRenderRotation::Degrees180,
                270 | -90 => PdfPageRenderRotation::Degrees270,
                _ => PdfPageRenderRotation::None,
            };
            page.set_rotation(final_rot);
        }
        
        new_doc.pages_mut().copy_page_from_document(&doc, inst.page_index.into(), (i as u16).into()).map_err(|e| e.to_string())?;
    }

    new_doc.save_to_file(&out_path).map_err(|e| e.to_string())?;
    Ok(out_path)
}
