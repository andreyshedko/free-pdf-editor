use crate::AppWindow;

#[derive(Debug, Clone, Copy)]
pub enum Language {
    En,
    Ru,
    Cs,
    Pl,
    Es,
    Fr,
    It,
    De,
    Pt,
}

#[derive(Debug, Clone, Copy)]
pub struct UiStrings {
    pub app_title: &'static str,
    pub no_document: &'static str,
    pub page_preview: &'static str,
    pub current_prefix: &'static str,
    pub no_document_loaded: &'static str,
    pub page_prefix: &'static str,
    pub license_prefix: &'static str,
    pub expires_prefix: &'static str,
    pub open_recent: &'static str,
    pub no_recent_files: &'static str,
    pub click_image_to_edit: &'static str,
    pub width_short: &'static str,
    pub height_short: &'static str,
    pub lock_aspect: &'static str,
    pub apply: &'static str,
    pub text: &'static str,
    pub font: &'static str,
    pub size: &'static str,
    pub insert: &'static str,
    pub close: &'static str,
    pub zoom: &'static str,
    pub menu_file: &'static str,
    pub menu_edit: &'static str,
    pub menu_view: &'static str,
    pub menu_annotate: &'static str,
    pub menu_page: &'static str,
    pub menu_tools: &'static str,
    pub menu_license: &'static str,
    pub open: &'static str,
    pub save: &'static str,
    pub save_as: &'static str,
    pub undo: &'static str,
    pub redo: &'static str,
    pub reset_100: &'static str,
    pub highlight: &'static str,
    pub note: &'static str,
    pub previous: &'static str,
    pub next: &'static str,
    pub delete_page: &'static str,
    pub rotate_page: &'static str,
    pub insert_text: &'static str,
    pub font_substitute: &'static str,
    pub insert_image: &'static str,
    pub set_password: &'static str,
    pub redact: &'static str,
    pub apply_ocr: &'static str,
    pub reorder_pages: &'static str,
    pub merge: &'static str,
    pub create_field: &'static str,
    pub set_field_value: &'static str,
    pub detect_fields: &'static str,
    pub export_fields: &'static str,
    pub upgrade: &'static str,
    pub activate: &'static str,
    pub status_ready: &'static str,
    pub status_document_opened: &'static str,
    pub status_document_closed: &'static str,
    pub status_saved_prefix: &'static str,
    pub status_page_modified: &'static str,
    pub status_error_prefix: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct Localization {
    pub language: Language,
    pub ui: UiStrings,
}

impl Localization {
    pub fn detect_system() -> Self {
        let locale = sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string());
        let language = Language::from_locale(&locale);
        Self {
            language,
            ui: UiStrings::for_language(language),
        }
    }

    pub fn localize_status_message(&self, message: &str) -> String {
        if matches!(self.language, Language::En) {
            return message.to_string();
        }

        if matches!(self.language, Language::Ru) {
            if let Some(rest) = message.strip_prefix("Recent file no longer exists: ") {
                return format!("Недавний файл больше не существует: {rest}");
            }
            if let Some(rest) = message.strip_prefix("Selected image resource '") {
                return format!("Выбран ресурс изображения '{rest}");
            }
            if let Some(rest) = message.strip_prefix("Detected ") {
                if let Some(count) = rest.strip_suffix(" form field(s)") {
                    return format!("Найдено полей формы: {count}");
                }
            }
            if let Some(rest) = message.strip_prefix("Exported form data to ") {
                return format!("Данные формы экспортированы в {rest}");
            }
            if let Some(rest) = message.strip_prefix("Failed to export form data: ") {
                return format!("Не удалось экспортировать данные формы: {rest}");
            }
            if let Some(rest) = message.strip_prefix("Failed to prepare render bytes: ") {
                return format!("Не удалось подготовить данные рендера: {rest}");
            }
            if let Some(rest) = message.strip_prefix("License activation failed: ") {
                return format!("Ошибка активации лицензии: {rest}");
            }

            let translated = match message {
                "Visit https://example.com/upgrade to purchase a commercial license." => {
                    "Перейдите на https://example.com/upgrade для покупки коммерческой лицензии."
                }
                "Open canceled (optionally set OPEN_PDF as fallback)" => {
                    "Открытие отменено (можно задать OPEN_PDF как резервный путь)"
                }
                "Recent document entry not available" => "Недавний документ недоступен",
                "This build uses a placeholder renderer. Rebuild with --features mupdf for full PDF preview." => {
                    "В этой сборке используется упрощенный рендер. Пересоберите с --features mupdf для полного предпросмотра PDF."
                }
                "Saving with personal license – exported PDF may contain watermark for commercial use." => {
                    "Сохранение с персональной лицензией: экспортированный PDF может содержать водяной знак для коммерческого использования."
                }
                "Save canceled" => "Сохранение отменено",
                "No document is open" => "Нет открытого документа",
                "Save As canceled" => "Сохранение как отменено",
                "Highlight placement mode disabled" => "Режим установки выделения отключен",
                "Click on the page to place a highlight" => {
                    "Нажмите на страницу, чтобы поставить выделение"
                }
                "Note placement mode disabled" => "Режим установки заметки отключен",
                "Click on the page to place a note" => {
                    "Нажмите на страницу, чтобы поставить заметку"
                }
                "Note edit canceled" => "Редактирование заметки отменено",
                "Note unchanged" => "Заметка не изменена",
                "Note updated. Click to edit/add another note, or select Note again to stop." => {
                    "Заметка обновлена. Нажмите для редактирования/добавления следующей заметки или снова выберите 'Заметка', чтобы остановить режим."
                }
                "Note insertion canceled" => "Вставка заметки отменена",
                "Text panel opened for selected text" => {
                    "Панель текста открыта для выбранного текста"
                }
                "No editable text or image found at click location" => {
                    "В месте клика не найден редактируемый текст или изображение"
                }
                "Open a document first" => "Сначала откройте документ",
                "Text insert panel opened" => "Панель вставки текста открыта",
                "Insert text canceled (empty text)" => {
                    "Вставка текста отменена (пустой текст)"
                }
                "Insert text failed (page is not available)" => {
                    "Не удалось вставить текст (страница недоступна)"
                }
                "Modify text canceled" => "Изменение текста отменено",
                "Modify text canceled (empty find text)" => {
                    "Изменение текста отменено (пустая строка поиска)"
                }
                "Font substitution canceled" => "Замена шрифта отменена",
                "Font substitution canceled (empty source font)" => {
                    "Замена шрифта отменена (пустой исходный шрифт)"
                }
                "Font substitution canceled (empty target font)" => {
                    "Замена шрифта отменена (пустой целевой шрифт)"
                }
                "Insert image canceled" => "Вставка изображения отменена",
                "Image edit canceled" => "Редактирование изображения отменено",
                "No image resources found on current page" => {
                    "На текущей странице ресурсы изображений не найдены"
                }
                "Selected image resource is not available" => {
                    "Выбранный ресурс изображения недоступен"
                }
                "No selected image to delete" => "Нет выбранного изображения для удаления",
                "Replace image canceled" => "Замена изображения отменена",
                "Resize image canceled" => "Изменение размера изображения отменено",
                "Invalid size format" => "Неверный формат размера",
                "Invalid width value" => "Неверное значение ширины",
                "Invalid height value" => "Неверное значение высоты",
                "Width/height must be > 1" => "Ширина/высота должны быть > 1",
                "Image transform is not axis-aligned; resize unsupported" => {
                    "Трансформация изображения не выровнена по осям; изменение размера не поддерживается"
                }
                "Unknown image action" => "Неизвестное действие с изображением",
                "Select an image first" => "Сначала выберите изображение",
                "Invalid width/height" => "Неверные ширина/высота",
                "Resize is not supported for this transform" => {
                    "Изменение размера не поддерживается для этой трансформации"
                }
                "Set password canceled" => "Установка пароля отменена",
                "Need at least 2 pages to reorder" => {
                    "Для переупорядочивания нужно минимум 2 страницы"
                }
                "Merge canceled" => "Объединение отменено",
                "Create field canceled" => "Создание поля отменено",
                "No form fields found" => "Поля формы не найдены",
                "Set field value canceled" => "Установка значения поля отменена",
                "Set value for first form field" => "Установите значение для первого поля формы",
                "Export fields canceled" => "Экспорт полей отменен",
                "Unknown menu action" => "Неизвестное действие меню",
                "Set ACTIVATE_LICENSE env var to the path of your .pdfeditor-license file" => {
                    "Задайте переменную окружения ACTIVATE_LICENSE с путем к вашему файлу .pdfeditor-license"
                }
                "Commercial license activated successfully." => {
                    "Коммерческая лицензия успешно активирована."
                }
                "License activation is not available in this build." => {
                    "Активация лицензии недоступна в этой сборке."
                }
                _ => message,
            };
            return translated.to_string();
        }

        let dynamic = match self.language {
            Language::Cs => Self::localize_dynamic_status(
                message,
                "Posledni soubor uz neexistuje: ",
                "Vybrany zdroj obrazku '",
                "Nalezena pole formulare: ",
                "Data formulare exportovana do ",
                "Export dat formulare se nezdaril: ",
                "Nepodarilo se pripravit data renderu: ",
                "Aktivace licence selhala: ",
            ),
            Language::Pl => Self::localize_dynamic_status(
                message,
                "Ostatni plik juz nie istnieje: ",
                "Wybrany zasob obrazu '",
                "Wykryto pola formularza: ",
                "Dane formularza wyeksportowano do ",
                "Nie udalo sie wyeksportowac danych formularza: ",
                "Nie udalo sie przygotowac danych renderowania: ",
                "Aktywacja licencji nie powiodla sie: ",
            ),
            Language::Es => Self::localize_dynamic_status(
                message,
                "El archivo reciente ya no existe: ",
                "Recurso de imagen seleccionado '",
                "Campos de formulario detectados: ",
                "Datos del formulario exportados a ",
                "Error al exportar datos del formulario: ",
                "No se pudieron preparar los datos de render: ",
                "Fallo la activacion de licencia: ",
            ),
            Language::Fr => Self::localize_dynamic_status(
                message,
                "Le fichier recent n'existe plus: ",
                "Ressource d'image selectionnee '",
                "Champs de formulaire detectes: ",
                "Donnees du formulaire exportees vers ",
                "Echec de l'export des donnees du formulaire: ",
                "Echec de la preparation des donnees de rendu: ",
                "Echec de l'activation de la licence: ",
            ),
            Language::It => Self::localize_dynamic_status(
                message,
                "Il file recente non esiste piu: ",
                "Risorsa immagine selezionata '",
                "Campi modulo rilevati: ",
                "Dati modulo esportati in ",
                "Esportazione dati modulo non riuscita: ",
                "Impossibile preparare i dati di rendering: ",
                "Attivazione licenza non riuscita: ",
            ),
            Language::De => Self::localize_dynamic_status(
                message,
                "Zuletzt verwendete Datei existiert nicht mehr: ",
                "Ausgewahlte Bildressource '",
                "Gefundene Formularfelder: ",
                "Formulardaten exportiert nach ",
                "Export der Formulardaten fehlgeschlagen: ",
                "Render-Daten konnten nicht vorbereitet werden: ",
                "Lizenzaktivierung fehlgeschlagen: ",
            ),
            Language::Pt => Self::localize_dynamic_status(
                message,
                "O arquivo recente nao existe mais: ",
                "Recurso de imagem selecionado '",
                "Campos de formulario detectados: ",
                "Dados do formulario exportados para ",
                "Falha ao exportar dados do formulario: ",
                "Falha ao preparar dados de renderizacao: ",
                "Falha na ativacao da licenca: ",
            ),
            _ => None,
        };
        if let Some(translated) = dynamic {
            return translated;
        }

        let translated = match (self.language, message) {
            (Language::Cs, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Otevreni zruseno (volitelne nastavte OPEN_PDF jako zalohu)"
            }
            (Language::Pl, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Otwarcie anulowane (opcjonalnie ustaw OPEN_PDF jako rezerwe)"
            }
            (Language::Es, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Apertura cancelada (opcionalmente define OPEN_PDF como respaldo)"
            }
            (Language::Fr, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Ouverture annulee (optionnel: definir OPEN_PDF comme secours)"
            }
            (Language::It, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Apertura annullata (facoltativo: imposta OPEN_PDF come fallback)"
            }
            (Language::De, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Offnen abgebrochen (optional OPEN_PDF als Fallback setzen)"
            }
            (Language::Pt, "Open canceled (optionally set OPEN_PDF as fallback)") => {
                "Abertura cancelada (opcionalmente defina OPEN_PDF como fallback)"
            }

            (Language::Cs, "Recent document entry not available") => {
                "Polozka nedavnych dokumentu neni dostupna"
            }
            (Language::Pl, "Recent document entry not available") => {
                "Pozycja ostatniego dokumentu jest niedostepna"
            }
            (Language::Es, "Recent document entry not available") => {
                "La entrada de documento reciente no esta disponible"
            }
            (Language::Fr, "Recent document entry not available") => {
                "L'entree de document recent n'est pas disponible"
            }
            (Language::It, "Recent document entry not available") => {
                "La voce del documento recente non e disponibile"
            }
            (Language::De, "Recent document entry not available") => {
                "Eintrag fur zuletzt verwendetes Dokument ist nicht verfugbar"
            }
            (Language::Pt, "Recent document entry not available") => {
                "A entrada de documento recente nao esta disponivel"
            }

            (Language::Cs, "Save canceled") => "Ulozeni zruseno",
            (Language::Pl, "Save canceled") => "Zapisywanie anulowane",
            (Language::Es, "Save canceled") => "Guardado cancelado",
            (Language::Fr, "Save canceled") => "Enregistrement annule",
            (Language::It, "Save canceled") => "Salvataggio annullato",
            (Language::De, "Save canceled") => "Speichern abgebrochen",
            (Language::Pt, "Save canceled") => "Salvamento cancelado",

            (Language::Cs, "No document is open") => "Neni otevren zadny dokument",
            (Language::Pl, "No document is open") => "Brak otwartego dokumentu",
            (Language::Es, "No document is open") => "No hay ningun documento abierto",
            (Language::Fr, "No document is open") => "Aucun document n'est ouvert",
            (Language::It, "No document is open") => "Nessun documento e aperto",
            (Language::De, "No document is open") => "Kein Dokument ist geoffnet",
            (Language::Pt, "No document is open") => "Nenhum documento esta aberto",

            (Language::Cs, "Save As canceled") => "Ulozit jako zruseno",
            (Language::Pl, "Save As canceled") => "Zapisz jako anulowane",
            (Language::Es, "Save As canceled") => "Guardar como cancelado",
            (Language::Fr, "Save As canceled") => "Enregistrer sous annule",
            (Language::It, "Save As canceled") => "Salva con nome annullato",
            (Language::De, "Save As canceled") => "Speichern unter abgebrochen",
            (Language::Pt, "Save As canceled") => "Salvar como cancelado",

            (Language::Cs, "Open a document first") => "Nejprve otevrete dokument",
            (Language::Pl, "Open a document first") => "Najpierw otworz dokument",
            (Language::Es, "Open a document first") => "Primero abre un documento",
            (Language::Fr, "Open a document first") => "Ouvrez d'abord un document",
            (Language::It, "Open a document first") => "Apri prima un documento",
            (Language::De, "Open a document first") => "Offnen Sie zuerst ein Dokument",
            (Language::Pt, "Open a document first") => "Abra um documento primeiro",

            (Language::Cs, "No editable text or image found at click location") => {
                "Na miste kliknuti nebyl nalezen upravitelny text ani obrazek"
            }
            (Language::Pl, "No editable text or image found at click location") => {
                "W miejscu klikniecia nie znaleziono edytowalnego tekstu ani obrazu"
            }
            (Language::Es, "No editable text or image found at click location") => {
                "No se encontro texto o imagen editable en la posicion del clic"
            }
            (Language::Fr, "No editable text or image found at click location") => {
                "Aucun texte ou image modifiable trouve a l'emplacement du clic"
            }
            (Language::It, "No editable text or image found at click location") => {
                "Nessun testo o immagine modificabile trovato nel punto cliccato"
            }
            (Language::De, "No editable text or image found at click location") => {
                "An der Klickposition wurde kein bearbeitbarer Text oder kein Bild gefunden"
            }
            (Language::Pt, "No editable text or image found at click location") => {
                "Nenhum texto ou imagem editavel encontrado no local do clique"
            }

            (Language::Cs, "Unknown menu action") => "Neznama akce menu",
            (Language::Pl, "Unknown menu action") => "Nieznana akcja menu",
            (Language::Es, "Unknown menu action") => "Accion de menu desconocida",
            (Language::Fr, "Unknown menu action") => "Action de menu inconnue",
            (Language::It, "Unknown menu action") => "Azione menu sconosciuta",
            (Language::De, "Unknown menu action") => "Unbekannte Menuaktion",
            (Language::Pt, "Unknown menu action") => "Acao de menu desconhecida",

            (Language::Cs, "Commercial license activated successfully.") => {
                "Komerci licence byla uspesne aktivovana."
            }
            (Language::Pl, "Commercial license activated successfully.") => {
                "Licencja komercyjna zostala pomyslnie aktywowana."
            }
            (Language::Es, "Commercial license activated successfully.") => {
                "La licencia comercial se activo correctamente."
            }
            (Language::Fr, "Commercial license activated successfully.") => {
                "La licence commerciale a ete activee avec succes."
            }
            (Language::It, "Commercial license activated successfully.") => {
                "Licenza commerciale attivata con successo."
            }
            (Language::De, "Commercial license activated successfully.") => {
                "Kommerzielle Lizenz erfolgreich aktiviert."
            }
            (Language::Pt, "Commercial license activated successfully.") => {
                "Licenca comercial ativada com sucesso."
            }

            (Language::Cs, "License activation is not available in this build.") => {
                "Aktivace licence neni v tomto sestaveni dostupna."
            }
            (Language::Pl, "License activation is not available in this build.") => {
                "Aktywacja licencji nie jest dostepna w tej kompilacji."
            }
            (Language::Es, "License activation is not available in this build.") => {
                "La activacion de licencia no esta disponible en esta compilacion."
            }
            (Language::Fr, "License activation is not available in this build.") => {
                "L'activation de licence n'est pas disponible dans cette build."
            }
            (Language::It, "License activation is not available in this build.") => {
                "L'attivazione della licenza non e disponibile in questa build."
            }
            (Language::De, "License activation is not available in this build.") => {
                "Lizenzaktivierung ist in diesem Build nicht verfugbar."
            }
            (Language::Pt, "License activation is not available in this build.") => {
                "A ativacao de licenca nao esta disponivel nesta compilacao."
            }
            _ => message,
        };
        return translated.to_string();
    }

    fn localize_dynamic_status(
        message: &str,
        recent_missing_prefix: &str,
        selected_image_prefix: &str,
        detected_fields_prefix: &str,
        exported_to_prefix: &str,
        export_failed_prefix: &str,
        render_bytes_failed_prefix: &str,
        license_activation_failed_prefix: &str,
    ) -> Option<String> {
        if let Some(rest) = message.strip_prefix("Recent file no longer exists: ") {
            return Some(format!("{recent_missing_prefix}{rest}"));
        }
        if let Some(rest) = message.strip_prefix("Selected image resource '") {
            return Some(format!("{selected_image_prefix}{rest}"));
        }
        if let Some(rest) = message.strip_prefix("Detected ") {
            if let Some(count) = rest.strip_suffix(" form field(s)") {
                return Some(format!("{detected_fields_prefix}{count}"));
            }
        }
        if let Some(rest) = message.strip_prefix("Exported form data to ") {
            return Some(format!("{exported_to_prefix}{rest}"));
        }
        if let Some(rest) = message.strip_prefix("Failed to export form data: ") {
            return Some(format!("{export_failed_prefix}{rest}"));
        }
        if let Some(rest) = message.strip_prefix("Failed to prepare render bytes: ") {
            return Some(format!("{render_bytes_failed_prefix}{rest}"));
        }
        if let Some(rest) = message.strip_prefix("License activation failed: ") {
            return Some(format!("{license_activation_failed_prefix}{rest}"));
        }
        None
    }

    pub fn apply_to_window(&self, window: &AppWindow) {
        window.set_tr_app_title(self.ui.app_title.into());
        window.set_tr_no_document(self.ui.no_document.into());
        window.set_tr_page_preview(self.ui.page_preview.into());
        window.set_tr_current_prefix(self.ui.current_prefix.into());
        window.set_tr_no_document_loaded(self.ui.no_document_loaded.into());
        window.set_tr_page_prefix(self.ui.page_prefix.into());
        window.set_tr_license_prefix(self.ui.license_prefix.into());
        window.set_tr_expires_prefix(self.ui.expires_prefix.into());
        window.set_tr_open_recent(self.ui.open_recent.into());
        window.set_tr_no_recent_files(self.ui.no_recent_files.into());
        window.set_tr_click_image_to_edit(self.ui.click_image_to_edit.into());
        window.set_tr_width_short(self.ui.width_short.into());
        window.set_tr_height_short(self.ui.height_short.into());
        window.set_tr_lock_aspect(self.ui.lock_aspect.into());
        window.set_tr_apply(self.ui.apply.into());
        window.set_tr_text(self.ui.text.into());
        window.set_tr_font(self.ui.font.into());
        window.set_tr_size(self.ui.size.into());
        window.set_tr_insert(self.ui.insert.into());
        window.set_tr_close(self.ui.close.into());
        window.set_tr_zoom(self.ui.zoom.into());

        window.set_tr_menu_file(self.ui.menu_file.into());
        window.set_tr_menu_edit(self.ui.menu_edit.into());
        window.set_tr_menu_view(self.ui.menu_view.into());
        window.set_tr_menu_annotate(self.ui.menu_annotate.into());
        window.set_tr_menu_page(self.ui.menu_page.into());
        window.set_tr_menu_tools(self.ui.menu_tools.into());
        window.set_tr_menu_license(self.ui.menu_license.into());

        window.set_tr_open(self.ui.open.into());
        window.set_tr_save(self.ui.save.into());
        window.set_tr_save_as(self.ui.save_as.into());
        window.set_tr_undo(self.ui.undo.into());
        window.set_tr_redo(self.ui.redo.into());
        window.set_tr_reset_100(self.ui.reset_100.into());
        window.set_tr_highlight(self.ui.highlight.into());
        window.set_tr_note(self.ui.note.into());
        window.set_tr_previous(self.ui.previous.into());
        window.set_tr_next(self.ui.next.into());
        window.set_tr_delete_page(self.ui.delete_page.into());
        window.set_tr_rotate_page(self.ui.rotate_page.into());
        window.set_tr_insert_text(self.ui.insert_text.into());
        window.set_tr_font_substitute(self.ui.font_substitute.into());
        window.set_tr_insert_image(self.ui.insert_image.into());
        window.set_tr_set_password(self.ui.set_password.into());
        window.set_tr_redact(self.ui.redact.into());
        window.set_tr_apply_ocr(self.ui.apply_ocr.into());
        window.set_tr_reorder_pages(self.ui.reorder_pages.into());
        window.set_tr_merge(self.ui.merge.into());
        window.set_tr_create_field(self.ui.create_field.into());
        window.set_tr_set_field_value(self.ui.set_field_value.into());
        window.set_tr_detect_fields(self.ui.detect_fields.into());
        window.set_tr_export_fields(self.ui.export_fields.into());
        window.set_tr_upgrade(self.ui.upgrade.into());
        window.set_tr_activate(self.ui.activate.into());

        window.set_status_text(self.ui.status_ready.into());
    }
}

impl Language {
    fn from_locale(locale: &str) -> Self {
        let normalized = locale.to_ascii_lowercase();
        let code = normalized
            .split(['-', '_', '.'])
            .next()
            .unwrap_or("en");

        match code {
            "ru" => Self::Ru,
            "cs" | "cz" => Self::Cs,
            "pl" => Self::Pl,
            "es" => Self::Es,
            "fr" => Self::Fr,
            "it" => Self::It,
            "de" | "ger" => Self::De,
            "pt" => Self::Pt,
            "us" | "en" => Self::En,
            _ => Self::En,
        }
    }
}

impl UiStrings {
    fn for_language(language: Language) -> Self {
        match language {
            Language::En => Self::en(),
            Language::Ru => Self::ru(),
            Language::Cs => Self::cs(),
            Language::Pl => Self::pl(),
            Language::Es => Self::es(),
            Language::Fr => Self::fr(),
            Language::It => Self::it(),
            Language::De => Self::de(),
            Language::Pt => Self::pt(),
        }
    }

    fn en() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "No document",
            page_preview: "Page Preview",
            current_prefix: "Current",
            no_document_loaded: "No document loaded",
            page_prefix: "Page",
            license_prefix: "License",
            expires_prefix: "Expires",
            open_recent: "Open Recent",
            no_recent_files: "No recent files",
            click_image_to_edit: "Click image on page to edit",
            width_short: "W",
            height_short: "H",
            lock_aspect: "Lock aspect",
            apply: "Apply",
            text: "Text",
            font: "Font",
            size: "Size",
            insert: "Insert",
            close: "Close",
            zoom: "Zoom",
            menu_file: "File",
            menu_edit: "Edit",
            menu_view: "View",
            menu_annotate: "Annotate",
            menu_page: "Page",
            menu_tools: "Tools",
            menu_license: "License",
            open: "Open",
            save: "Save",
            save_as: "Save As",
            undo: "Undo",
            redo: "Redo",
            reset_100: "Reset 100%",
            highlight: "Highlight",
            note: "Note",
            previous: "Previous",
            next: "Next",
            delete_page: "Delete Page",
            rotate_page: "Rotate Page",
            insert_text: "Insert Text",
            font_substitute: "Font Substitute",
            insert_image: "Insert Image",
            set_password: "Set Password",
            redact: "Redact",
            apply_ocr: "Apply OCR",
            reorder_pages: "Reorder Pages",
            merge: "Merge",
            create_field: "Create Field",
            set_field_value: "Set Field Value",
            detect_fields: "Detect Fields",
            export_fields: "Export Fields",
            upgrade: "Upgrade",
            activate: "Activate",
            status_ready: "Ready",
            status_document_opened: "Document opened",
            status_document_closed: "Document closed",
            status_saved_prefix: "Saved",
            status_page_modified: "Page modified",
            status_error_prefix: "Error",
        }
    }

    fn ru() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Нет документа",
            page_preview: "Предпросмотр страницы",
            current_prefix: "Текущая",
            no_document_loaded: "Документ не открыт",
            page_prefix: "Страница",
            license_prefix: "Лицензия",
            expires_prefix: "Действует до",
            open_recent: "Недавние файлы",
            no_recent_files: "Нет недавних файлов",
            click_image_to_edit: "Нажмите на изображение для редактирования",
            width_short: "Ш",
            height_short: "В",
            lock_aspect: "Сохранить пропорции",
            apply: "Применить",
            text: "Текст",
            font: "Шрифт",
            size: "Размер",
            insert: "Вставить",
            close: "Закрыть",
            zoom: "Масштаб",
            menu_file: "Файл",
            menu_edit: "Правка",
            menu_view: "Вид",
            menu_annotate: "Аннотации",
            menu_page: "Страница",
            menu_tools: "Инструменты",
            menu_license: "Лицензия",
            open: "Открыть",
            save: "Сохранить",
            save_as: "Сохранить как",
            undo: "Отменить",
            redo: "Повторить",
            reset_100: "Сброс 100%",
            highlight: "Выделение",
            note: "Заметка",
            previous: "Предыдущая",
            next: "Следующая",
            delete_page: "Удалить страницу",
            rotate_page: "Повернуть страницу",
            insert_text: "Вставить текст",
            font_substitute: "Замена шрифта",
            insert_image: "Вставить изображение",
            set_password: "Установить пароль",
            redact: "Редактировать",
            apply_ocr: "Применить OCR",
            reorder_pages: "Переупорядочить страницы",
            merge: "Объединить",
            create_field: "Создать поле",
            set_field_value: "Задать значение поля",
            detect_fields: "Найти поля",
            export_fields: "Экспорт полей",
            upgrade: "Обновить",
            activate: "Активировать",
            status_ready: "Готово",
            status_document_opened: "Документ открыт",
            status_document_closed: "Документ закрыт",
            status_saved_prefix: "Сохранено",
            status_page_modified: "Страница изменена",
            status_error_prefix: "Ошибка",
        }
    }

    fn cs() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Zadny dokument",
            page_preview: "Nahled stranky",
            current_prefix: "Aktualni",
            no_document_loaded: "Dokument neni nacten",
            page_prefix: "Stranka",
            license_prefix: "Licence",
            expires_prefix: "Plati do",
            open_recent: "Nedavne soubory",
            no_recent_files: "Zadne nedavne soubory",
            click_image_to_edit: "Kliknete na obrazek pro upravu",
            width_short: "S",
            height_short: "V",
            lock_aspect: "Zamknout pomer",
            apply: "Pouzit",
            text: "Text",
            font: "Pismo",
            size: "Velikost",
            insert: "Vlozit",
            close: "Zavrit",
            zoom: "Priblizeni",
            menu_file: "Soubor",
            menu_edit: "Upravit",
            menu_view: "Zobrazeni",
            menu_annotate: "Anotace",
            menu_page: "Stranka",
            menu_tools: "Nastroje",
            menu_license: "Licence",
            open: "Otevrit",
            save: "Ulozit",
            save_as: "Ulozit jako",
            undo: "Zpet",
            redo: "Znovu",
            reset_100: "Reset 100%",
            highlight: "Zvyraznit",
            note: "Poznamka",
            previous: "Predchozi",
            next: "Dalsi",
            delete_page: "Smazat stranku",
            rotate_page: "Otocit stranku",
            insert_text: "Vlozit text",
            font_substitute: "Nahrada pisma",
            insert_image: "Vlozit obrazek",
            set_password: "Nastavit heslo",
            redact: "Redigovat",
            apply_ocr: "Pouzit OCR",
            reorder_pages: "Preusporadat stranky",
            merge: "Sloucit",
            create_field: "Vytvorit pole",
            set_field_value: "Nastavit hodnotu pole",
            detect_fields: "Najit pole",
            export_fields: "Export poli",
            upgrade: "Upgrade",
            activate: "Aktivovat",
            status_ready: "Pripraveno",
            status_document_opened: "Dokument otevren",
            status_document_closed: "Dokument zavren",
            status_saved_prefix: "Ulozeno",
            status_page_modified: "Stranka upravena",
            status_error_prefix: "Chyba",
        }
    }

    fn pl() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Brak dokumentu",
            page_preview: "Podglad strony",
            current_prefix: "Biezaca",
            no_document_loaded: "Nie zaladowano dokumentu",
            page_prefix: "Strona",
            license_prefix: "Licencja",
            expires_prefix: "Wygasa",
            open_recent: "Ostatnie pliki",
            no_recent_files: "Brak ostatnich plikow",
            click_image_to_edit: "Kliknij obraz, aby edytowac",
            width_short: "S",
            height_short: "W",
            lock_aspect: "Zablokuj proporcje",
            apply: "Zastosuj",
            text: "Tekst",
            font: "Czcionka",
            size: "Rozmiar",
            insert: "Wstaw",
            close: "Zamknij",
            zoom: "Powiekszenie",
            menu_file: "Plik",
            menu_edit: "Edycja",
            menu_view: "Widok",
            menu_annotate: "Adnotacje",
            menu_page: "Strona",
            menu_tools: "Narzedzia",
            menu_license: "Licencja",
            open: "Otworz",
            save: "Zapisz",
            save_as: "Zapisz jako",
            undo: "Cofnij",
            redo: "Ponow",
            reset_100: "Reset 100%",
            highlight: "Podswietlenie",
            note: "Notatka",
            previous: "Poprzednia",
            next: "Nastepna",
            delete_page: "Usun strone",
            rotate_page: "Obroc strone",
            insert_text: "Wstaw tekst",
            font_substitute: "Podmiana czcionki",
            insert_image: "Wstaw obraz",
            set_password: "Ustaw haslo",
            redact: "Redaguj",
            apply_ocr: "Zastosuj OCR",
            reorder_pages: "Zmien kolejnosc stron",
            merge: "Scal",
            create_field: "Utworz pole",
            set_field_value: "Ustaw wartosc pola",
            detect_fields: "Wykryj pola",
            export_fields: "Eksportuj pola",
            upgrade: "Ulepsz",
            activate: "Aktywuj",
            status_ready: "Gotowe",
            status_document_opened: "Dokument otwarty",
            status_document_closed: "Dokument zamkniety",
            status_saved_prefix: "Zapisano",
            status_page_modified: "Strona zmodyfikowana",
            status_error_prefix: "Blad",
        }
    }

    fn es() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Sin documento",
            page_preview: "Vista previa",
            current_prefix: "Actual",
            no_document_loaded: "Ningun documento cargado",
            page_prefix: "Pagina",
            license_prefix: "Licencia",
            expires_prefix: "Expira",
            open_recent: "Recientes",
            no_recent_files: "No hay archivos recientes",
            click_image_to_edit: "Haz clic en la imagen para editar",
            width_short: "A",
            height_short: "H",
            lock_aspect: "Bloquear proporcion",
            apply: "Aplicar",
            text: "Texto",
            font: "Fuente",
            size: "Tamano",
            insert: "Insertar",
            close: "Cerrar",
            zoom: "Zoom",
            menu_file: "Archivo",
            menu_edit: "Editar",
            menu_view: "Ver",
            menu_annotate: "Anotar",
            menu_page: "Pagina",
            menu_tools: "Herramientas",
            menu_license: "Licencia",
            open: "Abrir",
            save: "Guardar",
            save_as: "Guardar como",
            undo: "Deshacer",
            redo: "Rehacer",
            reset_100: "Restablecer 100%",
            highlight: "Resaltar",
            note: "Nota",
            previous: "Anterior",
            next: "Siguiente",
            delete_page: "Eliminar pagina",
            rotate_page: "Girar pagina",
            insert_text: "Insertar texto",
            font_substitute: "Sustituir fuente",
            insert_image: "Insertar imagen",
            set_password: "Establecer contrasena",
            redact: "Redactar",
            apply_ocr: "Aplicar OCR",
            reorder_pages: "Reordenar paginas",
            merge: "Combinar",
            create_field: "Crear campo",
            set_field_value: "Definir valor del campo",
            detect_fields: "Detectar campos",
            export_fields: "Exportar campos",
            upgrade: "Mejorar",
            activate: "Activar",
            status_ready: "Listo",
            status_document_opened: "Documento abierto",
            status_document_closed: "Documento cerrado",
            status_saved_prefix: "Guardado",
            status_page_modified: "Pagina modificada",
            status_error_prefix: "Error",
        }
    }

    fn fr() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Aucun document",
            page_preview: "Apercu de page",
            current_prefix: "Actuelle",
            no_document_loaded: "Aucun document charge",
            page_prefix: "Page",
            license_prefix: "Licence",
            expires_prefix: "Expire",
            open_recent: "Recents",
            no_recent_files: "Aucun fichier recent",
            click_image_to_edit: "Cliquez sur l'image pour modifier",
            width_short: "L",
            height_short: "H",
            lock_aspect: "Verrouiller les proportions",
            apply: "Appliquer",
            text: "Texte",
            font: "Police",
            size: "Taille",
            insert: "Inserer",
            close: "Fermer",
            zoom: "Zoom",
            menu_file: "Fichier",
            menu_edit: "Edition",
            menu_view: "Affichage",
            menu_annotate: "Annotations",
            menu_page: "Page",
            menu_tools: "Outils",
            menu_license: "Licence",
            open: "Ouvrir",
            save: "Enregistrer",
            save_as: "Enregistrer sous",
            undo: "Annuler",
            redo: "Retablir",
            reset_100: "Reinitialiser 100%",
            highlight: "Surligner",
            note: "Note",
            previous: "Precedente",
            next: "Suivante",
            delete_page: "Supprimer la page",
            rotate_page: "Pivoter la page",
            insert_text: "Inserer du texte",
            font_substitute: "Remplacer la police",
            insert_image: "Inserer une image",
            set_password: "Definir un mot de passe",
            redact: "Rediger",
            apply_ocr: "Appliquer OCR",
            reorder_pages: "Reordonner les pages",
            merge: "Fusionner",
            create_field: "Creer un champ",
            set_field_value: "Definir la valeur du champ",
            detect_fields: "Detecter les champs",
            export_fields: "Exporter les champs",
            upgrade: "Mettre a niveau",
            activate: "Activer",
            status_ready: "Pret",
            status_document_opened: "Document ouvert",
            status_document_closed: "Document ferme",
            status_saved_prefix: "Enregistre",
            status_page_modified: "Page modifiee",
            status_error_prefix: "Erreur",
        }
    }

    fn it() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Nessun documento",
            page_preview: "Anteprima pagina",
            current_prefix: "Corrente",
            no_document_loaded: "Nessun documento caricato",
            page_prefix: "Pagina",
            license_prefix: "Licenza",
            expires_prefix: "Scade",
            open_recent: "Recenti",
            no_recent_files: "Nessun file recente",
            click_image_to_edit: "Fai clic sull'immagine per modificare",
            width_short: "L",
            height_short: "A",
            lock_aspect: "Blocca proporzioni",
            apply: "Applica",
            text: "Testo",
            font: "Carattere",
            size: "Dimensione",
            insert: "Inserisci",
            close: "Chiudi",
            zoom: "Zoom",
            menu_file: "File",
            menu_edit: "Modifica",
            menu_view: "Vista",
            menu_annotate: "Annota",
            menu_page: "Pagina",
            menu_tools: "Strumenti",
            menu_license: "Licenza",
            open: "Apri",
            save: "Salva",
            save_as: "Salva con nome",
            undo: "Annulla",
            redo: "Ripeti",
            reset_100: "Reimposta 100%",
            highlight: "Evidenzia",
            note: "Nota",
            previous: "Precedente",
            next: "Successiva",
            delete_page: "Elimina pagina",
            rotate_page: "Ruota pagina",
            insert_text: "Inserisci testo",
            font_substitute: "Sostituisci carattere",
            insert_image: "Inserisci immagine",
            set_password: "Imposta password",
            redact: "Oscura",
            apply_ocr: "Applica OCR",
            reorder_pages: "Riordina pagine",
            merge: "Unisci",
            create_field: "Crea campo",
            set_field_value: "Imposta valore campo",
            detect_fields: "Rileva campi",
            export_fields: "Esporta campi",
            upgrade: "Aggiorna",
            activate: "Attiva",
            status_ready: "Pronto",
            status_document_opened: "Documento aperto",
            status_document_closed: "Documento chiuso",
            status_saved_prefix: "Salvato",
            status_page_modified: "Pagina modificata",
            status_error_prefix: "Errore",
        }
    }

    fn de() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Kein Dokument",
            page_preview: "Seitenvorschau",
            current_prefix: "Aktuell",
            no_document_loaded: "Kein Dokument geladen",
            page_prefix: "Seite",
            license_prefix: "Lizenz",
            expires_prefix: "Lauft ab",
            open_recent: "Zuletzt verwendet",
            no_recent_files: "Keine zuletzt verwendeten Dateien",
            click_image_to_edit: "Zum Bearbeiten auf das Bild klicken",
            width_short: "B",
            height_short: "H",
            lock_aspect: "Seitenverhaltnis sperren",
            apply: "Anwenden",
            text: "Text",
            font: "Schrift",
            size: "Grosse",
            insert: "Einfugen",
            close: "Schliessen",
            zoom: "Zoom",
            menu_file: "Datei",
            menu_edit: "Bearbeiten",
            menu_view: "Ansicht",
            menu_annotate: "Anmerken",
            menu_page: "Seite",
            menu_tools: "Werkzeuge",
            menu_license: "Lizenz",
            open: "Offnen",
            save: "Speichern",
            save_as: "Speichern unter",
            undo: "Ruckgangig",
            redo: "Wiederholen",
            reset_100: "Auf 100%",
            highlight: "Markieren",
            note: "Notiz",
            previous: "Vorherige",
            next: "Nachste",
            delete_page: "Seite loschen",
            rotate_page: "Seite drehen",
            insert_text: "Text einfugen",
            font_substitute: "Schrift ersetzen",
            insert_image: "Bild einfugen",
            set_password: "Passwort setzen",
            redact: "Schwarzen",
            apply_ocr: "OCR anwenden",
            reorder_pages: "Seiten neu ordnen",
            merge: "Zusammenfugen",
            create_field: "Feld erstellen",
            set_field_value: "Feldwert setzen",
            detect_fields: "Felder erkennen",
            export_fields: "Felder exportieren",
            upgrade: "Upgrade",
            activate: "Aktivieren",
            status_ready: "Bereit",
            status_document_opened: "Dokument geoffnet",
            status_document_closed: "Dokument geschlossen",
            status_saved_prefix: "Gespeichert",
            status_page_modified: "Seite geandert",
            status_error_prefix: "Fehler",
        }
    }

    fn pt() -> Self {
        Self {
            app_title: "Free PDF Editor",
            no_document: "Sem documento",
            page_preview: "Pre-visualizacao",
            current_prefix: "Atual",
            no_document_loaded: "Nenhum documento carregado",
            page_prefix: "Pagina",
            license_prefix: "Licenca",
            expires_prefix: "Expira",
            open_recent: "Recentes",
            no_recent_files: "Sem arquivos recentes",
            click_image_to_edit: "Clique na imagem para editar",
            width_short: "L",
            height_short: "A",
            lock_aspect: "Bloquear proporcao",
            apply: "Aplicar",
            text: "Texto",
            font: "Fonte",
            size: "Tamanho",
            insert: "Inserir",
            close: "Fechar",
            zoom: "Zoom",
            menu_file: "Arquivo",
            menu_edit: "Editar",
            menu_view: "Visualizar",
            menu_annotate: "Anotar",
            menu_page: "Pagina",
            menu_tools: "Ferramentas",
            menu_license: "Licenca",
            open: "Abrir",
            save: "Salvar",
            save_as: "Salvar como",
            undo: "Desfazer",
            redo: "Refazer",
            reset_100: "Redefinir 100%",
            highlight: "Destaque",
            note: "Nota",
            previous: "Anterior",
            next: "Proxima",
            delete_page: "Excluir pagina",
            rotate_page: "Girar pagina",
            insert_text: "Inserir texto",
            font_substitute: "Substituir fonte",
            insert_image: "Inserir imagem",
            set_password: "Definir senha",
            redact: "Redigir",
            apply_ocr: "Aplicar OCR",
            reorder_pages: "Reordenar paginas",
            merge: "Mesclar",
            create_field: "Criar campo",
            set_field_value: "Definir valor do campo",
            detect_fields: "Detectar campos",
            export_fields: "Exportar campos",
            upgrade: "Upgrade",
            activate: "Ativar",
            status_ready: "Pronto",
            status_document_opened: "Documento aberto",
            status_document_closed: "Documento fechado",
            status_saved_prefix: "Salvo",
            status_page_modified: "Pagina modificada",
            status_error_prefix: "Erro",
        }
    }
}
