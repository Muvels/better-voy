'use client';                                 // ⬅️ 1. keep this on the client

import Head from 'next/head';
import { useCallback, useEffect, useRef, useState } from 'react';

import styles from '@/styles/Home.module.css';
import { NavigationBar } from '@/components/NavigationBar';
import { useVoy } from '@/context/VoyContext';

import { TextModel, type TextModelRunner } from '@visheratin/web-ai';

const phrases = [
  'That is a very happy Person',
  'That is a Happy Dog',
  'Today is a sunny day',
  'Yesterday is a sunny day',
];

export default function ClientSideVoyDemo() {
  /* ------------------------------------------------------------------ */
  /*  HOOKS / STATE                                                     */
  /* ------------------------------------------------------------------ */
  const voy = useVoy();                       // ready – provider waits for it
  const [model, setModel] = useState<TextModelRunner | null>(null);
  const [results, setResults] = useState<any[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);

  /* ------------------------------------------------------------------ */
  /*  1)  LOAD THE TEXT-EMBEDDING MODEL (ONCE)                          */
  /* ------------------------------------------------------------------ */
  useEffect(() => {
    if (model) return;                        // already loaded

    (async () => {
      const modelName = 'gtr-t5-quant';
      console.log('Loading model:', modelName);

      const m = await TextModel.create(modelName);
      setModel(m.model);                      // TextModel.create returns { model }
    })();
  }, [model]);

  /* ------------------------------------------------------------------ */
  /*  2)  BUILD THE VOY INDEX AS SOON AS BOTH MODEL + VOY ARE READY     */
  /* ------------------------------------------------------------------ */
  useEffect(() => {
    if (!model || !voy) return;               // wait until both are ready

    (async () => {
      console.log('Creating embeddings for starter phrases …');

      const docs = await Promise.all(
        phrases.map(async (text, i) => {
          const { result } = await model.process(text);      // Float32Array
          return {
            id: String(i),
            title: text,
            url: `/path/${i}`,
            embeddings: Array.from(result),  // Voy expects plain JS array
          };
        }),
      );

      voy.index({ embeddings: docs });       // resource shape expected by Voy
      console.log('Voy index built ✅');
    })();
  }, [model, voy]);

  /* ------------------------------------------------------------------ */
  /*  3)  SEARCH HANDLER                                                */
  /* ------------------------------------------------------------------ */
  const onSubmit = useCallback(async () => {
    if (!model || !voy) return;

    const query = inputRef.current?.value?.trim();
    if (!query) return;

    const { result } = await model.process(query);
    const { neighbors } = voy.search(result, 4);   // top-4

    setResults(neighbors);
  }, [model, voy]);

  /* ------------------------------------------------------------------ */
  /*  UI                                                                */
  /* ------------------------------------------------------------------ */
  const isReady = model && voy;

  return (
    <>
      <Head>
        <title>Voy × Next.js – Client Demo</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>

      <main>
        <NavigationBar />

        <section>
          <code className={styles.example}>
            {!isReady ? (
              <>Loading {model ? 'Voy' : 'model'} …</>
            ) : (
              <>
                {results.length > 0 && (
                  <span>
                    {results.map((r, i) => (
                      <p
                        key={r.id}
                        className={i === 0 ? styles.primary : styles.secondary}
                      >
                        {r.title}
                      </p>
                    ))}
                  </span>
                )}

                <input ref={inputRef} type="text" placeholder="Search…" />{' '}
                <button onClick={onSubmit}>Submit</button>

                <p>Try searching for:</p>
                <ul>
                  <li>
                    That is a happy <b>p</b>erson
                  </li>
                  <li>
                    That is a happy <b>P</b>erson
                  </li>
                  <li>sunny</li>
                  <li>sunny day</li>
                </ul>
              </>
            )}
          </code>
        </section>
      </main>
    </>
  );
}
