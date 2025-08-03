import React, { useState } from 'react';
import { User, Token } from '../types';
import Spinner from './Spinner';
import PizzaSliceIcon from './icons/PizzaSliceIcon';
import BeverageIcon from './icons/BeverageIcon';
import UserIcon from './icons/UserIcon';
import LogoutIcon from './icons/LogoutIcon';
import QRScannerModal from './QRScannerModal';
import AdCard from './AdCard';
import SoroswapIcon from './icons/SoroswapIcon';
import QuestionMarkIcon from './icons/QuestionMarkIcon';
import SparklesIcon from './icons/SparklesIcon';
import CheckIcon from './icons/CheckIcon';

interface ExchangeScreenProps {
  user: User;
  tokens: Token[];
  onExchange: () => void;
  isLoading: boolean;
  onLogout: () => void;
  hasExchanged: boolean;
}

const TOKEN_ICONS: { [key: string]: React.ReactNode } = {
    SLICE: <PizzaSliceIcon className="w-8 h-8 text-yellow-400 drop-shadow-[0_0_8px_rgba(255,255,0,0.7)]" />, // brillo
    BEBIDA: <BeverageIcon className="w-8 h-8 text-cyan-400 drop-shadow-[0_0_8px_rgba(0,255,255,0.7)]" />, // brillo
};

const TokenRow: React.FC<{ token: Token; onClick: () => void; isClickable: boolean; }> = ({ token, onClick, isClickable }) => (
    <button 
        onClick={onClick}
        disabled={!isClickable}
        className={`w-full bg-white/10 backdrop-blur-md p-4 rounded-2xl flex items-center justify-between space-x-4 text-left transition-all duration-200 shadow-xl border-2 border-white/20 hover:border-white/40 ${isClickable ? 'cursor-pointer hover:bg-white/20 focus:outline-none focus:ring-2 focus:ring-white/40' : 'cursor-default'} group`}
        aria-label={isClickable ? `Canjear ${token.name}` : token.name}
        style={{ boxShadow: '0 4px 24px 0 rgba(255,255,255,0.15), 0 1.5px 8px 0 rgba(0,0,0,0.25)' }}
    >
        <div className="flex items-center space-x-4">
            <div className="bg-black/80 p-2 rounded-full group-hover:scale-110 transition-transform duration-200 shadow-lg shadow-white/20">
                {TOKEN_ICONS[token.code]}
            </div>
            <div>
                <p className="font-extrabold text-white text-lg tracking-wider drop-shadow-[0_0_6px_rgba(255,255,255,0.7)]">
                  {token.code === 'SLICE' ? '$SLICE' : token.code === 'BEBIDA' ? '$DRINK' : token.name}
                </p>
            </div>
        </div>
        <div className="flex items-baseline">
            <p className="font-bold text-white text-2xl drop-shadow-[0_0_6px_rgba(255,255,255,0.7)]">{token.balance}</p>
            {isClickable && <span className="ml-2 text-xs font-extrabold text-black bg-white/80 px-2 py-1 rounded shadow-md shadow-black/30 group-hover:bg-black group-hover:text-white transition-colors">CANJEAR</span>}
        </div>
  </button>
);

const Task: React.FC<{ 
  number: number;
  text: string; 
  isCompleted: boolean; 
  onToggle: () => void; 
}> = ({ number, text, isCompleted, onToggle }) => (
    <button onClick={onToggle} className="w-full flex items-start justify-between text-left p-2 -ml-2 rounded-lg hover:bg-zinc-800 transition-colors group">
        <div className="flex items-start space-x-4">
            <div className={`flex-shrink-0 w-8 h-8 rounded-full flex items-center justify-center font-bold text-sm transition-all duration-200 ${
                isCompleted 
                ? 'bg-green-500 text-white' 
                : 'bg-zinc-700 text-zinc-400 group-hover:bg-zinc-600'
            }`}>
                {isCompleted ? <CheckIcon className="w-5 h-5" /> : number}
            </div>
            <p className={`text-sm sm:text-base pt-1 pr-2 transition-colors ${isCompleted ? 'text-zinc-500 line-through' : 'text-zinc-300'}`}>{text}</p>
        </div>
        <p className="flex-shrink-0 text-yellow-400/80 font-bold text-sm pt-1">+20 Beny Pts</p>
    </button>
);

function ExchangeScreen({ user, tokens, onExchange, isLoading, onLogout, hasExchanged }: ExchangeScreenProps): React.ReactNode {
  const [scannerToken, setScannerToken] = useState<Token | null>(null);
  const [tasks, setTasks] = useState([
    { id: 1, text: "Verifica tu ticket reclamado", completed: false },
    { id: 2, text: "Comparte en Twitter que llegas a esta fiesta", completed: false },
    { id: 3, text: "Haz CLICK y canjea tus tokens", completed: false },
  ]);

  const handleToggleTask = (id: number) => {
      setTasks(currentTasks =>
          currentTasks.map(task =>
              task.id === id ? { ...task, completed: !task.completed } : task
          )
      );
  };

  const allTasksCompleted = tasks.every(task => task.completed);

  const handleTokenClick = (token: Token) => {
    if (hasExchanged && token.balance > 0) {
      setScannerToken(token);
    }
  };

  const ExchangeBox = (
    <div className="bg-zinc-900/70 p-6 rounded-xl border border-dashed border-zinc-600 space-y-5">
      <h3 className="font-bold text-white text-lg">Bienvenido, canjea tus tokens:</h3>
      <div className="space-y-2">
        {tasks.map(task => (
            <Task 
                key={task.id}
                number={task.id}
                text={task.text}
                isCompleted={task.completed}
                onToggle={() => handleToggleTask(task.id)}
            />
        ))}
      </div>
      <button
        onClick={onExchange}
        disabled={isLoading || !allTasksCompleted}
        className="w-full flex justify-center items-center bg-yellow-500 text-zinc-900 font-bold py-3 px-4 rounded-lg hover:bg-yellow-400 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-900 focus:ring-yellow-500 transition-all duration-300 disabled:bg-yellow-500/30 disabled:text-zinc-500 disabled:cursor-not-allowed text-md mt-4"
      >
        {isLoading ? (
          <>
            <Spinner />
            <span className="ml-2">Procesando...</span>
          </>
        ) : (
          'CANJEAR TOKENS'
        )}
      </button>
    </div>
  );
  
  const TokenList = (
     <div>
        <h2 className="text-sm font-bold text-zinc-300 mb-3">Tus Tokens</h2>
        <div className="space-y-3">
          {tokens.map(token => 
            <TokenRow 
                key={token.code} 
                token={token} 
                onClick={() => handleTokenClick(token)}
                isClickable={hasExchanged && token.balance > 0}
            />
          )}
        </div>
      </div>
  );

  const AdSection = (
    <div>
      <h2 className="text-sm font-bold text-zinc-300 mb-3">Descubre Más</h2>
      <div className="space-y-3">
        <AdCard
          icon={<SoroswapIcon className="w-6 h-6 text-cyan-400" />}
          title="Conoce Soroswap"
        />
        <AdCard
          icon={<QuestionMarkIcon className="w-6 h-6 text-zinc-300" />}
          title="¿Qué es XLM?"
        />
        <AdCard
          icon={<SparklesIcon className="w-6 h-6 text-yellow-400" />}
          title="¿Quieres participar en PizzaDAOApp?"
        />
      </div>
    </div>
  );

  return (
    <>
    <div className="bg-zinc-800 p-6 sm:p-8 rounded-2xl shadow-2xl shadow-yellow-500/15 border border-zinc-700 space-y-8">
      
      <div className="flex items-center space-x-4">
        <div className="bg-zinc-700 p-3 rounded-full">
            <UserIcon className="w-10 h-10 text-zinc-300" />
        </div>
        <div>
            <h1 className="font-display text-3xl text-white">{user.name}</h1>
            <p className="text-yellow-400 font-bold">{user.benyPoints} Beny Pts</p>
        </div>
      </div>

      {!hasExchanged && ExchangeBox}

      <div className={hasExchanged ? "-mt-4" : ""}>
        {TokenList}
      </div>

      {hasExchanged && AdSection}
      
      <button
        onClick={onLogout}
        className="w-full flex justify-center items-center gap-2 bg-red-600/80 text-white font-bold py-3 px-4 rounded-lg hover:bg-red-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-zinc-800 focus:ring-red-500 transition-all duration-300"
      >
        <LogoutIcon className="w-5 h-5" />
        Cerrar Sesión
      </button>

    </div>
    {scannerToken && <QRScannerModal token={scannerToken} onClose={() => setScannerToken(null)} />}
    </>
  );
}

export default ExchangeScreen;