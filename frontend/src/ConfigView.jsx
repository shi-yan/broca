import logo from './logo.svg';
import styles from './ConfigView.module.css';
import { render } from 'solid-js/web';
import { useAppContext } from './AppContext';
import { tauri_invoke, tauri_dialog } from './tauri';
import { createSignal } from 'solid-js';

function ConfigView(prop) {

    const { configured } = useAppContext();

    const [vocabularyFolder, setVocabularyFolder] = createSignal('');

    async function onFolderSelected(e) {
        console.log('onFolderSelected');
        e.preventDefault(); // prevent the default upload behavior
        const selected = await tauri_dialog().open({
            multiple: false,
            directory: true,
            title: 'Choose a directory for notes'
        });

        console.log('folder selected', selected);

        if (selected) {
            setVocabularyFolder(selected);
        }
    }

    let openaiInput;

    async function onApply(e) {
        e.preventDefault();
        
        try {
            const workspaceData = await tauri_invoke('first_time_setup', { workspacePath: vocabularyFolder(), openaiToken: openaiInput.value });
            configured.setConfigured(true);
        } catch (e) {
            console.log(e);
        }
    }

    return (
        <div class={styles.ConfigView}>
            <p>{vocabularyFolder()}</p>
            <button onclick={onFolderSelected}>Pick a vocabulary directory</button>
            <label for="apikey">OpenAI API Key</label>
            <input ref={openaiInput} id="apikey" type="text" />
            <button onClick={onApply}>Apply</button>

        </div>
    );
}

export default ConfigView;