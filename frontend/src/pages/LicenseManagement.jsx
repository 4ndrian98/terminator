import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { ArrowLeft, Key, Calendar, CheckCircle, AlertTriangle, Loader2, RefreshCw, Trash2 } from 'lucide-react';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { Modal } from '../components/Modal';
import { Input } from '../components/Input';
import { getLicenseStatus, activateLicense, deactivateLicense } from '../services/tauriApi';

export function LicenseManagement() {
  const navigate = useNavigate();
  const [license, setLicense] = useState(null);
  const [loading, setLoading] = useState(true);
  const [showRenewModal, setShowRenewModal] = useState(false);
  const [showDeactivateModal, setShowDeactivateModal] = useState(false);
  const [newLicenseKey, setNewLicenseKey] = useState('');
  const [renewLoading, setRenewLoading] = useState(false);
  const [deactivateLoading, setDeactivateLoading] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    loadLicense();
  }, []);

  const loadLicense = async () => {
    setLoading(true);
    try {
      const licenseData = await getLicenseStatus();
      setLicense(licenseData);
    } catch (err) {
      console.error('Errore caricamento licenza:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleRenew = async () => {
    if (!newLicenseKey.trim()) {
      setError('Inserisci una nuova chiave di licenza');
      return;
    }

    setRenewLoading(true);
    setError('');

    try {
      const newLicense = await activateLicense(newLicenseKey.trim());
      setLicense(newLicense);
      setShowRenewModal(false);
      setNewLicenseKey('');
    } catch (err) {
      setError(err.message || 'Chiave non valida');
    } finally {
      setRenewLoading(false);
    }
  };

  const handleDeactivate = async () => {
    setDeactivateLoading(true);
    try {
      await deactivateLicense();
      setLicense(null);
      setShowDeactivateModal(false);
      // Redirect to activation page
      setTimeout(() => {
        window.location.reload();
      }, 500);
    } catch (err) {
      console.error('Errore disattivazione:', err);
    } finally {
      setDeactivateLoading(false);
    }
  };

  const getLicenseTypeName = (type) => {
    const types = {
      'Monthly': 'Mensile',
      'Quarterly': 'Trimestrale',
      'Annual': 'Annuale',
    };
    return types[type] || type;
  };

  const formatDate = (dateStr) => {
    if (!dateStr || dateStr.length !== 8) return 'N/A';
    const year = dateStr.substring(0, 4);
    const month = dateStr.substring(4, 6);
    const day = dateStr.substring(6, 8);
    return `${day}/${month}/${year}`;
  };

  const getDaysRemaining = (expirationDate) => {
    if (!expirationDate || expirationDate.length !== 8) return 0;
    
    const year = parseInt(expirationDate.substring(0, 4));
    const month = parseInt(expirationDate.substring(4, 6)) - 1;
    const day = parseInt(expirationDate.substring(6, 8));
    
    const expDate = new Date(year, month, day);
    const today = new Date();
    today.setHours(0, 0, 0, 0);
    
    const diffTime = expDate - today;
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
    
    return diffDays;
  };

  const isValid = license ? getDaysRemaining(license.expiration_date) >= 0 : false;
  const daysRemaining = license ? getDaysRemaining(license.expiration_date) : 0;

  if (loading) {
    return (
      <div className="max-w-4xl mx-auto p-8">
        <Card className="flex items-center justify-center py-12">
          <Loader2 className="w-8 h-8 animate-spin text-primary-600" />
          <p className="ml-3 text-gray-600">Caricamento licenza...</p>
        </Card>
      </div>
    );
  }

  if (!license) {
    return (
      <div className="max-w-4xl mx-auto p-8">
        <Card className="text-center py-12">
          <div className="w-20 h-20 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-6">
            <AlertTriangle className="w-10 h-10 text-red-600" />
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-4">
            Nessuna Licenza Attiva
          </h2>
          <p className="text-gray-600 mb-6">
            Non è stata trovata alcuna licenza attiva. Inserisci una chiave per continuare.
          </p>
          <Button onClick={() => window.location.reload()} variant="primary">
            <Key className="w-5 h-5" />
            Attiva Licenza
          </Button>
        </Card>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto p-8">
      {/* Header */}
      <div className="mb-8">
        <Button onClick={() => navigate('/')} variant="ghost" className="mb-4">
          <ArrowLeft className="w-4 h-4" />
          Torna alla Dashboard
        </Button>
        <h1 className="text-3xl font-bold text-gray-900 mb-2">
          Gestione Licenza
        </h1>
        <p className="text-gray-600">
          Visualizza e gestisci la tua licenza attiva
        </p>
      </div>

      {/* License Status Card */}
      <Card className={`mb-8 ${isValid ? 'border-green-200 bg-green-50' : 'border-red-200 bg-red-50'}`}>
        <div className="flex items-start justify-between">
          <div className="flex-1">
            <div className="flex items-center gap-3 mb-4">
              {isValid ? (
                <CheckCircle className="w-8 h-8 text-green-600" />
              ) : (
                <AlertTriangle className="w-8 h-8 text-red-600" />
              )}
              <div>
                <h2 className="text-2xl font-bold text-gray-900">
                  {isValid ? 'Licenza Attiva' : 'Licenza Scaduta'}
                </h2>
                <p className={`text-sm ${isValid ? 'text-green-700' : 'text-red-700'}`}>
                  {isValid 
                    ? daysRemaining > 7 
                      ? 'Tutto funziona correttamente' 
                      : '⚠️ Licenza in scadenza'
                    : 'Rinnova la licenza per continuare'}
                </p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-6">
              <div>
                <p className="text-sm text-gray-600 mb-1">Tipo Licenza</p>
                <p className="text-lg font-semibold text-gray-900">
                  {getLicenseTypeName(license.license_type)}
                </p>
              </div>
              <div>
                <p className="text-sm text-gray-600 mb-1">Data Scadenza</p>
                <p className="text-lg font-semibold text-gray-900">
                  {formatDate(license.expiration_date)}
                </p>
              </div>
              <div>
                <p className="text-sm text-gray-600 mb-1">Giorni Rimanenti</p>
                <p className={`text-3xl font-bold ${
                  daysRemaining > 30 ? 'text-green-600' : 
                  daysRemaining > 7 ? 'text-yellow-600' : 
                  daysRemaining >= 0 ? 'text-orange-600' : 'text-red-600'
                }`}>
                  {daysRemaining >= 0 ? daysRemaining : 0}
                </p>
              </div>
              <div>
                <p className="text-sm text-gray-600 mb-1">Chiave Licenza</p>
                <p className="text-sm font-mono text-gray-900 break-all">
                  {license.license_key}
                </p>
              </div>
            </div>
          </div>
        </div>
      </Card>

      {/* Progress Bar */}
      {isValid && (
        <Card className="mb-8">
          <p className="text-sm text-gray-600 mb-2">Periodo di validità</p>
          <div className="w-full bg-gray-200 rounded-full h-3">
            <div
              className={`h-3 rounded-full transition-all ${
                daysRemaining > 30 ? 'bg-green-500' :
                daysRemaining > 7 ? 'bg-yellow-500' : 'bg-orange-500'
              }`}
              style={{
                width: `${Math.min(100, (daysRemaining / 365) * 100)}%`
              }}
            />
          </div>
          <p className="text-xs text-gray-500 mt-1">
            {daysRemaining} {daysRemaining === 1 ? 'giorno' : 'giorni'} rimanenti
          </p>
        </Card>
      )}

      {/* Actions */}
      <div className="grid grid-cols-2 gap-4 mb-8">
        <Button
          onClick={() => setShowRenewModal(true)}
          variant="primary"
          className="py-3"
        >
          <RefreshCw className="w-5 h-5" />
          {isValid ? 'Rinnova Licenza' : 'Attiva Nuova Licenza'}
        </Button>
        <Button
          onClick={() => setShowDeactivateModal(true)}
          variant="danger"
          className="py-3"
        >
          <Trash2 className="w-5 h-5" />
          Disattiva Licenza
        </Button>
      </div>

      {/* Info Box */}
      <Card className="bg-blue-50 border-blue-200">
        <h3 className="text-lg font-semibold text-gray-900 mb-3">
          ℹ️ Informazioni Licenza
        </h3>
        <ul className="space-y-2 text-sm text-gray-700">
          <li>• La licenza è valida fino alla data di scadenza indicata</li>
          <li>• Dopo la scadenza, non potrai più eseguire workflow (ma potrai visualizzarli)</li>
          <li>• Per rinnovare, inserisci una nuova chiave di licenza</li>
          <li>• Contatta il fornitore per acquistare una licenza o per assistenza</li>
        </ul>
      </Card>

      {/* Renew Modal */}
      <Modal
        isOpen={showRenewModal}
        onClose={() => {
          setShowRenewModal(false);
          setNewLicenseKey('');
          setError('');
        }}
        title={isValid ? 'Rinnova Licenza' : 'Attiva Nuova Licenza'}
        footer={
          <>
            <Button
              variant="secondary"
              onClick={() => {
                setShowRenewModal(false);
                setNewLicenseKey('');
                setError('');
              }}
              className="flex-1"
            >
              Annulla
            </Button>
            <Button
              variant="primary"
              onClick={handleRenew}
              loading={renewLoading}
              className="flex-1"
            >
              Attiva
            </Button>
          </>
        }
      >
        <div className="space-y-4">
          <p className="text-gray-700">
            {isValid 
              ? 'Inserisci una nuova chiave per rinnovare o estendere la licenza.'
              : 'Inserisci una nuova chiave per riattivare l\'applicazione.'}
          </p>
          <Input
            label="Nuova Chiave di Licenza"
            placeholder="WA-X-XXXXXXXX-XXXXXXXX"
            value={newLicenseKey}
            onChange={(e) => setNewLicenseKey(e.target.value)}
            error={error}
            className="font-mono text-center"
          />
        </div>
      </Modal>

      {/* Deactivate Modal */}
      <Modal
        isOpen={showDeactivateModal}
        onClose={() => setShowDeactivateModal(false)}
        title="Conferma Disattivazione"
        footer={
          <>
            <Button
              variant="secondary"
              onClick={() => setShowDeactivateModal(false)}
              className="flex-1"
            >
              Annulla
            </Button>
            <Button
              variant="danger"
              onClick={handleDeactivate}
              loading={deactivateLoading}
              className="flex-1"
            >
              Disattiva
            </Button>
          </>
        }
      >
        <p className="text-gray-700">
          Sei sicuro di voler disattivare la licenza corrente?
          <br /><br />
          Dovrai inserire una nuova chiave per continuare a usare l'applicazione.
        </p>
      </Modal>
    </div>
  );
}
