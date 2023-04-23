import { createSignal, createContext, useContext, onMount } from "solid-js";
import { tauri_invoke, tauri_dialog } from './tauri';

const AppContext = createContext();

export function AppContextProvider(props) {
  const [configured, setConfigured] = createSignal(false);
  const [allItems, setAllItems] = createSignal([]);
  const [detail, setDetail] = createSignal({});

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
    <AppContext.Provider value={{ configured: { configured, setConfigured }, narrowDown, allItems, setAllItems, detail: { detail, setDetail } }}>
      {props.children}
    </AppContext.Provider>
  );
}

export function useAppContext() { return useContext(AppContext); }