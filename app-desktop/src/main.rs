use cstr::cstr;
use image::RgbaImage;
use base64::Engine as _;
use pdf_annotations::{AddAnnotationCommand, Annotation, AnnotationKind, Color, Rect};
use pdf_core::{CommandHistory, Document, DocumentCommand, OcrResult, TextRegion};
use pdf_editor::{
    ApplyOcrCommand, DeletePageCommand, FontSubstitutionCommand, InsertTextCommand,
    ModifyTextCommand, RedactRegionCommand, RotatePageCommand, SetPasswordCommand,
};
use pdf_render::{RenderEngine, SoftwareRenderer};
#[cfg(feature = "mupdf")]
use pdf_render::MuPdfRenderer;
use qmetaobject::prelude::*;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use sys_locale::get_locale;
use tracing::info;

#[derive(Debug)]
struct AppState {
    document: Option<Document>,
    history: CommandHistory,
    current_page: u32,
    zoom: f32,
    render_nonce: u64,
    recent_documents: Vec<PathBuf>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            document: None,
            history: CommandHistory::new(128),
            current_page: 0,
            zoom: 1.0,
            render_nonce: 0,
            recent_documents: load_recent_documents(),
        }
    }
}

fn state() -> &'static Mutex<AppState> {
    static STATE: OnceLock<Mutex<AppState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(AppState::default()))
}

fn recent_store_path() -> Option<PathBuf> {
    if let Ok(appdata) = std::env::var("APPDATA") {
        let dir = PathBuf::from(appdata).join("free-pdf-editor");
        let _ = std::fs::create_dir_all(&dir);
        return Some(dir.join("recent.txt"));
    }
    std::env::current_dir().ok().map(|d| d.join("recent.txt"))
}

fn load_recent_documents() -> Vec<PathBuf> {
    let Some(path) = recent_store_path() else {
        return Vec::new();
    };
    let Ok(data) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for line in data.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let p = PathBuf::from(trimmed);
        if p.exists() && !out.iter().any(|x| x == &p) {
            out.push(p);
        }
        if out.len() >= 5 {
            break;
        }
    }
    out
}

fn save_recent_documents(paths: &[PathBuf]) {
    let Some(path) = recent_store_path() else {
        return;
    };
    let mut lines = String::new();
    for p in paths.iter().take(5) {
        lines.push_str(&p.to_string_lossy());
        lines.push('\n');
    }
    let _ = std::fs::write(path, lines);
}

#[derive(Clone, Copy)]
enum Lang {
    En,
    Ru,
    Cs,
    Pl,
    Hu,
    Es,
    Fr,
    De,
}

fn current_lang() -> Lang {
    static LANG: OnceLock<Lang> = OnceLock::new();
    *LANG.get_or_init(|| {
        let locale = get_locale().unwrap_or_else(|| "en".to_owned()).to_lowercase();
        let code = locale
            .split(['-', '_'])
            .next()
            .unwrap_or("en");
        match code {
            "ru" => Lang::Ru,
            "cs" | "cz" => Lang::Cs,
            "pl" => Lang::Pl,
            "hu" => Lang::Hu,
            "es" => Lang::Es,
            "fr" => Lang::Fr,
            "de" => Lang::De,
            _ => Lang::En,
        }
    })
}

fn tr(key: &str) -> &'static str {
    match current_lang() {
        Lang::Ru => match key {
            "ready" => "Готово",
            "error" => "Ошибка",
            "open_first" => "Сначала откройте документ",
            "document_closed" => "Документ закрыт",
            "no_recent_slot" => "В этом слоте нет недавнего файла",
            "recent_not_found" => "Недавний файл не найден",
            "undo" => "Отмена",
            "redo" => "Повтор",
            "rendered_page" => "Отрисована страница",
            "opened" => "Открыт",
            "saved" => "Сохранено",
            "saved_as" => "Сохранено как",
            "opened_recent" => "Открыт недавний",
            "not_implemented_qt" => "пока не реализовано в Qt",
            "page_rotated" => "Страница повернута",
            "page_deleted" => "Страница удалена",
            "inserted_text" => "Текст вставлен",
            "modified_text" => "Текст изменен",
            "font_subst" => "Замена шрифта применена",
            "added_highlight" => "Выделение добавлено",
            "added_note" => "Заметка добавлена",
            "ocr_applied" => "OCR слой применен",
            "redaction_applied" => "Редакция применена",
            "password_set" => "Пароль документа установлен",
            "page" => "Страница",
            "zoom" => "Масштаб",
            _ => "",
        },
        Lang::Cs => match key {
            "ready" => "Pripraveno",
            "error" => "Chyba",
            "open_first" => "Nejdriv otevrete dokument",
            "document_closed" => "Dokument zavren",
            "no_recent_slot" => "V tomto slotu neni nedavny soubor",
            "recent_not_found" => "Nedavny soubor nenalezen",
            "undo" => "Zpet",
            "redo" => "Znovu",
            "rendered_page" => "Vykreslena stranka",
            "opened" => "Otevreno",
            "saved" => "Ulozeno",
            "saved_as" => "Ulozeno jako",
            "opened_recent" => "Otevren nedavny",
            "not_implemented_qt" => "zatim neni implementovano v Qt",
            "page_rotated" => "Stranka otocena",
            "page_deleted" => "Stranka smazana",
            "inserted_text" => "Text vlozen",
            "modified_text" => "Text upraven",
            "font_subst" => "Nahrazeni pisma provedeno",
            "added_highlight" => "Zvyrazneni pridano",
            "added_note" => "Poznamka pridana",
            "ocr_applied" => "OCR vrstva aplikovana",
            "redaction_applied" => "Redakce aplikovana",
            "password_set" => "Heslo dokumentu nastaveno",
            "page" => "Stranka",
            "zoom" => "Priblizeni",
            _ => "",
        },
        Lang::Pl => match key {
            "ready" => "Gotowe",
            "error" => "Blad",
            "open_first" => "Najpierw otworz dokument",
            "document_closed" => "Dokument zamkniety",
            "no_recent_slot" => "Brak ostatniego pliku w tym slocie",
            "recent_not_found" => "Nie znaleziono ostatniego pliku",
            "undo" => "Cofnij",
            "redo" => "Ponow",
            "rendered_page" => "Wyrenderowano strone",
            "opened" => "Otwarto",
            "saved" => "Zapisano",
            "saved_as" => "Zapisano jako",
            "opened_recent" => "Otwarto ostatni",
            "not_implemented_qt" => "jeszcze nie zaimplementowano w Qt",
            "page_rotated" => "Strona obrocona",
            "page_deleted" => "Strona usunieta",
            "inserted_text" => "Wstawiono tekst",
            "modified_text" => "Zmieniono tekst",
            "font_subst" => "Zastosowano podmiane czcionki",
            "added_highlight" => "Dodano podkreslenie",
            "added_note" => "Dodano notatke",
            "ocr_applied" => "Zastosowano warstwe OCR",
            "redaction_applied" => "Zastosowano redakcje",
            "password_set" => "Ustawiono haslo dokumentu",
            "page" => "Strona",
            "zoom" => "Powiekszenie",
            _ => "",
        },
        Lang::Hu => match key {
            "ready" => "Kesz",
            "error" => "Hiba",
            "open_first" => "Eloszor nyisson meg egy dokumentumot",
            "document_closed" => "Dokumentum bezarva",
            "no_recent_slot" => "Nincs legutobbi fajl ebben a helyben",
            "recent_not_found" => "Legutobbi fajl nem talalhato",
            "undo" => "Visszavonas",
            "redo" => "Ujra",
            "rendered_page" => "Oldal renderelve",
            "opened" => "Megnyitva",
            "saved" => "Mentve",
            "saved_as" => "Mentve maskepp",
            "opened_recent" => "Legutobbi megnyitva",
            "not_implemented_qt" => "meg nincs megvalositva Qt alatt",
            "page_rotated" => "Oldal elforgatva",
            "page_deleted" => "Oldal torolve",
            "inserted_text" => "Szoveg beszurva",
            "modified_text" => "Szoveg modositva",
            "font_subst" => "Betutipus csere alkalmazva",
            "added_highlight" => "Kiemeles hozzaadva",
            "added_note" => "Jegyzet hozzaadva",
            "ocr_applied" => "OCR reteg alkalmazva",
            "redaction_applied" => "Kitakaras alkalmazva",
            "password_set" => "Dokumentum jelszo beallitva",
            "page" => "Oldal",
            "zoom" => "Nagyitas",
            _ => "",
        },
        Lang::Es => match key {
            "ready" => "Listo",
            "error" => "Error",
            "open_first" => "Primero abra un documento",
            "document_closed" => "Documento cerrado",
            "no_recent_slot" => "No hay archivo reciente en esta posicion",
            "recent_not_found" => "Archivo reciente no encontrado",
            "undo" => "Deshacer",
            "redo" => "Rehacer",
            "rendered_page" => "Pagina renderizada",
            "opened" => "Abierto",
            "saved" => "Guardado",
            "saved_as" => "Guardado como",
            "opened_recent" => "Reciente abierto",
            "not_implemented_qt" => "todavia no implementado en Qt",
            "page_rotated" => "Pagina girada",
            "page_deleted" => "Pagina eliminada",
            "inserted_text" => "Texto insertado",
            "modified_text" => "Texto modificado",
            "font_subst" => "Sustitucion de fuente aplicada",
            "added_highlight" => "Resaltado agregado",
            "added_note" => "Nota agregada",
            "ocr_applied" => "Capa OCR aplicada",
            "redaction_applied" => "Redaccion aplicada",
            "password_set" => "Contrasena del documento establecida",
            "page" => "Pagina",
            "zoom" => "Zoom",
            _ => "",
        },
        Lang::Fr => match key {
            "ready" => "Pret",
            "error" => "Erreur",
            "open_first" => "Ouvrez d'abord un document",
            "document_closed" => "Document ferme",
            "no_recent_slot" => "Aucun fichier recent dans cet emplacement",
            "recent_not_found" => "Fichier recent introuvable",
            "undo" => "Annuler",
            "redo" => "Retablir",
            "rendered_page" => "Page rendue",
            "opened" => "Ouvert",
            "saved" => "Enregistre",
            "saved_as" => "Enregistre sous",
            "opened_recent" => "Recent ouvert",
            "not_implemented_qt" => "pas encore implemente dans Qt",
            "page_rotated" => "Page pivotee",
            "page_deleted" => "Page supprimee",
            "inserted_text" => "Texte insere",
            "modified_text" => "Texte modifie",
            "font_subst" => "Substitution de police appliquee",
            "added_highlight" => "Surlignage ajoute",
            "added_note" => "Note ajoutee",
            "ocr_applied" => "Couche OCR appliquee",
            "redaction_applied" => "Redaction appliquee",
            "password_set" => "Mot de passe du document defini",
            "page" => "Page",
            "zoom" => "Zoom",
            _ => "",
        },
        Lang::De => match key {
            "ready" => "Bereit",
            "error" => "Fehler",
            "open_first" => "Offnen Sie zuerst ein Dokument",
            "document_closed" => "Dokument geschlossen",
            "no_recent_slot" => "Kein letzter Datei in diesem Feld",
            "recent_not_found" => "Letzte Datei nicht gefunden",
            "undo" => "Ruckgangig",
            "redo" => "Wiederholen",
            "rendered_page" => "Seite gerendert",
            "opened" => "Geoffnet",
            "saved" => "Gespeichert",
            "saved_as" => "Gespeichert unter",
            "opened_recent" => "Letzte Datei geoffnet",
            "not_implemented_qt" => "noch nicht in Qt implementiert",
            "page_rotated" => "Seite gedreht",
            "page_deleted" => "Seite geloscht",
            "inserted_text" => "Text eingefugt",
            "modified_text" => "Text geandert",
            "font_subst" => "Schriftart-Ersetzung angewendet",
            "added_highlight" => "Markierung hinzugefugt",
            "added_note" => "Notiz hinzugefugt",
            "ocr_applied" => "OCR-Ebene angewendet",
            "redaction_applied" => "Schwarzung angewendet",
            "password_set" => "Dokumentpasswort gesetzt",
            "page" => "Seite",
            "zoom" => "Zoom",
            _ => "",
        },
        _ => match key {
            "ready" => "Ready",
            "error" => "Error",
            "open_first" => "Open a document first",
            "document_closed" => "Document closed",
            "no_recent_slot" => "No recent file in this slot",
            "recent_not_found" => "Recent file not found",
            "undo" => "Undo",
            "redo" => "Redo",
            "rendered_page" => "Rendered page",
            "opened" => "Opened",
            "saved" => "Saved",
            "saved_as" => "Saved As",
            "opened_recent" => "Opened recent",
            "not_implemented_qt" => "not implemented in Qt yet",
            "page_rotated" => "Page rotated",
            "page_deleted" => "Page deleted",
            "inserted_text" => "Inserted text",
            "modified_text" => "Modified text",
            "font_subst" => "Applied font substitution",
            "added_highlight" => "Added highlight",
            "added_note" => "Added note",
            "ocr_applied" => "Applied OCR layer",
            "redaction_applied" => "Applied redaction",
            "password_set" => "Document password set",
            "page" => "Page",
            "zoom" => "Zoom",
            _ => "",
        },
    }
}

fn localize_status(msg: &str) -> String {
    if let Some(rest) = msg.strip_prefix("Error: ") {
        return format!("{}: {}", tr("error"), rest);
    }
    if msg == "Open a document first" {
        return tr("open_first").to_owned();
    }
    if msg == "Document closed" {
        return tr("document_closed").to_owned();
    }
    if msg == "No recent file in this slot" {
        return tr("no_recent_slot").to_owned();
    }
    if let Some(rest) = msg.strip_prefix("Recent file not found: ") {
        return format!("{}: {}", tr("recent_not_found"), rest);
    }
    if msg == "Undo" {
        return tr("undo").to_owned();
    }
    if msg == "Redo" {
        return tr("redo").to_owned();
    }
    if let Some(rest) = msg.strip_prefix("Rendered page ") {
        return format!("{} {}", tr("rendered_page"), rest);
    }
    if let Some(rest) = msg.strip_prefix("Opened: ") {
        return format!("{}: {}", tr("opened"), rest);
    }
    if let Some(rest) = msg.strip_prefix("Saved: ") {
        return format!("{}: {}", tr("saved"), rest);
    }
    if let Some(rest) = msg.strip_prefix("Saved As: ") {
        return format!("{}: {}", tr("saved_as"), rest);
    }
    if let Some(rest) = msg.strip_prefix("Opened recent: ") {
        return format!("{}: {}", tr("opened_recent"), rest);
    }
    if let Some(rest) = msg.strip_suffix(": not implemented in Qt yet") {
        return format!("{}: {}", rest, tr("not_implemented_qt"));
    }
    match msg {
        "Page rotated" => tr("page_rotated").to_owned(),
        "Page deleted" => tr("page_deleted").to_owned(),
        "Inserted text" => tr("inserted_text").to_owned(),
        "Modified text" => tr("modified_text").to_owned(),
        "Applied font substitution" => tr("font_subst").to_owned(),
        "Added highlight" => tr("added_highlight").to_owned(),
        "Added note" => tr("added_note").to_owned(),
        "Applied OCR layer" => tr("ocr_applied").to_owned(),
        "Applied redaction" => tr("redaction_applied").to_owned(),
        "Document password set" => tr("password_set").to_owned(),
        _ => msg.to_owned(),
    }
}

#[derive(QObject)]
struct AppBridge {
    base: qt_base_class!(trait QObject),
    status: qt_property!(QString; NOTIFY status_changed),
    status_changed: qt_signal!(),
    page_source: qt_property!(QString; NOTIFY page_source_changed),
    page_source_changed: qt_signal!(),
    page_info: qt_property!(QString; NOTIFY page_info_changed),
    page_info_changed: qt_signal!(),
    zoom_info: qt_property!(QString; NOTIFY zoom_info_changed),
    zoom_info_changed: qt_signal!(),
    zoom_level: qt_property!(f32; NOTIFY zoom_level_changed),
    zoom_level_changed: qt_signal!(),
    recent_a: qt_property!(QString; NOTIFY recent_changed),
    recent_b: qt_property!(QString; NOTIFY recent_changed),
    recent_c: qt_property!(QString; NOTIFY recent_changed),
    recent_d: qt_property!(QString; NOTIFY recent_changed),
    recent_e: qt_property!(QString; NOTIFY recent_changed),
    recent_changed: qt_signal!(),

    open_document: qt_method!(fn open_document(&mut self) { self.open_document_impl(); }),
    save_document: qt_method!(fn save_document(&mut self) { self.save_document_impl(); }),
    save_document_as: qt_method!(fn save_document_as(&mut self) { self.save_document_as_impl(); }),
    close_document: qt_method!(fn close_document(&mut self) { self.close_document_impl(); }),
    prev_page: qt_method!(fn prev_page(&mut self) { self.prev_page_impl(); }),
    next_page: qt_method!(fn next_page(&mut self) { self.next_page_impl(); }),
    zoom_in: qt_method!(fn zoom_in(&mut self) { self.zoom_in_impl(); }),
    zoom_out: qt_method!(fn zoom_out(&mut self) { self.zoom_out_impl(); }),
    zoom_reset: qt_method!(fn zoom_reset(&mut self) { self.zoom_reset_impl(); }),
    zoom_set: qt_method!(fn zoom_set(&mut self, value: f32) { self.zoom_set_impl(value); }),
    undo: qt_method!(fn undo(&mut self) { self.undo_impl(); }),
    redo: qt_method!(fn redo(&mut self) { self.redo_impl(); }),
    rotate_page: qt_method!(fn rotate_page(&mut self) { self.rotate_page_impl(); }),
    delete_page: qt_method!(fn delete_page(&mut self) { self.delete_page_impl(); }),
    insert_text: qt_method!(fn insert_text(&mut self) { self.insert_text_impl(); }),
    modify_text: qt_method!(fn modify_text(&mut self) { self.modify_text_impl(); }),
    add_highlight: qt_method!(fn add_highlight(&mut self) { self.add_highlight_impl(); }),
    add_note: qt_method!(fn add_note(&mut self) { self.add_note_impl(); }),
    apply_ocr: qt_method!(fn apply_ocr(&mut self) { self.apply_ocr_impl(); }),
    redact_region: qt_method!(fn redact_region(&mut self) { self.redact_region_impl(); }),
    set_password: qt_method!(fn set_password(&mut self) { self.set_password_impl(); }),
    substitute_font: qt_method!(fn substitute_font(&mut self) { self.substitute_font_impl(); }),
    insert_image: qt_method!(fn insert_image(&mut self) { self.insert_image_impl(); }),
    reorder_pages: qt_method!(fn reorder_pages(&mut self) { self.reorder_pages_impl(); }),
    merge_document: qt_method!(fn merge_document(&mut self) { self.merge_document_impl(); }),
    create_field: qt_method!(fn create_field(&mut self) { self.create_field_impl(); }),
    set_field_value: qt_method!(fn set_field_value(&mut self) { self.set_field_value_impl(); }),
    detect_fields: qt_method!(fn detect_fields(&mut self) { self.detect_fields_impl(); }),
    export_form_data: qt_method!(fn export_form_data(&mut self) { self.export_form_data_impl(); }),
    upgrade_license: qt_method!(fn upgrade_license(&mut self) { self.upgrade_license_impl(); }),
    activate_license: qt_method!(fn activate_license(&mut self) { self.activate_license_impl(); }),
    open_recent: qt_method!(fn open_recent(&mut self, index: i32) { self.open_recent_impl(index); }),
}

impl Default for AppBridge {
    fn default() -> Self {
        let mut labels = match state().lock() {
            Ok(s) => s
                .recent_documents
                .iter()
                .take(5)
                .map(Self::recent_display)
                .collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        };
        while labels.len() < 5 {
            labels.push(String::new());
        }

        Self {
            base: Default::default(),
            status: QString::from(tr("ready")),
            status_changed: Default::default(),
            page_source: QString::from(""),
            page_source_changed: Default::default(),
            page_info: QString::from(format!("{}: -/-", tr("page"))),
            page_info_changed: Default::default(),
            zoom_info: QString::from(format!("{}: 100%", tr("zoom"))),
            zoom_info_changed: Default::default(),
            zoom_level: 1.0,
            zoom_level_changed: Default::default(),
            recent_a: labels[0].clone().into(),
            recent_b: labels[1].clone().into(),
            recent_c: labels[2].clone().into(),
            recent_d: labels[3].clone().into(),
            recent_e: labels[4].clone().into(),
            recent_changed: Default::default(),
            open_document: Default::default(),
            save_document: Default::default(),
            save_document_as: Default::default(),
            close_document: Default::default(),
            prev_page: Default::default(),
            next_page: Default::default(),
            zoom_in: Default::default(),
            zoom_out: Default::default(),
            zoom_reset: Default::default(),
            zoom_set: Default::default(),
            undo: Default::default(),
            redo: Default::default(),
            rotate_page: Default::default(),
            delete_page: Default::default(),
            insert_text: Default::default(),
            modify_text: Default::default(),
            add_highlight: Default::default(),
            add_note: Default::default(),
            apply_ocr: Default::default(),
            redact_region: Default::default(),
            set_password: Default::default(),
            substitute_font: Default::default(),
            insert_image: Default::default(),
            reorder_pages: Default::default(),
            merge_document: Default::default(),
            create_field: Default::default(),
            set_field_value: Default::default(),
            detect_fields: Default::default(),
            export_form_data: Default::default(),
            upgrade_license: Default::default(),
            activate_license: Default::default(),
            open_recent: Default::default(),
        }
    }
}

impl AppBridge {
    fn recent_display(path: &PathBuf) -> String {
        match path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name.to_owned(),
            None => path.to_string_lossy().into_owned(),
        }
    }

    fn refresh_recent_properties(&mut self) {
        let recents = match state().lock() {
            Ok(s) => s.recent_documents.clone(),
            Err(_) => return,
        };
        let mut labels = recents
            .iter()
            .take(5)
            .map(Self::recent_display)
            .collect::<Vec<_>>();
        while labels.len() < 5 {
            labels.push(String::new());
        }
        self.recent_a = labels[0].clone().into();
        self.recent_b = labels[1].clone().into();
        self.recent_c = labels[2].clone().into();
        self.recent_d = labels[3].clone().into();
        self.recent_e = labels[4].clone().into();
        self.recent_changed();
    }

    fn push_recent(path: PathBuf) {
        if let Ok(mut st) = state().lock() {
            st.recent_documents.retain(|p| p != &path);
            st.recent_documents.insert(0, path);
            if st.recent_documents.len() > 5 {
                st.recent_documents.truncate(5);
            }
            save_recent_documents(&st.recent_documents);
        }
    }

    fn set_status(&mut self, msg: impl AsRef<str>) {
        self.status = QString::from(localize_status(msg.as_ref()));
        self.status_changed();
    }

    fn set_page_info_and_zoom(&mut self, current: u32, total: u32, zoom: f32) {
        self.page_info = QString::from(format!("{}: {}/{}", tr("page"), current + 1, total));
        self.page_info_changed();
        self.zoom_info = QString::from(format!("{}: {:.0}%", tr("zoom"), zoom * 100.0));
        self.zoom_info_changed();
        self.zoom_level = zoom;
        self.zoom_level_changed();
    }

    fn clear_page_view(&mut self) {
        self.page_source = QString::from("");
        self.page_source_changed();
        self.page_info = QString::from(format!("{}: -/-", tr("page")));
        self.page_info_changed();
    }

    fn render_current_page(&mut self) -> Result<(), String> {
        let (rendered, current_page, page_count, zoom, nonce) = {
            let mut st = state()
                .lock()
                .map_err(|_| "state lock poisoned".to_owned())?;
            let count = match st.document.as_ref() {
                Some(doc) => doc.page_count(),
                None => {
                    self.clear_page_view();
                    return Ok(());
                }
            };
            if count == 0 {
                self.clear_page_view();
                return Ok(());
            }
            if st.current_page >= count {
                st.current_page = count - 1;
            }
            let current_page = st.current_page;
            let zoom = st.zoom;
            let rendered = {
                #[cfg(feature = "mupdf")]
                {
                    MuPdfRenderer::new()
                        .render_page(st.document.as_ref().expect("document checked"), current_page, zoom)
                        .or_else(|e| {
                            tracing::warn!("MuPDF render failed ({e}), fallback to software");
                            SoftwareRenderer.render_page(
                                st.document.as_ref().expect("document checked"),
                                current_page,
                                zoom,
                            )
                        })
                        .map_err(|e| e.to_string())?
                }
                #[cfg(not(feature = "mupdf"))]
                {
                    SoftwareRenderer
                        .render_page(st.document.as_ref().expect("document checked"), current_page, zoom)
                        .map_err(|e| e.to_string())?
                }
            };
            st.render_nonce = st.render_nonce.saturating_add(1);
            (rendered, current_page, count, zoom, st.render_nonce)
        };

        let img = RgbaImage::from_raw(rendered.width, rendered.height, rendered.data)
            .ok_or_else(|| "failed to build RGBA image".to_owned())?;
        let mut png_bytes = Vec::new();
        {
            let mut cursor = std::io::Cursor::new(&mut png_bytes);
            image::DynamicImage::ImageRgba8(img)
                .write_to(&mut cursor, image::ImageFormat::Png)
                .map_err(|e| format!("failed to encode preview: {e}"))?;
        }
        let encoded = base64::engine::general_purpose::STANDARD.encode(png_bytes);

        self.page_source = QString::from("");
        self.page_source_changed();
        self.page_source = QString::from(format!("data:image/png;base64,{}", encoded));
        self.page_source_changed();
        self.set_page_info_and_zoom(current_page, page_count, zoom);
        self.set_status(format!(
            "Rendered page {} ({}x{}, nonce {})",
            current_page + 1,
            rendered.width,
            rendered.height,
            nonce
        ));
        Ok(())
    }

    fn run_command(&mut self, cmd: Box<dyn DocumentCommand>, success: &str) {
        let result = {
            let mut st = match state().lock() {
                Ok(s) => s,
                Err(_) => {
                    self.set_status("Error: state lock poisoned");
                    return;
                }
            };
            let Some(mut doc) = st.document.take() else {
                self.set_status("Open a document first");
                return;
            };
            let res = st.history.execute(cmd, &mut doc);
            st.document = Some(doc);
            res
        };
        match result {
            Ok(()) => match self.render_current_page() {
                Ok(()) => self.set_status(success),
                Err(e) => self.set_status(format!("Error: {e}")),
            },
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn open_document_impl(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("PDF document", &["pdf"])
            .pick_file()
        else {
            return;
        };

        match Document::open(&path) {
            Ok(doc) => {
                let mut st = match state().lock() {
                    Ok(s) => s,
                    Err(_) => {
                        self.set_status("Error: state lock poisoned");
                        return;
                    }
                };
                st.document = Some(doc);
                st.current_page = 0;
                st.zoom = 1.0;
                st.history.clear();
                drop(st);
                Self::push_recent(path.clone());
                self.refresh_recent_properties();
                match self.render_current_page() {
                    Ok(()) => {
                        #[cfg(feature = "mupdf")]
                        self.set_status(format!("Opened: {}", path.display()));
                        #[cfg(not(feature = "mupdf"))]
                        self.set_status(format!(
                            "Opened: {} (placeholder renderer; rebuild with --features mupdf)",
                            path.display()
                        ));
                    }
                    Err(e) => self.set_status(format!("Error: {e}")),
                }
            }
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn save_document_impl(&mut self) {
        let result = {
            let mut st = match state().lock() {
                Ok(s) => s,
                Err(_) => {
                    self.set_status("Error: state lock poisoned");
                    return;
                }
            };
            let Some(doc) = st.document.as_mut() else {
                self.set_status("Open a document first");
                return;
            };
            doc.save().map(|_| doc.path.clone())
        };
        match result {
            Ok(path) => self.set_status(format!("Saved: {}", path.display())),
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn save_document_as_impl(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .set_file_name("document.pdf")
            .save_file()
        else {
            return;
        };
        let result = {
            let mut st = match state().lock() {
                Ok(s) => s,
                Err(_) => {
                    self.set_status("Error: state lock poisoned");
                    return;
                }
            };
            let Some(doc) = st.document.as_mut() else {
                self.set_status("Open a document first");
                return;
            };
            doc.save_to(&path)
        };
        match result {
            Ok(()) => self.set_status(format!("Saved As: {}", path.display())),
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn close_document_impl(&mut self) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        st.document = None;
        st.history.clear();
        st.current_page = 0;
        st.zoom = 1.0;
        drop(st);
        self.clear_page_view();
        self.set_status("Document closed");
    }

    fn open_recent_impl(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        let idx = index as usize;
        let path = match state().lock() {
            Ok(s) => s.recent_documents.get(idx).cloned(),
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        let Some(path) = path else {
            self.set_status("No recent file in this slot");
            return;
        };
        if !path.exists() {
            if let Ok(mut st) = state().lock() {
                st.recent_documents.retain(|p| p != &path);
                save_recent_documents(&st.recent_documents);
            }
            self.refresh_recent_properties();
            self.set_status(format!("Recent file not found: {}", path.display()));
            return;
        }

        match Document::open(&path) {
            Ok(doc) => {
                if let Ok(mut st) = state().lock() {
                    st.document = Some(doc);
                    st.current_page = 0;
                    st.zoom = 1.0;
                    st.history.clear();
                }
                Self::push_recent(path.clone());
                self.refresh_recent_properties();
                match self.render_current_page() {
                    Ok(()) => self.set_status(format!("Opened recent: {}", path.display())),
                    Err(e) => self.set_status(format!("Error: {e}")),
                }
            }
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn prev_page_impl(&mut self) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        if st.document.is_none() {
            self.set_status("Open a document first");
            return;
        }
        if st.current_page > 0 {
            st.current_page -= 1;
        }
        drop(st);
        if let Err(e) = self.render_current_page() {
            self.set_status(format!("Error: {e}"));
        }
    }

    fn next_page_impl(&mut self) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        let Some(doc) = st.document.as_ref() else {
            self.set_status("Open a document first");
            return;
        };
        let count = doc.page_count();
        if st.current_page + 1 < count {
            st.current_page += 1;
        }
        drop(st);
        if let Err(e) = self.render_current_page() {
            self.set_status(format!("Error: {e}"));
        }
    }

    fn zoom_in_impl(&mut self) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        st.zoom = (st.zoom * 1.25).clamp(0.1, 10.0);
        drop(st);
        if let Err(e) = self.render_current_page() {
            self.set_status(format!("Error: {e}"));
        }
    }

    fn zoom_out_impl(&mut self) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        st.zoom = (st.zoom * 0.8).clamp(0.1, 10.0);
        drop(st);
        if let Err(e) = self.render_current_page() {
            self.set_status(format!("Error: {e}"));
        }
    }

    fn zoom_reset_impl(&mut self) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        st.zoom = 1.0;
        drop(st);
        if let Err(e) = self.render_current_page() {
            self.set_status(format!("Error: {e}"));
        }
    }

    fn zoom_set_impl(&mut self, value: f32) {
        let mut st = match state().lock() {
            Ok(s) => s,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        st.zoom = value.clamp(0.25, 4.0);
        drop(st);
        if let Err(e) = self.render_current_page() {
            self.set_status(format!("Error: {e}"));
        }
    }

    fn undo_impl(&mut self) {
        let result = {
            let mut st = match state().lock() {
                Ok(s) => s,
                Err(_) => {
                    self.set_status("Error: state lock poisoned");
                    return;
                }
            };
            let Some(mut doc) = st.document.take() else {
                self.set_status("Open a document first");
                return;
            };
            let res = st.history.undo(&mut doc);
            st.document = Some(doc);
            res
        };
        match result {
            Ok(()) => match self.render_current_page() {
                Ok(()) => self.set_status("Undo"),
                Err(e) => self.set_status(format!("Error: {e}")),
            },
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn redo_impl(&mut self) {
        let result = {
            let mut st = match state().lock() {
                Ok(s) => s,
                Err(_) => {
                    self.set_status("Error: state lock poisoned");
                    return;
                }
            };
            let Some(mut doc) = st.document.take() else {
                self.set_status("Open a document first");
                return;
            };
            let res = st.history.redo(&mut doc);
            st.document = Some(doc);
            res
        };
        match result {
            Ok(()) => match self.render_current_page() {
                Ok(()) => self.set_status("Redo"),
                Err(e) => self.set_status(format!("Error: {e}")),
            },
            Err(e) => self.set_status(format!("Error: {e}")),
        }
    }

    fn rotate_page_impl(&mut self) {
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        self.run_command(Box::new(RotatePageCommand::new(page, 90)), "Page rotated");
    }

    fn delete_page_impl(&mut self) {
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        self.run_command(Box::new(DeletePageCommand::new(page)), "Page deleted");
    }

    fn insert_text_impl(&mut self) {
        let Some(text) = tinyfiledialogs::input_box("Insert Text", "Text:", "New text") else {
            return;
        };
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        self.run_command(
            Box::new(InsertTextCommand::new(page, text, 72.0, 700.0, 14.0)),
            "Inserted text",
        );
    }

    fn modify_text_impl(&mut self) {
        let Some(old_text) = tinyfiledialogs::input_box("Modify Text", "Find:", "") else {
            return;
        };
        let Some(new_text) = tinyfiledialogs::input_box("Modify Text", "Replace with:", "") else {
            return;
        };
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        self.run_command(
            Box::new(ModifyTextCommand::new(page, old_text, new_text)),
            "Modified text",
        );
    }

    fn substitute_font_impl(&mut self) {
        let Some(old_font) =
            tinyfiledialogs::input_box("Font Substitution", "Old font resource:", "Helvetica")
        else {
            return;
        };
        let Some(new_font) =
            tinyfiledialogs::input_box("Font Substitution", "New font resource:", "Times-Roman")
        else {
            return;
        };
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        self.run_command(
            Box::new(FontSubstitutionCommand::new(page, old_font, new_font)),
            "Applied font substitution",
        );
    }

    fn add_highlight_impl(&mut self) {
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        let annotation = Annotation::new(
            page,
            Rect {
                x: 80.0,
                y: 680.0,
                width: 180.0,
                height: 16.0,
            },
            AnnotationKind::Highlight {
                color: Color::yellow(),
            },
        );
        self.run_command(
            Box::new(AddAnnotationCommand::new(annotation)),
            "Added highlight",
        );
    }

    fn add_note_impl(&mut self) {
        let Some(content) = tinyfiledialogs::input_box("Insert Note", "Note text:", "Note #1")
        else {
            return;
        };
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        let annotation = Annotation::new(
            page,
            Rect {
                x: 80.0,
                y: 640.0,
                width: 22.0,
                height: 22.0,
            },
            AnnotationKind::Note {
                author: "User".into(),
                content,
            },
        );
        self.run_command(Box::new(AddAnnotationCommand::new(annotation)), "Added note");
    }

    fn apply_ocr_impl(&mut self) {
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        let result = OcrResult {
            page_index: page,
            regions: vec![TextRegion {
                text: "OCR sample".into(),
                x: 72.0,
                y: 640.0,
                width: 160.0,
                height: 20.0,
                confidence: 0.9,
            }],
            full_text: "OCR sample".into(),
        };
        self.run_command(Box::new(ApplyOcrCommand::new(result)), "Applied OCR layer");
    }

    fn redact_region_impl(&mut self) {
        let page = match state().lock() {
            Ok(s) => s.current_page,
            Err(_) => {
                self.set_status("Error: state lock poisoned");
                return;
            }
        };
        self.run_command(
            Box::new(RedactRegionCommand::new(page, 72.0, 680.0, 240.0, 40.0)),
            "Applied redaction",
        );
    }

    fn set_password_impl(&mut self) {
        let Some(password) = tinyfiledialogs::input_box("Set Password", "New password:", "") else {
            return;
        };
        self.run_command(
            Box::new(SetPasswordCommand::new(password)),
            "Document password set",
        );
    }

    fn insert_image_impl(&mut self) {
        self.set_status("Insert image: not implemented in Qt yet");
    }

    fn reorder_pages_impl(&mut self) {
        self.set_status("Reorder pages: not implemented in Qt yet");
    }

    fn merge_document_impl(&mut self) {
        self.set_status("Merge document: not implemented in Qt yet");
    }

    fn create_field_impl(&mut self) {
        self.set_status("Create field: not implemented in Qt yet");
    }

    fn set_field_value_impl(&mut self) {
        self.set_status("Set field value: not implemented in Qt yet");
    }

    fn detect_fields_impl(&mut self) {
        self.set_status("Detect fields: not implemented in Qt yet");
    }

    fn export_form_data_impl(&mut self) {
        self.set_status("Export form data: not implemented in Qt yet");
    }

    fn upgrade_license_impl(&mut self) {
        self.set_status("Upgrade license: not implemented in Qt yet");
    }

    fn activate_license_impl(&mut self) {
        self.set_status("Activate license: not implemented in Qt yet");
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("app_desktop=debug".parse().expect("valid directive")),
        )
        .init();

    info!("Free PDF Editor (Qt) starting");
    #[cfg(not(feature = "mupdf"))]
    info!("MuPDF feature is disabled; rendering uses placeholder software backend");

    qml_register_type::<AppBridge>(cstr!("PdfEditor"), 1, 0, cstr!("AppBridge"));

    let mut engine = QmlEngine::new();
    engine.load_data(
        r##"
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15
import PdfEditor 1.0

ApplicationWindow {
    id: root
    width: 1260
    height: 840
    visible: true
    title: t("app_title")
    Material.theme: (Qt.styleHints.colorScheme === Qt.ColorScheme.Dark) ? Material.Dark : Material.Light
    Material.primary: Material.BlueGrey
    Material.accent: Material.Cyan

    AppBridge { id: bridge }
    property string lang: {
        const n = Qt.locale().name.toLowerCase()
        const p = n.split(/[_-]/)[0]
        if (p === "cz") return "cs"
        return ["ru", "cs", "pl", "hu", "en", "es", "fr", "de"].indexOf(p) >= 0 ? p : "en"
    }
    function t(key) {
        const i18n = {
            "en": {
                app_title: "Free PDF Editor (Qt)", file: "File", open: "Open", save: "Save", save_as: "Save As", close: "Close",
                open_recent: "Open Recent", no_recent: "No recent files", edit: "Edit", undo: "Undo", redo: "Redo",
                annotate: "Annotate", highlight: "Highlight", note: "Note", page: "Page", previous: "Previous",
                next: "Next", delete_page: "Delete Page", rotate_page: "Rotate Page", tools: "Tools",
                insert_text: "Insert Text", font_substitute: "Font Substitute", insert_image: "Insert Image",
                set_password: "Set Password", redact: "Redact", apply_ocr: "Apply OCR", reorder_pages: "Reorder Pages",
                merge: "Merge", create_field: "Create Field", set_field_value: "Set Field Value",
                detect_fields: "Detect Fields", export_fields: "Export Fields", license: "License",
                upgrade: "Upgrade", activate: "Activate"
            },
            "ru": {
                app_title: "Free PDF Editor (Qt)", file: "Файл", open: "Открыть", save: "Сохранить", save_as: "Сохранить как", close: "Закрыть",
                open_recent: "Недавние файлы", no_recent: "Нет недавних файлов", edit: "Правка", undo: "Отменить", redo: "Повторить",
                annotate: "Аннотации", highlight: "Выделение", note: "Заметка", page: "Страница", previous: "Предыдущая",
                next: "Следующая", delete_page: "Удалить страницу", rotate_page: "Повернуть страницу", tools: "Инструменты",
                insert_text: "Вставить текст", font_substitute: "Замена шрифта", insert_image: "Вставить изображение",
                set_password: "Установить пароль", redact: "Редактировать", apply_ocr: "Применить OCR", reorder_pages: "Переупорядочить страницы",
                merge: "Объединить", create_field: "Создать поле", set_field_value: "Задать значение поля",
                detect_fields: "Найти поля", export_fields: "Экспорт полей", license: "Лицензия",
                upgrade: "Обновить", activate: "Активировать"
            },
            "cs": {
                app_title: "Free PDF Editor (Qt)", file: "Soubor", open: "Otevrit", save: "Ulozit", save_as: "Ulozit jako", close: "Zavrit",
                open_recent: "Nedavne soubory", no_recent: "Zadne nedavne soubory", edit: "Upravy", undo: "Zpet", redo: "Znovu",
                annotate: "Anotace", highlight: "Zvyraznit", note: "Poznamka", page: "Stranka", previous: "Predchozi",
                next: "Dalsi", delete_page: "Smazat stranku", rotate_page: "Otocit stranku", tools: "Nastroje",
                insert_text: "Vlozit text", font_substitute: "Nahrada pisma", insert_image: "Vlozit obrazek",
                set_password: "Nastavit heslo", redact: "Redigovat", apply_ocr: "Pouzit OCR", reorder_pages: "Preusporadat stranky",
                merge: "Sloucit", create_field: "Vytvorit pole", set_field_value: "Nastavit hodnotu pole",
                detect_fields: "Najit pole", export_fields: "Export poli", license: "Licence",
                upgrade: "Upgradovat", activate: "Aktivovat"
            },
            "pl": {
                app_title: "Free PDF Editor (Qt)", file: "Plik", open: "Otworz", save: "Zapisz", save_as: "Zapisz jako", close: "Zamknij",
                open_recent: "Ostatnie pliki", no_recent: "Brak ostatnich plikow", edit: "Edycja", undo: "Cofnij", redo: "Ponow",
                annotate: "Adnotacje", highlight: "Podkresl", note: "Notatka", page: "Strona", previous: "Poprzednia",
                next: "Nastepna", delete_page: "Usun strone", rotate_page: "Obroc strone", tools: "Narzędzia",
                insert_text: "Wstaw tekst", font_substitute: "Podmiana czcionki", insert_image: "Wstaw obraz",
                set_password: "Ustaw haslo", redact: "Redaguj", apply_ocr: "Zastosuj OCR", reorder_pages: "Zmien kolejnosc stron",
                merge: "Scal", create_field: "Utworz pole", set_field_value: "Ustaw wartosc pola",
                detect_fields: "Wykryj pola", export_fields: "Eksportuj pola", license: "Licencja",
                upgrade: "Ulepsz", activate: "Aktywuj"
            },
            "hu": {
                app_title: "Free PDF Editor (Qt)", file: "Fajl", open: "Megnyitas", save: "Mentes", save_as: "Mentés maskepp", close: "Bezaras",
                open_recent: "Legutobbi fajlok", no_recent: "Nincsenek legutobbi fajlok", edit: "Szerkesztes", undo: "Visszavonas", redo: "Ujra",
                annotate: "Jegyzetek", highlight: "Kiemeles", note: "Jegyzet", page: "Oldal", previous: "Elozo",
                next: "Kovetkezo", delete_page: "Oldal torlese", rotate_page: "Oldal forgatasa", tools: "Eszkozok",
                insert_text: "Szoveg beszurasa", font_substitute: "Betutipus csere", insert_image: "Kep beszurasa",
                set_password: "Jelszo beallitasa", redact: "Szerkeszt", apply_ocr: "OCR alkalmazasa", reorder_pages: "Oldalak at rendezese",
                merge: "Egyesites", create_field: "Mezo letrehozasa", set_field_value: "Mezo ertek beallitasa",
                detect_fields: "Mezok felismerese", export_fields: "Mezok exportalasa", license: "Licenc",
                upgrade: "Frissites", activate: "Aktivalas"
            },
            "es": {
                app_title: "Free PDF Editor (Qt)", file: "Archivo", open: "Abrir", save: "Guardar", save_as: "Guardar como", close: "Cerrar",
                open_recent: "Abrir recientes", no_recent: "Sin archivos recientes", edit: "Editar", undo: "Deshacer", redo: "Rehacer",
                annotate: "Anotar", highlight: "Resaltar", note: "Nota", page: "Pagina", previous: "Anterior",
                next: "Siguiente", delete_page: "Eliminar pagina", rotate_page: "Girar pagina", tools: "Herramientas",
                insert_text: "Insertar texto", font_substitute: "Sustituir fuente", insert_image: "Insertar imagen",
                set_password: "Establecer contrasena", redact: "Redactar", apply_ocr: "Aplicar OCR", reorder_pages: "Reordenar paginas",
                merge: "Combinar", create_field: "Crear campo", set_field_value: "Establecer valor de campo",
                detect_fields: "Detectar campos", export_fields: "Exportar campos", license: "Licencia",
                upgrade: "Mejorar", activate: "Activar"
            },
            "fr": {
                app_title: "Free PDF Editor (Qt)", file: "Fichier", open: "Ouvrir", save: "Enregistrer", save_as: "Enregistrer sous", close: "Fermer",
                open_recent: "Recents", no_recent: "Aucun fichier recent", edit: "Edition", undo: "Annuler", redo: "Retablir",
                annotate: "Annoter", highlight: "Surligner", note: "Note", page: "Page", previous: "Precedente",
                next: "Suivante", delete_page: "Supprimer la page", rotate_page: "Pivoter la page", tools: "Outils",
                insert_text: "Inserer du texte", font_substitute: "Substituer la police", insert_image: "Inserer une image",
                set_password: "Definir le mot de passe", redact: "Rediger", apply_ocr: "Appliquer OCR", reorder_pages: "Reordonner les pages",
                merge: "Fusionner", create_field: "Creer un champ", set_field_value: "Definir la valeur du champ",
                detect_fields: "Detecter les champs", export_fields: "Exporter les champs", license: "Licence",
                upgrade: "Mettre a niveau", activate: "Activer"
            },
            "de": {
                app_title: "Free PDF Editor (Qt)", file: "Datei", open: "Offnen", save: "Speichern", save_as: "Speichern unter", close: "Schliessen",
                open_recent: "Zuletzt geoffnet", no_recent: "Keine letzten Dateien", edit: "Bearbeiten", undo: "Ruckgangig", redo: "Wiederholen",
                annotate: "Annotieren", highlight: "Hervorheben", note: "Notiz", page: "Seite", previous: "Vorherige",
                next: "Nachste", delete_page: "Seite loschen", rotate_page: "Seite drehen", tools: "Werkzeuge",
                insert_text: "Text einfugen", font_substitute: "Schrift ersetzen", insert_image: "Bild einfugen",
                set_password: "Passwort setzen", redact: "Schwarzen", apply_ocr: "OCR anwenden", reorder_pages: "Seiten neu ordnen",
                merge: "Zusammenfuhren", create_field: "Feld erstellen", set_field_value: "Feldwert setzen",
                detect_fields: "Felder erkennen", export_fields: "Felder exportieren", license: "Lizenz",
                upgrade: "Upgrade", activate: "Aktivieren"
            }
        }
        if (i18n[lang] && i18n[lang][key] !== undefined) return i18n[lang][key]
        return (i18n["en"][key] !== undefined) ? i18n["en"][key] : key
    }

    menuBar: MenuBar {
        Menu {
            title: t("file")
            MenuItem { text: t("open"); onTriggered: bridge.open_document() }
            MenuItem { text: t("save"); onTriggered: bridge.save_document() }
            MenuItem { text: t("save_as"); onTriggered: bridge.save_document_as() }
            MenuItem { text: t("close"); onTriggered: bridge.close_document() }
            MenuSeparator {}
            Menu {
                title: t("open_recent")
                MenuItem { text: bridge.recent_a; visible: bridge.recent_a.length > 0; onTriggered: bridge.open_recent(0) }
                MenuItem { text: bridge.recent_b; visible: bridge.recent_b.length > 0; onTriggered: bridge.open_recent(1) }
                MenuItem { text: bridge.recent_c; visible: bridge.recent_c.length > 0; onTriggered: bridge.open_recent(2) }
                MenuItem { text: bridge.recent_d; visible: bridge.recent_d.length > 0; onTriggered: bridge.open_recent(3) }
                MenuItem { text: bridge.recent_e; visible: bridge.recent_e.length > 0; onTriggered: bridge.open_recent(4) }
                MenuItem {
                    text: t("no_recent")
                    enabled: false
                    visible: bridge.recent_a.length === 0
                }
            }
        }

        Menu {
            title: t("edit")
            MenuItem { text: t("undo"); onTriggered: bridge.undo() }
            MenuItem { text: t("redo"); onTriggered: bridge.redo() }
        }

        Menu {
            title: t("annotate")
            MenuItem { text: t("highlight"); onTriggered: bridge.add_highlight() }
            MenuItem { text: t("note"); onTriggered: bridge.add_note() }
        }

        Menu {
            title: t("page")
            MenuItem { text: t("previous"); onTriggered: bridge.prev_page() }
            MenuItem { text: t("next"); onTriggered: bridge.next_page() }
            MenuItem { text: t("delete_page"); onTriggered: bridge.delete_page() }
            MenuItem { text: t("rotate_page"); onTriggered: bridge.rotate_page() }
        }

        Menu {
            title: t("tools")
            MenuItem { text: t("insert_text"); onTriggered: bridge.insert_text() }
            MenuItem { text: t("font_substitute"); onTriggered: bridge.substitute_font() }
            MenuItem { text: t("insert_image"); onTriggered: bridge.insert_image() }
            MenuSeparator {}
            MenuItem { text: t("set_password"); onTriggered: bridge.set_password() }
            MenuItem { text: t("redact"); onTriggered: bridge.redact_region() }
            MenuItem { text: t("apply_ocr"); onTriggered: bridge.apply_ocr() }
            MenuItem { text: t("reorder_pages"); onTriggered: bridge.reorder_pages() }
            MenuItem { text: t("merge"); onTriggered: bridge.merge_document() }
            MenuItem { text: t("create_field"); onTriggered: bridge.create_field() }
            MenuItem { text: t("set_field_value"); onTriggered: bridge.set_field_value() }
            MenuItem { text: t("detect_fields"); onTriggered: bridge.detect_fields() }
            MenuItem { text: t("export_fields"); onTriggered: bridge.export_form_data() }
        }

        Menu {
            title: t("license")
            MenuItem { text: t("upgrade"); onTriggered: bridge.upgrade_license() }
            MenuItem { text: t("activate"); onTriggered: bridge.activate_license() }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 8

        RowLayout {
            Layout.fillWidth: true
            spacing: 18
            Label { text: bridge.page_info }
            Label { text: bridge.zoom_info }
            Slider {
                Layout.preferredWidth: 220
                from: 0.25
                to: 4.0
                value: bridge.zoom_level
                onMoved: bridge.zoom_set(value)
                onValueChanged: {
                    if (Math.abs(value - bridge.zoom_level) > 0.001) {
                        bridge.zoom_set(value)
                    }
                }
            }
            Button { text: "-"; onClicked: bridge.zoom_out() }
            Button { text: "+"; onClicked: bridge.zoom_in() }
            Button { text: "100%"; onClicked: bridge.zoom_reset() }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            border.color: "#c7c7c7"
            border.width: 1
            color: "#f9fafb"

            Flickable {
                id: pageView
                anchors.fill: parent
                clip: true
                boundsBehavior: Flickable.StopAtBounds
                contentWidth: Math.max(pageImage.width, width)
                contentHeight: Math.max(pageImage.height, height)

                Image {
                    id: pageImage
                    source: bridge.page_source
                    cache: false
                    smooth: true
                    fillMode: Image.PreserveAspectFit
                    width: implicitWidth > 0 ? implicitWidth * bridge.zoom_level : pageView.width
                    height: implicitHeight > 0 ? implicitHeight * bridge.zoom_level : pageView.height
                    x: Math.max((pageView.width - width) / 2, 0)
                    y: Math.max((pageView.height - height) / 2, 0)
                }
            }

            MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                hoverEnabled: true
                onWheel: function(wheel) {
                    if ((wheel.modifiers & Qt.ControlModifier) !== 0) {
                        if (wheel.angleDelta.y > 0) {
                            bridge.zoom_in()
                        } else if (wheel.angleDelta.y < 0) {
                            bridge.zoom_out()
                        }
                    } else {
                        if (wheel.angleDelta.y > 0) {
                            bridge.prev_page()
                        } else if (wheel.angleDelta.y < 0) {
                            bridge.next_page()
                        }
                    }
                    wheel.accepted = true
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            height: 34
            color: "#111827"
            Text {
                anchors.verticalCenter: parent.verticalCenter
                anchors.left: parent.left
                anchors.leftMargin: 10
                text: bridge.status
                color: "white"
            }
        }
    }
}
"##
        .into(),
    );
    engine.exec();
}



