import logo from './logo.svg';
import styles from './DetailArea.module.css';
import { useAppContext } from './AppContext';

function DetailArea() {

  const { configured, narrowDown, allItems, setAllItems, detail } = useAppContext();


  return (
    <div class={styles.DetailArea}>
      <Show when={detail.detail().query}>
        <p class={styles.Word}>{detail.detail().query}</p>
        <ul>
          <For each={detail.detail().meanings}>{(meaning, i) =>
            <li>
              <p class={styles.POS}>{meaning.pos}</p>
              <ul>
                <For each={meaning.meanings}>{(m, i) =>
                  <li  class={styles.Meaning}>
                    <ul>
                      <For each={m.meaning}>{(mm, i) =>
                        <li> {mm[Object.keys(mm)[0]]}</li>
                      }</For>
                    </ul>
                    <p>Examples</p>
                    <For each={m.examples}>{(example, i) =>
                      <ul>
                        <For each={example}>{(ee, i) =>
                          <li> {ee[Object.keys(ee)[0]]}</li>
                        }</For>
                      </ul>
                    }</For>
                  </li>
                }</For>
              </ul>
            </li>
          }</For>
        </ul>
      </Show>
    </div>
  );
}

export default DetailArea;