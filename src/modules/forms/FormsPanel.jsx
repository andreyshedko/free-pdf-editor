import React, { useState, useEffect } from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { getFormFields, fillFormFields } from './formsService';

export const FormsPanel = ({ onClose }) => {
  const { fileData, updateFileData } = usePdfDocumentStore();
  const [fields, setFields] = useState([]);
  const [values, setValues] = useState({});
  const [loading, setLoading] = useState(false);
  const [status, setStatus] = useState('');

  useEffect(() => {
    if (!fileData) return;
    setLoading(true);
    setStatus('');
    getFormFields(fileData)
      .then((f) => {
        setFields(f);
        const initial = {};
        f.forEach((field) => { initial[field.name] = field.value; });
        setValues(initial);
      })
      .catch((e) => {
        setStatus('Error reading form: ' + (e instanceof Error ? e.message : String(e)));
      })
      .finally(() => setLoading(false));
  }, [fileData]);

  const handleFill = async () => {
    if (!fileData) return;
    setStatus('Applying…');
    try {
      const newBytes = await fillFormFields(fileData, values);
      updateFileData(
        newBytes.buffer.slice(
          newBytes.byteOffset,
          newBytes.byteOffset + newBytes.byteLength,
        ),
      );
      setStatus('Form fields applied!');
    } catch (e) {
      setStatus('Error: ' + (e instanceof Error ? e.message : String(e)));
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-panel" style={{ width: 500, maxHeight: '80vh' }}>
        <div className="modal-header">
          <span>📋 Fill Form Fields</span>
          <button className="btn btn-ghost" onClick={onClose}>✕</button>
        </div>
        <div style={{ padding: '1rem', overflowY: 'auto', maxHeight: '55vh', display: 'flex', flexDirection: 'column', gap: '0.6rem' }}>
          {loading && (
            <span style={{ color: '#22c55e', fontSize: '0.8rem' }}>Reading form fields…</span>
          )}
          {!loading && fields.length === 0 && (
            <p style={{ color: 'var(--text-muted)', fontSize: '0.8rem', margin: 0 }}>
              No fillable form fields found in this PDF.
            </p>
          )}
          {fields.map((field) => (
            <div key={field.name} style={{ display: 'flex', flexDirection: 'column', gap: '0.2rem' }}>
              <label style={{ fontSize: '0.75rem', color: 'var(--text-muted)' }}>
                {field.name}{' '}
                <span style={{ opacity: 0.6 }}>({field.type})</span>
              </label>
              {field.type === 'checkbox' ? (
                <label style={{ display: 'flex', alignItems: 'center', gap: '0.4rem', fontSize: '0.8rem', cursor: 'pointer' }}>
                  <input
                    type="checkbox"
                    checked={values[field.name] === 'checked'}
                    onChange={(e) =>
                      setValues((v) => ({
                        ...v,
                        [field.name]: e.target.checked ? 'checked' : 'unchecked',
                      }))
                    }
                  />
                  {values[field.name] === 'checked' ? 'Checked' : 'Unchecked'}
                </label>
              ) : (
                <input
                  type="text"
                  className="text-input"
                  value={values[field.name] ?? ''}
                  onChange={(e) => setValues((v) => ({ ...v, [field.name]: e.target.value }))}
                  placeholder={`Enter ${field.name}…`}
                />
              )}
            </div>
          ))}
          {status && (
            <span style={{ fontSize: '0.75rem', color: status.startsWith('Error') ? 'var(--danger)' : '#22c55e' }}>
              {status}
            </span>
          )}
        </div>
        <div className="modal-actions">
          <button className="btn btn-ghost" onClick={onClose}>Close</button>
          <button
            className="btn btn-strong"
            onClick={handleFill}
            disabled={!fileData || fields.length === 0 || loading}
          >
            Apply to PDF
          </button>
        </div>
      </div>
    </div>
  );
};
