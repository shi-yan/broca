import logo from './logo.svg';
import styles from './SearchArea.module.css';
import { useAppContext } from "./AppContext";
import { tauri_invoke, tauri_dialog } from './tauri';
import loading from './loading.gif'
import { createSignal } from 'solid-js';

function SearchArea(prop) {
    const { configured, narrowDown, allItems, setAllItems, detail } = useAppContext();
    const [isLoading, setIsLoading] = createSignal(false);

    const [error, setError] = createSignal(null);

    let timeout = null;

    function onInput(e) {
        if (timeout) {
            clearTimeout(timeout);
        }
        const input = e.target.value;

        timeout = setTimeout((
        ) => { narrowDown(input); }, 100);
    }

    function onDismissError(e) {
        setError(null);
    }

    async function onKeyDown(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            setIsLoading(true);
            const query = e.target.value;
            console.log(query);
            setTimeout(async () => {
                try {
                    const word = await tauri_invoke('search', { query: query });
                    const parsed = JSON.parse(word);
                    console.log(parsed);

                    detail.setDetail(parsed);
                    setAllItems([parsed.query]);
                    setIsLoading(false);
                }
                catch (err) {
                    console.log(err);
                    setIsLoading(false);
                    setError(err);
                }
            }, 100);

        }
    }

    return (
        <div class={styles.SearchArea}>
            <Show when={isLoading()}>
                <div class={styles.Loading} >
                    <p>ChatGPT is Working ...</p>
                    <img src={loading} style="width:320px;" />
                </div>
            </Show>
            <Show when={error()}>
                <div class={styles.Error} >
                    <p>Error: {error()}</p>
                    <button onClick={onDismissError}>OK</button>
                </div>
            </Show>
            <input type="text" onKeyDown={onKeyDown} onInput={onInput} />
        </div>
    );
}

export default SearchArea;