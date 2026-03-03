import { describe, it, expect, vi, beforeEach } from 'vitest';

const { mockTextField, mockCheckBox, mockDropdown, mockRadioGroup, mockOtherField, mockDoc } =
  vi.hoisted(() => {
    const mockTextField = { getName: vi.fn().mockReturnValue('name'), getText: vi.fn().mockReturnValue('Alice'), setText: vi.fn() };
    const mockCheckBox = { getName: vi.fn().mockReturnValue('agree'), isChecked: vi.fn().mockReturnValue(true), check: vi.fn(), uncheck: vi.fn() };
    const mockDropdown = { getName: vi.fn().mockReturnValue('country'), getSelected: vi.fn().mockReturnValue(['US']), select: vi.fn() };
    const mockRadioGroup = { getName: vi.fn().mockReturnValue('gender'), getSelected: vi.fn().mockReturnValue('male') };
    const mockOtherField = { getName: vi.fn().mockReturnValue('unknown') };

    const mockForm = {
      getFields: vi.fn().mockReturnValue([]),
      getField: vi.fn(),
    };
    const mockDoc = {
      getForm: vi.fn().mockReturnValue(mockForm),
      save: vi.fn().mockResolvedValue(new Uint8Array([9, 8, 7])),
    };
    return { mockTextField, mockCheckBox, mockDropdown, mockRadioGroup, mockOtherField, mockDoc };
  });

vi.mock('pdf-lib', async () => {
  class FakePDFTextField { getName() { return mockTextField.getName(); } getText() { return mockTextField.getText(); } setText(v: string) { return mockTextField.setText(v); } }
  class FakePDFCheckBox { getName() { return mockCheckBox.getName(); } isChecked() { return mockCheckBox.isChecked(); } check() { return mockCheckBox.check(); } uncheck() { return mockCheckBox.uncheck(); } }
  class FakePDFDropdown { getName() { return mockDropdown.getName(); } getSelected() { return mockDropdown.getSelected(); } select(v: string) { return mockDropdown.select(v); } }
  class FakePDFRadioGroup { getName() { return mockRadioGroup.getName(); } getSelected() { return mockRadioGroup.getSelected(); } }

  // Make instanceof checks work by setting up the fakes as the exported classes
  Object.setPrototypeOf(mockTextField, FakePDFTextField.prototype);
  Object.setPrototypeOf(mockCheckBox, FakePDFCheckBox.prototype);
  Object.setPrototypeOf(mockDropdown, FakePDFDropdown.prototype);
  Object.setPrototypeOf(mockRadioGroup, FakePDFRadioGroup.prototype);

  return {
    PDFDocument: { load: vi.fn().mockResolvedValue(mockDoc) },
    PDFTextField: FakePDFTextField,
    PDFCheckBox: FakePDFCheckBox,
    PDFDropdown: FakePDFDropdown,
    PDFRadioGroup: FakePDFRadioGroup,
  };
});

import { PDFDocument } from 'pdf-lib';
import { getFormFields, fillFormFields } from './formsService';

describe('formsService', () => {
  const mockForm = mockDoc.getForm();

  beforeEach(() => {
    vi.clearAllMocks();
    (PDFDocument.load as ReturnType<typeof vi.fn>).mockResolvedValue(mockDoc);
    mockDoc.getForm.mockReturnValue(mockForm);
    mockDoc.save.mockResolvedValue(new Uint8Array([9, 8, 7]));
  });

  describe('getFormFields', () => {
    it('loads the PDF with ignoreEncryption: true', async () => {
      mockForm.getFields.mockReturnValue([]);
      const buf = new ArrayBuffer(8);
      await getFormFields(buf);
      expect(PDFDocument.load).toHaveBeenCalledWith(buf, { ignoreEncryption: true });
    });

    it('returns an empty array when there are no fields', async () => {
      mockForm.getFields.mockReturnValue([]);
      const result = await getFormFields(new ArrayBuffer(8));
      expect(result).toEqual([]);
    });

    it('maps a text field correctly', async () => {
      mockForm.getFields.mockReturnValue([mockTextField]);
      const result = await getFormFields(new ArrayBuffer(8));
      expect(result).toEqual([{ name: 'name', type: 'text', value: 'Alice' }]);
    });

    it('maps a checkbox field correctly', async () => {
      mockForm.getFields.mockReturnValue([mockCheckBox]);
      const result = await getFormFields(new ArrayBuffer(8));
      expect(result).toEqual([{ name: 'agree', type: 'checkbox', value: 'checked' }]);
    });

    it('maps a dropdown field correctly', async () => {
      mockForm.getFields.mockReturnValue([mockDropdown]);
      const result = await getFormFields(new ArrayBuffer(8));
      expect(result).toEqual([{ name: 'country', type: 'dropdown', value: 'US' }]);
    });

    it('maps a radio group field correctly', async () => {
      mockForm.getFields.mockReturnValue([mockRadioGroup]);
      const result = await getFormFields(new ArrayBuffer(8));
      expect(result).toEqual([{ name: 'gender', type: 'radio', value: 'male' }]);
    });
  });

  describe('fillFormFields', () => {
    it('loads the PDF with ignoreEncryption: true', async () => {
      mockForm.getField.mockReturnValue(mockTextField);
      const buf = new ArrayBuffer(8);
      await fillFormFields(buf, { name: 'Bob' });
      expect(PDFDocument.load).toHaveBeenCalledWith(buf, { ignoreEncryption: true });
    });

    it('sets text on a text field', async () => {
      mockForm.getField.mockReturnValue(mockTextField);
      await fillFormFields(new ArrayBuffer(8), { name: 'Bob' });
      expect(mockTextField.setText).toHaveBeenCalledWith('Bob');
    });

    it('checks a checkbox when value is "checked"', async () => {
      mockForm.getField.mockReturnValue(mockCheckBox);
      await fillFormFields(new ArrayBuffer(8), { agree: 'checked' });
      expect(mockCheckBox.check).toHaveBeenCalled();
    });

    it('unchecks a checkbox when value is "unchecked"', async () => {
      mockForm.getField.mockReturnValue(mockCheckBox);
      await fillFormFields(new ArrayBuffer(8), { agree: 'unchecked' });
      expect(mockCheckBox.uncheck).toHaveBeenCalled();
    });

    it('selects a dropdown option', async () => {
      mockForm.getField.mockReturnValue(mockDropdown);
      await fillFormFields(new ArrayBuffer(8), { country: 'CA' });
      expect(mockDropdown.select).toHaveBeenCalledWith('CA');
    });

    it('returns a Uint8Array', async () => {
      mockForm.getField.mockReturnValue(mockTextField);
      const result = await fillFormFields(new ArrayBuffer(8), { name: 'Bob' });
      expect(result).toBeInstanceOf(Uint8Array);
    });

    it('silently skips unknown fields', async () => {
      mockForm.getField.mockImplementation(() => { throw new Error('not found'); });
      await expect(fillFormFields(new ArrayBuffer(8), { missing: 'value' })).resolves.not.toThrow();
    });
  });
});
