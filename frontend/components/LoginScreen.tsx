import React, { useState } from 'react';
import PencilIcon from './icons/PencilIcon';
import Spinner from './Spinner';

interface LoginScreenProps {
  onLogin: (email: string) => void;
  isLoading: boolean;
}

function LoginScreen({ onLogin, isLoading }: LoginScreenProps): React.ReactNode {
  const [email, setEmail] = useState<string>('');

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (email && !isLoading) {
      onLogin(email);
    }
  };

  return (
    <div className="bg-zinc-800 p-8 rounded-2xl shadow-2xl shadow-red-500/20 border border-zinc-700 transform transition-all duration-500">
      <div className="text-center mb-8">
        <div className="inline-block bg-red-600 p-4 rounded-full mb-4">
          <PencilIcon className="w-12 h-12 text-yellow-300" />
        </div>
        <h1 className="font-display text-4xl text-white">GLOBAL PIZZA PARTY 2026</h1>
        <p className="text-zinc-400 mt-2">Ingresa para canjear tus tokens</p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-6">
        <div>
          <label htmlFor="email" className="text-sm font-bold text-zinc-300 block mb-2">
            Tu Correo Electrónico
          </label>
          <input
            id="email"
            type="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            placeholder="tu@email.com"
            className="w-full px-4 py-3 bg-zinc-900 border border-zinc-700 rounded-lg text-white placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-red-500 transition-all"
            required
            disabled={isLoading}
          />
        </div>
        <button
          type="submit"
          disabled={isLoading}
          className="w-full flex justify-center items-center bg-red-600 text-white font-bold py-3 px-4 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-800 focus:ring-red-500 transition-all duration-300 disabled:bg-red-800 disabled:cursor-not-allowed"
        >
          {isLoading ? (
            <>
              <Spinner />
              <span className="ml-2">Enviando...</span>
            </>
          ) : (
            'INGRESAR'
          )}
        </button>
      </form>
    </div>
  );
}

export default LoginScreen;