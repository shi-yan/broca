import logo from './logo.svg';
import styles from './DetailArea.module.css';
import { useAppContext } from './AppContext';
import { tauri_invoke, tauri_dialog } from './tauri';

function DetailArea() {

  const { configured, narrowDown, allItems, setAllItems, detail } = useAppContext();

  async function onDelete(query) {
    try {
      const workspaceData = await tauri_invoke('delete_word', { query: query });
      detail.setDetail({});
      narrowDown();
    } catch (err) {
      console.log(err);
    }
  }

  return (
    <div class={styles.DetailArea}>
      <Show when={detail.detail().query}>
        <p class={styles.Word}>{detail.detail().query} <a href="#" onClick={(e) => {onDelete(detail.detail().query);}}><svg style="width:24px;height24px;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>trash-can-outline</title><path d="M9,3V4H4V6H5V19A2,2 0 0,0 7,21H17A2,2 0 0,0 19,19V6H20V4H15V3H9M7,6H17V19H7V6M9,8V17H11V8H9M13,8V17H15V8H13Z" /></svg></a></p>
        <ul>
          <For each={detail.detail().meanings}>{(meaning, i) =>
            <li>
              <p class={styles.POS}>{meaning.pos}</p>
              <ul>
                <For each={meaning.meanings}>{(m, i) =>
                  <li class={styles.Meaning}>
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