import { createSignal, createContext, useContext, onMount } from "solid-js";
import { tauri_invoke, tauri_dialog } from './tauri';

const AppContext = createContext();

export function AppContextProvider(props) {
  const [showConfig, setShowConfig] = createSignal(false);
  const [configured, setConfigured] = createSignal(null);
  const [allItems, setAllItems] = createSignal([]);
  const [detail, setDetail] = createSignal({});
  const [error, setError] = createSignal(null);


  async function narrowDown(query) {
    console.log(query);
    try {
      if (query) {
        const words = await tauri_invoke('query_words', { query: query });
        setAllItems(words);
      }
      else {
        const words = await tauri_invoke('fetch_all_words');
        setAllItems(words);
      }
    }
    catch (err) {
      console.log(err);
    }
  }


  onMount(async () => {

  });

  return (
    <AppContext.Provider value={{ configured: { configured, setConfigured }, narrowDown, allItems, setAllItems, detail: { detail, setDetail }, showConfig: {showConfig, setShowConfig}, error: {error, setError} }}>
      {props.children}
    </AppContext.Provider>
  );
}

export function useAppContext() { return useContext(AppContext); }