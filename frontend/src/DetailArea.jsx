import logo from './logo.svg';
import styles from './DetailArea.module.css';
import { useAppContext } from './AppContext';
import { tauri_invoke, tauri_dialog } from './tauri';

function DetailArea() {

  const { configured, narrowDown, allItems, setAllItems, detail , error} = useAppContext();

  async function onDelete(query) {
    try {
      const workspaceData = await tauri_invoke('delete_word', { query: query });
      detail.setDetail({});
      narrowDown();
    } catch (err) {
      console.log(err);
      error.setError(err);
    }
  }

  async function onPronouns(query) {
    try {
      const audioPath = await tauri_invoke('say', { query: query });
      console.log("audio path", audioPath)
      const audioUrl = await window.__TAURI__.tauri.convertFileSrc(audioPath);
      new Audio(audioUrl).play();
    } catch (err) {
      console.log(err);
      error.setError(err);
    }
  }

  async function onGenerateMore(query, meaning) {
    console.log("generate more", query, meaning);
  }

  return (
    <div class={styles.DetailArea}>
      <Show when={detail.detail().query}>
        <div class={styles.Item}>
          <span class={styles.Word}>{detail.detail().query}
            <Show when={configured.configured() !== null && configured.configured().polly_config}>
              <a href="#" onClick={(e) => { onPronouns(detail.detail().query); }}>
                <svg style="width:24px;height24px;margin-left:10px;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>volume-high</title><path d="M14,3.23V5.29C16.89,6.15 19,8.83 19,12C19,15.17 16.89,17.84 14,18.7V20.77C18,19.86 21,16.28 21,12C21,7.72 18,4.14 14,3.23M16.5,12C16.5,10.23 15.5,8.71 14,7.97V16C15.5,15.29 16.5,13.76 16.5,12M3,9V15H7L12,20V4L7,9H3Z" /></svg>
              </a>
            </Show>
          </span>
          <a href="#" onClick={(e) => { onDelete(detail.detail().query); }}>
            <svg style="width:24px;height24px;margin-right:10px;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>trash-can-outline</title><path d="M9,3V4H4V6H5V19A2,2 0 0,0 7,21H17A2,2 0 0,0 19,19V6H20V4H15V3H9M7,6H17V19H7V6M9,8V17H11V8H9M13,8V17H15V8H13Z" /></svg>
          </a>
        </div>
        <ul>
          <For each={detail.detail().meanings}>{(meaning, i) =>
            <li>
              <p class={styles.POS}>{meaning.pos}</p>
              <ol>
                <For each={meaning.meanings}>{(m, i) =>
                  <li class={styles.Meaning}>
                    <ul>
                      <For each={m.meaning}>{(mm, i) =>
                        <li> {mm[Object.keys(mm)[0]]}
                          <Show when={Object.keys(mm)[0] === 'English' && configured.configured() !== null && configured.configured().polly_config}>
                            <a href="#" onClick={(e) => { onPronouns(mm[Object.keys(mm)[0]]); }}>
                              <svg style="width:24px;height24px;margin-left:10px;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>volume-high</title><path d="M14,3.23V5.29C16.89,6.15 19,8.83 19,12C19,15.17 16.89,17.84 14,18.7V20.77C18,19.86 21,16.28 21,12C21,7.72 18,4.14 14,3.23M16.5,12C16.5,10.23 15.5,8.71 14,7.97V16C15.5,15.29 16.5,13.76 16.5,12M3,9V15H7L12,20V4L7,9H3Z" /></svg>
                            </a>
                          </Show>
                        </li>
                      }</For>
                    </ul>
                    <p>Examples</p>
                    <ol type="a">
                    <For each={m.examples}>{(example, i) =>
                      <li>
                      <ul>
                        <For each={example}>{(ee, i) =>
                          <li> {ee[Object.keys(ee)[0]]}
                            <Show when={Object.keys(ee)[0] === 'English' && configured.configured() !== null && configured.configured().polly_config}>
                              <a href="#" onClick={(e) => { onPronouns(ee[Object.keys(ee)[0]]); }}>
                                <svg style="width:24px;height24px;margin-left:10px;" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><title>volume-high</title><path d="M14,3.23V5.29C16.89,6.15 19,8.83 19,12C19,15.17 16.89,17.84 14,18.7V20.77C18,19.86 21,16.28 21,12C21,7.72 18,4.14 14,3.23M16.5,12C16.5,10.23 15.5,8.71 14,7.97V16C15.5,15.29 16.5,13.76 16.5,12M3,9V15H7L12,20V4L7,9H3Z" /></svg>
                              </a>
                            </Show>
                          </li>
                        }</For>
                      </ul></li>
                    }</For>
                    </ol>
                    <button class={styles.GenerateMore} onClick={(e) => { onGenerateMore(detail.detail().query, m.meaning) }}>Generate More ...</button>
                  </li>
                }</For>
              </ol>
            </li>
          }</For>
        </ul>
      </Show>
    </div>
  );
}

export default DetailArea;