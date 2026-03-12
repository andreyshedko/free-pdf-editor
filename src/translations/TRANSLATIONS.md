# PDF Editor Translations

This directory contains translation files for the PDF Editor application.

## Supported Languages

- **en_US** - English (US) [Default]
- **es** - Spanish
- **fr** - French
- **de** - German
- **cs** - Czech
- **pl** - Polish
- **ru** - Russian

## Translation Files

- `.ts` files: Translation source files (human-editable XML format)
- `.qm` files: Compiled translations (binary format, generated from .ts files)

## How Translations Work

1. **Source Code** — Strings in the application are wrapped with `tr("...")` macro
2. **lupdate** — Qt tool that extracts all translatable strings from C++ source code and updates `.ts` files
3. **Translation** — Translators edit `.ts` files to add translations for each language
4. **lrelease** — Qt tool that compiles `.ts` files into binary `.qm` files
5. **Runtime** — The application loads `.qm` files based on system locale

## Building with Translations

### Automatic (CMake)

The CMake build system automatically handles translations if Qt6 LinguistTools are installed:

```bash
cmake -B build -S . -G "MinGW Makefiles"
cmake --build build -j4
```

This runs `qt6_add_translations()` which:
1. Compiles all `.ts` files to `.qm` files
2. Embeds `.qm` files into the application as resources

### Manual Translation Update

To extract new translatable strings from the source code:

```bash
# Windows
"C:\Qt\Tools\mingw1310_64\bin\lupdate.exe" -recursive src/ -ts src/translations/*.ts

# Linux/macOS
lupdate -recursive src/ -ts src/translations/*.ts
```

This updates the `.ts` files with new strings and marks old strings as deprecated.

## Editing Translations

### Using Qt Linguist GUI

```bash
# Windows
"C:\Qt\6.10.2\mingw_64\bin\linguist.exe" src/translations/pdfditor_es.ts

# Linux/macOS
linguist src/translations/pdfditor_es.ts
```

### Manual Edit

Edit the `.ts` files with any text editor. The format is:

```xml
<message>
    <source>English text</source>
    <translation>Translated text</translation>
</message>
```

## Compiling Translations

To compile `.ts` files to `.qm` files:

```bash
# Windows
"C:\Qt\6.10.2\mingw_64\bin\lrelease.exe" src/translations/*.ts

# Linux/macOS
lrelease src/translations/*.ts
```

This generates `.qm` files that the application loads at runtime.

## Adding a New Language

1. Create a new `.ts` file from a template:
   ```bash
   lupdate -recursive src/ -ts src/translations/pdfditor_XX.ts
   ```
   where `XX` is the language code (e.g., `it` for Italian)

2. Add the filename to `src/translations/CMakeLists.txt`:
   ```cmake
   set(TRANSLATION_FILES
       pdfditor_en_US.ts
       pdfditor_es.ts
       pdfditor_fr.ts
       # ... other languages ...
       pdfditor_XX.ts  # New language
   )
   ```

3. Translate strings using Qt Linguist or a text editor

4. Rebuild the project

## Runtime Translation

The application automatically selects the translation file based on **system locale**:

- System locale: `de_DE` → loads `pdfditor_de.ts` translation
- System locale: `es_ES` → loads `pdfditor_es.ts` translation
- System locale: `unknown` → falls back to English (built-in)

### Forcing a Specific Language (for testing)

Edit `src/app/Application.cpp` to hardcode a locale:

```cpp
const QString locale = "de_DE";  // Force German
```

## File Format Reference

### .ts File Example

```xml
<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE TS>
<TS version="2.1" language="es">
    <context>
        <name>MainWindow</name>
        <message>
            <source>&amp;File</source>
            <translation>&amp;Archivo</translation>
        </message>
    </context>
</TS>
```

- `language` attribute: BCP 47 language tag (e.g., `es`, `de_DE`, `pt_BR`)
- `source` tag: Original English string from code
- `translation` tag: Translated string (empty if not translated)

## Best Practices

1. **Keep .ts files in version control** — They're human-readable and track translation history
2. **Don't edit .qm files manually** — They're generated from .ts files
3. **Run lupdate regularly** — To pick up newly added strings in the code
4. **Use context names** — They help translators understand where strings appear
5. **Add comments** — Use `<comment>` tags in .ts files for translator notes:
   ```xml
   <message>
       <comment>Menu item in File menu</comment>
       <source>&amp;Open...</source>
       <translation>&amp;Abrir...</translation>
   </message>
   ```

## Troubleshooting

### Translations not loading

- Check system locale: `locale` (Linux) or `$env:USERLOCALE` (PowerShell)
- Ensure `.qm` files are generated from `.ts` files
- Check that Application.cpp loads translations on startup
- Verify translation file is in the app resources

### lupdate doesn't find strings

- Ensure strings are wrapped with `tr("...")` in C++ code (not `QStringLiteral()`)
- Run `lupdate` with verbose flag: `lupdate -verbose ...`
- Check that file paths in `-ts` argument are correct

### Partial translations

- Empty `<translation>` tags mean the string is not yet translated
- Use Qt Linguist to mark translations as complete
- Untranslated strings fall back to English

## Qt Documentation

- [Qt Internationalization (i18n)](https://doc.qt.io/qt-6/i18n-source-code.html)
- [Qt Linguist Manual](https://doc.qt.io/qt-6/linguist-manual.html)
- [Translating with .ts Files](https://doc.qt.io/qt-6/linguist-ts-file-format.html)
