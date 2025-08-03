import React, { useEffect } from 'react';
import { Token } from '../types';
import CameraIcon from './icons/CameraIcon';
import XMarkIcon from './icons/XMarkIcon';

interface QRScannerModalProps {
  token: Token;
  onClose: () => void;
}

function QRScannerModal({ token, onClose }: QRScannerModalProps): React.ReactNode {
  useEffect(() => {
    // Hack to get viewport height for animation on mobile browsers
    const setVh = () => {
      const vh = window.innerHeight * 0.01;
      document.documentElement.style.setProperty('--vh', `${vh}px`);
    };

    setVh();

    window.addEventListener('resize', setVh);

    return () => {
      window.removeEventListener('resize', setVh);
      document.documentElement.style.removeProperty('--vh');
    };
  }, []);

  return (
    <div 
      className="fixed inset-0 bg-black/80 flex items-center justify-center z-50 p-4 animate-fade-in"
      aria-labelledby="qr-scanner-title"
      role="dialog"
      aria-modal="true"
      onClick={onClose}
    >
      <div 
        className="bg-zinc-800 border border-zinc-700 rounded-2xl p-6 sm:p-8 w-full max-w-sm text-center transform transition-all"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex justify-between items-center mb-4">
            <h2 id="qr-scanner-title" className="font-display text-2xl text-white">Canjear {token.name}</h2>
            <button 
                onClick={onClose} 
                className="text-zinc-500 hover:text-white transition-colors"
                aria-label="Cerrar"
            >
                <XMarkIcon className="w-7 h-7" />
            </button>
        </div>
        
        <p className="text-zinc-400 mb-6">
          Apunte la cámara al código QR en el mostrador para recibir su {token.code === 'SLICE' ? 'rebanada de pizza' : 'bebida'}.
        </p>

        <div className="relative aspect-square bg-black rounded-lg overflow-hidden border border-zinc-700 flex flex-col items-center justify-center p-4">
            <CameraIcon className="w-16 h-16 text-zinc-600" />
            <p className="mt-2 text-zinc-500 text-sm">Cámara no disponible</p>
            
            <div className="absolute top-0 left-0 right-0 h-1 bg-red-500 shadow-[0_0_15px_2px] shadow-red-500/70 animate-scan"></div>
        </div>

        <button
            onClick={onClose}
            className="mt-6 w-full bg-red-600 text-white font-bold py-3 px-4 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-800 focus:ring-red-500 transition-all duration-300"
        >
            Cerrar
        </button>
      </div>
      <style>{`
        @keyframes scan {
          0% { transform: translateY(-10%); }
          100% { transform: translateY(calc(var(--vh, 1vh) * 35)); }
        }
        .animate-scan {
            animation: scan 2.5s ease-in-out infinite;
        }
        @keyframes fade-in {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .animate-fade-in {
            animation: fade-in 0.3s ease-out;
        }
      `}</style>
    </div>
  );
}

export default QRScannerModal;
