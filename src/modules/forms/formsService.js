import { PDFDocument, PDFTextField, PDFCheckBox, PDFDropdown, PDFRadioGroup } from 'pdf-lib';

export async function getFormFields(sourceBytes) {
  const pdfDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const form = pdfDoc.getForm();
  return form.getFields().map((field) => {
    const name = field.getName();
    if (field instanceof PDFTextField) {
      return { name, type: 'text', value: field.getText() ?? '' };
    }
    if (field instanceof PDFCheckBox) {
      return { name, type: 'checkbox', value: field.isChecked() ? 'checked' : 'unchecked' };
    }
    if (field instanceof PDFDropdown) {
      return { name, type: 'dropdown', value: field.getSelected().join(', ') };
    }
    if (field instanceof PDFRadioGroup) {
      return { name, type: 'radio', value: field.getSelected() ?? '' };
    }
    return { name, type: 'other', value: '' };
  });
}

export async function fillFormFields(sourceBytes, values) {
  const pdfDoc = await PDFDocument.load(sourceBytes, { ignoreEncryption: true });
  const form = pdfDoc.getForm();
  for (const [name, value] of Object.entries(values)) {
    try {
      const field = form.getField(name);
      if (field instanceof PDFTextField) {
        field.setText(value);
      } else if (field instanceof PDFCheckBox) {
        if (value === 'checked') field.check();
        else field.uncheck();
      } else if (field instanceof PDFDropdown && value) {
        field.select(value);
      }
    } catch {
      // field not found or wrong type – skip silently
    }
  }
  return pdfDoc.save();
}
