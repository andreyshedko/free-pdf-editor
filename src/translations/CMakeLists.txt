set(TRANSLATION_FILES
    pdfditor_en_US.ts
    pdfditor_es.ts
    pdfditor_fr.ts
    pdfditor_de.ts
    pdfditor_cs.ts
    pdfditor_pl.ts
    pdfditor_ru.ts
)

# Generate .qm (compiled translation) files from .ts files
qt6_add_translations(pdf-editor
    TS_FILES ${TRANSLATION_FILES}
    RESOURCE_PREFIX "/translations"
)
