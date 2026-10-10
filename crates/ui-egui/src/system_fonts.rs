//! The last-resort interface font for CJK text: one face already installed on this machine,
//! found through the same font database that serves documents ([`wordcraft_fonts::FontDb`]).
//!
//! The embedded faces (Inter, Source Sans, JetBrains Mono, craft-fonts) come first in every
//! family; this one only draws characters none of them has, such as Chinese interface text in a
//! build without craft-fonts. It is read at runtime and never embedded or shipped, and
//! `WORDCRAFT_SYSTEM_FONTS=0` turns it off.

#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, OnceLock};

#[cfg(not(target_arch = "wasm32"))]
use egui::FontData;

/// Han families to try, best first. The first two ship with Windows (the Traditional and the
/// Simplified interface font); the others cover macOS and stock Linux installs.
#[cfg(not(target_arch = "wasm32"))]
const CJK_FAMILIES: &[&str] = &[
    "Microsoft JhengHei",
    "Microsoft YaHei",
    "DengXian",
    "SimSun",
    "PingFang SC",
    "PingFang TC",
    "Hiragino Sans",
    "Noto Sans CJK SC",
    "Noto Sans CJK TC",
    "Noto Sans CJK JP",
    "Arial Unicode MS",
];

/// The installed Han fallback face, read once. `None` when it is turned off or no family has
/// U+4E2D 中. The UI installs it only when the build has no embedded Han faces of its own.
#[cfg(not(target_arch = "wasm32"))]
pub fn cjk() -> Option<Arc<FontData>> {
    static CACHE: OnceLock<Option<Arc<FontData>>> = OnceLock::new();
    CACHE.get_or_init(load).clone()
}

#[cfg(not(target_arch = "wasm32"))]
fn load() -> Option<Arc<FontData>> {
    if std::env::var_os("WORDCRAFT_SYSTEM_FONTS").is_some_and(|v| v == "0") {
        return None;
    }
    let db = wordcraft_fonts::FontDb::global();
    for family in CJK_FAMILIES {
        if !db.has_family(family) {
            continue;
        }
        let face = db.face(family, "Regular");
        if face.covers('中') {
            // One copy of the file the document fonts share; egui parses it with the same skrifa.
            let mut data = FontData::from_owned(face.data().to_vec());
            data.index = face.index();
            log::info!("interface font fallback: {} ({})", face.family, face.style);
            return Some(Arc::new(data));
        }
    }
    None
}