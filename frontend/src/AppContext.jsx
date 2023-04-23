import { createSignal, createContext, useContext, onMount } from "solid-js";

const AppContext = createContext();

export function AppContextProvider(props) {  
  const [configured, setConfigured] = createSignal(false);
  const [allItems, setAllItems] = createSignal([]);

  function narrowDown(query) {
    let count = 100;
    if (query) {
      count = Math.floor( 100 / query.length);
    }
    if (count <= 0) {
      count = 1;
    }

    let debugWords = [];

    for(let i =0;i<count;i++) {
      debugWords.push((query?query:'word') + ' ' + i);
    }

    setAllItems(debugWords);
  }

  onMount(async () => {
    narrowDown();
  });

  return (
    <AppContext.Provider value={{configured:{configured, setConfigured}, narrowDown, allItems}}>
      {props.children}
    </AppContext.Provider>
  );
}

export function useAppContext() { return useContext(AppContext); }