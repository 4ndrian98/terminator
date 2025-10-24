import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Key, CheckCircle, Loader2 } from 'lucide-react';
import { Card } from '../components/Card';
import { Button } from '../components/Button';
import { Input } from '../components/Input';
import { activateLicense } from '../services/tauriApi';

export function ActivateLicense({ onActivated }) {
  const navigate = useNavigate();
  const [licenseKey, setLicenseKey] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState(false);

  const handleActivate = async () => {
    if (!licenseKey.trim()) {
      setError('Inserisci una chiave di licenza');
      return;
    }

    setLoading(true);
    setError('');

    try {
      const license = await activateLicense(licenseKey.trim());
      setSuccess(true);
      
      setTimeout(() => {
        if (onActivated) {
          onActivated(license);
        }
        navigate('/');
      }, 1500);
    } catch (err) {
      setError(err.message || 'Chiave di licenza non valida');
    } finally {
      setLoading(false);
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !loading) {
      handleActivate();
    }
  };

  if (success) {
    return (
      <div className="min-h-screen bg-gradient-to-br from-primary-50 to-primary-100 flex items-center justify-center p-8">
        <Card className="max-w-md w-full text-center py-12">
          <div className="w-24 h-24 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-6">
            <CheckCircle className="w-12 h-12 text-green-600" />
          </div>
          <h2 className="text-2xl font-bold text-gray-900 mb-2">
            ✅ Licenza Attivata!
          </h2>
          <p className="text-gray-600">
            Reindirizzamento alla dashboard...
          </p>
        </Card>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary-50 to-primary-100 flex items-center justify-center p-8">
      <Card className="max-w-md w-full">
        {/* Logo/Icon */}
        <div className="text-center mb-8">
          <div className="w-20 h-20 bg-gradient-to-br from-primary-500 to-primary-700 rounded-full flex items-center justify-center mx-auto mb-4">
            <Key className="w-10 h-10 text-white" />
          </div>
          <h1 className="text-2xl font-bold text-gray-900 mb-2">
            Attiva Licenza
          </h1>
          <p className="text-gray-600">
            Inserisci la tua chiave di licenza per continuare
          </p>
        </div>

        {/* Input Form */}
        <div className="space-y-6">
          <Input
            label="Chiave di Licenza"
            placeholder="WA-X-XXXXXXXX-XXXXXXXX"
            value={licenseKey}
            onChange={(e) => setLicenseKey(e.target.value)}
            onKeyPress={handleKeyPress}
            error={error}
            disabled={loading}
            className="font-mono text-center text-lg"
          />

          <Button
            onClick={handleActivate}
            variant="primary"
            className="w-full py-3 text-lg"
            loading={loading}
            disabled={!licenseKey.trim()}
          >
            {loading ? (
              <>
                <Loader2 className="w-5 h-5 animate-spin" />
                Verifica in corso...
              </>
            ) : (
              <>
                <Key className="w-5 h-5" />
                Attiva Licenza
              </>
            )}
          </Button>
        </div>

        {/* Info Box */}
        <Card className="mt-6 bg-blue-50 border-blue-200 p-4">
          <h3 className="text-sm font-semibold text-gray-900 mb-2">
            ℹ️ Come ottenere una licenza?
          </h3>
          <p className="text-sm text-gray-700">
            Contatta il fornitore del software per ricevere la tua chiave di licenza personale.
            La chiave ha il formato: <code className="font-mono bg-white px-1">WA-X-XXXXXXXX-XXXXXXXX</code>
          </p>
        </Card>

        {/* License Types Info */}
        <div className="mt-6 text-center text-sm text-gray-600">
          <p className="font-medium mb-2">Piani Disponibili:</p>
          <div className="flex justify-center gap-4 text-xs">
            <span>📅 Mensile</span>
            <span>📅 Trimestrale</span>
            <span>📅 Annuale</span>
          </div>
        </div>
      </Card>
    </div>
  );
}
