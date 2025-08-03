import React from 'react';
import { Token } from '../types';
import PizzaSliceIcon from './icons/PizzaSliceIcon';
import BeverageIcon from './icons/BeverageIcon';
import CheckIcon from './icons/CheckIcon';

interface ConfirmationScreenProps {
  tokens: Token[];
  onReset: () => void;
}

const TOKEN_ICONS: { [key: string]: React.ReactNode } = {
    SLICE: <PizzaSliceIcon className="w-6 h-6 text-yellow-400" />,
    BEBIDA: <BeverageIcon className="w-6 h-6 text-cyan-400" />,
};

function ConfirmationScreen({ tokens, onReset }: ConfirmationScreenProps): React.ReactNode {
  return (
    <div className="bg-zinc-800 p-8 rounded-2xl shadow-2xl shadow-green-500/20 border border-zinc-700 text-center space-y-6">
      <div className="mx-auto bg-green-400 w-20 h-20 rounded-full flex items-center justify-center">
        <CheckIcon className="h-12 w-12 text-zinc-900" />
      </div>

      <div>
        <h1 className="font-display text-4xl text-white">¡Canje Exitoso!</h1>
        <p className="text-zinc-400 mt-2">Tus nuevos tokens están en tu billetera.</p>
      </div>

      <div>
        <h2 className="text-sm font-bold text-zinc-300 mb-3">Nuevos Saldos</h2>
        <div className="space-y-3">
          {tokens.map(token => (
            <div key={token.code} className="bg-zinc-900 p-4 rounded-lg flex justify-between items-center text-left">
               <div className="flex items-center space-x-3">
                 {TOKEN_ICONS[token.code]}
                 <span className="font-semibold text-white">{token.name}</span>
               </div>
               <p className="font-bold text-white text-lg">{token.balance}</p>
            </div>
          ))}
        </div>
      </div>

      <button
        onClick={onReset}
        className="w-full bg-red-600 text-white font-bold py-3 px-4 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-800 focus:ring-red-500 transition-all duration-300"
      >
        VOLVER A TU WALLET
      </button>
    </div>
  );
}

export default ConfirmationScreen;