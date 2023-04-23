import logo from './logo.svg';
import styles from './App.module.css';
import VocabularyArea from './VocabularyArea.jsx';
import DetailArea from './DetailArea.jsx';
import ConfigView from './ConfigView';
import SearchArea from './SearchArea';
import { createSignal, Show, Switch, Match } from "solid-js";
import { useAppContext } from './AppContext';

function App() {

  const {configured, narrowDown} = useAppContext();

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
