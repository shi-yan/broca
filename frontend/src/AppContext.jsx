import { createSignal, createContext, useContext, onMount } from "solid-js";
import { tauri_invoke, tauri_dialog } from './tauri';

const AppContext = createContext();

export function AppContextProvider(props) {
  const [showConfig, setShowConfig] = createSignal(false);
  const [configured, setConfigured] = createSignal(null);
  const [allItems, setAllItems] = createSignal([]);
  const [detail, setDetail] = createSignal({});
  const [error, setError] = createSignal(null);
  const [isLoading, setIsLoading] = createSignal(false);
  const [promptTokenUsage, setPromptTokenUsage] = createSignal(0);
  const [completionTokenUsage, setCompletionTokenUsage] = createSignal(0);

  async function refreshUsage() {
    try {
      const result = await tauri_invoke('load_usage');
      setPromptTokenUsage(result[0]);
      setCompletionTokenUsage(result[1]);
    }
    catch (err) {
      console.log(err);
    }
  }

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

  async function dismissLoading() {
    await refreshUsage();
    setIsLoading(false);
  }

  onMount(async () => {
   
  });

  return (
    <AppContext.Provider value={{
      configured: { configured, setConfigured },
      narrowDown, allItems, setAllItems,
      detail: { detail, setDetail },
      showConfig: { showConfig, setShowConfig },
      error: { error, setError },
      loading: { isLoading, setIsLoading, dismissLoading },
      usage: {promptTokenUsage, completionTokenUsage, refreshUsage}
    }}>
      {props.children}
    </AppContext.Provider>
  );
}

export function useAppContext() { return useContext(AppContext); }