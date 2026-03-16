set(SRC_ROOT ${CMAKE_CURRENT_SOURCE_DIR})

add_library(overlay
    overlay/OverlayObject.h
    overlay/AnnotationObject.h
    overlay/AnnotationObject.cpp
    overlay/TextEditObject.h
    overlay/TextEditObject.cpp
    overlay/ImageObject.h
    overlay/ImageObject.cpp
    overlay/ShapeObject.h
    overlay/ShapeObject.cpp
)

target_include_directories(overlay PUBLIC ${SRC_ROOT})
target_link_libraries(overlay PUBLIC Qt6::Core Qt6::Gui)

add_library(document
    document/DocumentMetadata.h
    document/PageModel.h
    document/PageModel.cpp
    document/Document.h
    document/Document.cpp
)

target_include_directories(document PUBLIC ${SRC_ROOT})
target_link_libraries(document PUBLIC Qt6::Core Qt6::Gui overlay)

add_library(cache
    cache/PageRenderCache.h
    cache/PageRenderCache.cpp
)

target_include_directories(cache PUBLIC ${SRC_ROOT})
target_link_libraries(cache PUBLIC Qt6::Core Qt6::Gui)

add_library(pdf_engine
    pdf_engine/PdfiumBridge.h
    pdf_engine/PdfiumBridge.cpp
    pdf_engine/PdfRenderer.h
    pdf_engine/PdfRenderer.cpp
    pdf_engine/PdfWriter.h
    pdf_engine/PdfWriter.cpp
)

target_include_directories(pdf_engine PUBLIC ${SRC_ROOT})
target_link_libraries(pdf_engine PUBLIC Qt6::Core Qt6::Gui cache document)

add_library(ocr
    ocr/OCRProcessor.h
    ocr/OCRProcessor.cpp
)

target_include_directories(ocr PUBLIC ${SRC_ROOT})
target_link_libraries(ocr PUBLIC Qt6::Core Qt6::Gui)

add_library(utils
    utils/Logger.h
    utils/Logger.cpp
    utils/FileUtils.h
    utils/FileUtils.cpp
)

target_include_directories(utils PUBLIC ${SRC_ROOT})
target_link_libraries(utils PUBLIC Qt6::Core Qt6::Gui)

add_library(editor
    editor/Command.h
    editor/UndoStack.h
    editor/UndoStack.cpp
    editor/SelectionManager.h
    editor/SelectionManager.cpp
    editor/EditorController.h
    editor/EditorController.cpp
    editor/commands/AddAnnotationCommand.h
    editor/commands/AddAnnotationCommand.cpp
    editor/commands/AddTextEditCommand.h
    editor/commands/AddTextEditCommand.cpp
    editor/commands/DeletePageCommand.h
    editor/commands/DeletePageCommand.cpp
    editor/commands/InsertImageCommand.h
    editor/commands/InsertImageCommand.cpp
    editor/commands/MoveObjectCommand.h
    editor/commands/MoveObjectCommand.cpp
    editor/commands/ReplaceTextCommand.h
    editor/commands/ReplaceTextCommand.cpp
)

target_include_directories(editor PUBLIC ${SRC_ROOT})
target_link_libraries(editor PUBLIC Qt6::Core Qt6::Gui document pdf_engine ocr utils)

add_library(ui
    resources/assets.qrc
    ui/Toolbar.h
    ui/Toolbar.cpp
    ui/ThumbnailPanel.h
    ui/ThumbnailPanel.cpp
    ui/InspectorPanel.h
    ui/InspectorPanel.cpp
    ui/PageView.h
    ui/PageView.cpp
    ui/MainWindow.h
    ui/MainWindow.cpp
)

target_include_directories(ui PUBLIC ${SRC_ROOT})
target_link_libraries(ui PUBLIC Qt6::Core Qt6::Gui Qt6::Widgets editor)

add_library(app
    app/Application.h
    app/Application.cpp
)

target_include_directories(app PUBLIC ${SRC_ROOT})
target_link_libraries(app PUBLIC Qt6::Core Qt6::Gui Qt6::Widgets ui)

# Add translations
add_subdirectory(translations)

add_executable(pdf-editor
    app/main.cpp
)

target_link_libraries(pdf-editor PRIVATE app)
