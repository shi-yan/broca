import logo from './logo.svg';
import styles from './App.module.css';
import VocabularyArea from './VocabularyArea.jsx';
import DetailArea from './DetailArea.jsx';
import ConfigView from './ConfigView';
import SearchArea from './SearchArea';
import { createSignal, Show, Switch, Match, onMount } from "solid-js";
import { useAppContext } from './AppContext';
import { tauri_invoke , tauri_dialog } from './tauri';

function App() {

  const {configured, narrowDown} = useAppContext();

  onMount(async () => {
    try {
      console.log("calling load_config");
      const workspaceData = await tauri_invoke('load_config');
      console.log("load_config called", workspaceData);
      configured.setConfigured(true);
    }
    catch (err) {
      console.log(err);
      configured.setConfigured(false);
    }
  });

  return (

      <Switch >
        <Match when={configured.configured()}>
          <div class={styles.App}>
            <div class={styles.TopArea}>
              <DetailArea class={styles.DetailArea}></DetailArea>
              <VocabularyArea class={styles.VocabularyArea}></VocabularyArea>
            </div>
            <SearchArea></SearchArea>
          </div>
        </Match>
        <Match when={!configured.configured()}>
          <ConfigView></ConfigView>
        </Match>
      </Switch>

  );
}

export default App;
