import styles from './SearchArea.module.css';
import { useAppContext } from "./AppContext";
import { tauri_invoke, tauri_dialog } from './tauri';

import { createSignal } from 'solid-js';

function SearchArea(prop) {
    const { configured, narrowDown, allItems, setAllItems, detail , error, loading} = useAppContext();

    let timeout = null;

    function onInput(e) {
        if (timeout) {
            clearTimeout(timeout);
        }
        const input = e.target.value;

        timeout = setTimeout((
        ) => { narrowDown(input); }, 100);
    }

    async function onKeyDown(e) {
        if (e.key === 'Enter') {
            e.preventDefault();
            loading.setIsLoading(true);
            const query = e.target.value;
            console.log(query);
            setTimeout(async () => {
                try {
                    const word = await tauri_invoke('search', { query: query });
                    const parsed = JSON.parse(word);
                    console.log(parsed);

                    detail.setDetail(parsed);
                    setAllItems([parsed.query]);
                    loading.dismissLoading(false);
                }
                catch (err) {
                    console.log(err);
                    loading.dismissLoading(false);
                    error.setError(err);
                }
            }, 100);

        }
    }

    return (
        <div class={styles.SearchArea}>
            <input type="text" onKeyDown={onKeyDown} onInput={onInput} />
        </div>
    );
}

export default SearchArea;