'use client';                                   // ① run only in the browser

import {
  createContext,
  useContext,
  useEffect,
  useState,
  PropsWithChildren,
} from 'react';

import init, { Voy } from 'voy-search';

const VoyContext = createContext<Voy | null>(null);

export const VoyProvider = ({ children }: PropsWithChildren) => {
  const [voy, setVoy] = useState<Voy | null>(null);

  useEffect(() => {
    let cancelled = false;

    (async () => {
      // ② download & instantiate the WASM once the component is mounted
      await init();                               // can also pass a URL here
      if (cancelled) return;

      const opts = { onSearch: () => console.log('searched') };
      setVoy(new Voy(undefined, opts));
    })();

    return () => { cancelled = true; };
  }, []);

  // optional loading UI while the .wasm file is streaming in
  if (!voy) return null;

  return (
    <VoyContext.Provider value={voy}>
      {children}
    </VoyContext.Provider>
  );
};

export const useVoy = () => {
  const ctx = useContext(VoyContext);
  if (!ctx) throw new Error('useVoy must be used within a VoyProvider');
  return ctx;
};
