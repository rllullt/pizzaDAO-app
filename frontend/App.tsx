import React, { useState, useCallback } from 'react';
import { Screen, User, Token } from './types';
import LoginScreen from './components/LoginScreen';
import ExchangeScreen from './components/ExchangeScreen';
import ConfirmationScreen from './components/ConfirmationScreen';

const INITIAL_TOKENS: Token[] = [
  { name: 'Token de Slice', code: 'SLICE', balance: 0 },
  { name: 'Token de Bebida', code: 'BEBIDA', balance: 0 },
];

function App(): React.ReactNode {
  const [currentScreen, setCurrentScreen] = useState<Screen>(Screen.Login);
  const [user, setUser] = useState<User | null>(null);
  const [tokens, setTokens] = useState<Token[]>(INITIAL_TOKENS);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [hasExchanged, setHasExchanged] = useState<boolean>(false);

  const handleLogin = useCallback((email: string) => {
    setIsLoading(true);
    // Simulate sending a magic link and waiting for authentication
    setTimeout(() => {
      setUser({ email, name: 'Vegeta', benyPoints: 0 });
      setCurrentScreen(Screen.Exchange);
      setIsLoading(false);
    }, 1500);
  }, []);

  const handleExchange = useCallback(() => {
    setIsLoading(true);
    // Simulate the Stellar transaction
    setTimeout(() => {
      setTokens(prevTokens => 
        prevTokens.map(token => ({ ...token, balance: token.balance + 1 }))
      );
      setUser(prevUser => {
        if (!prevUser) return null;
        // Award 20 points for each of the 3 steps
        return { ...prevUser, benyPoints: prevUser.benyPoints + 60 };
      });
      setHasExchanged(true);
      setCurrentScreen(Screen.Confirmation);
      setIsLoading(false);
    }, 2000);
  }, []);

  const handleReset = useCallback(() => {
    setCurrentScreen(Screen.Exchange);
  }, []);

  const handleLogout = useCallback(() => {
    setUser(null);
    setTokens(INITIAL_TOKENS);
    setHasExchanged(false);
    setCurrentScreen(Screen.Login);
  }, []);

  const renderScreen = (): React.ReactNode => {
    switch (currentScreen) {
      case Screen.Login:
        return <LoginScreen onLogin={handleLogin} isLoading={isLoading} />;
      case Screen.Exchange:
        if (user) {
            return <ExchangeScreen user={user} tokens={tokens} onExchange={handleExchange} isLoading={isLoading} onLogout={handleLogout} hasExchanged={hasExchanged} />;
        }
        // Fallback to login if user is somehow null
        setCurrentScreen(Screen.Login);
        return null;
      case Screen.Confirmation:
        return <ConfirmationScreen tokens={tokens} onReset={handleReset} />;
      default:
        return <LoginScreen onLogin={handleLogin} isLoading={isLoading} />;
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center font-sans p-4" style={{
      backgroundImage: `url('/assets/pizza1.jpg')`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
    }}>
      <div className="w-full max-w-sm mx-auto">
        {renderScreen()}
      </div>
    </div>
  );
}

export default App;