import logo from './logo.svg';
import styles from './ConfigView.module.css';
import { render } from 'solid-js/web';
import { useAppContext } from './AppContext';
import { tauri_invoke, tauri_dialog } from './tauri';
import { createEffect, createSignal, onMount } from 'solid-js';

function ConfigView(prop) {

    const { configured, narrowDown, allItems, setAllItems, detail, showConfig  } = useAppContext();

    const [vocabularyFolder, setVocabularyFolder] = createSignal('');

    const [error, setError] = createSignal(null);

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
    let targetLang;
    let awsKey;
    let awsSecret;

    createEffect(() => {
        if (configured.configured() !== null) {
            setVocabularyFolder(configured.configured().workspace_path);

            if (openaiInput) {
                openaiInput.value = configured.configured().openai_token;
            }

            if (targetLang) {
                targetLang.value = configured.configured().target_lang;
            }

            if (awsKey && configured.configured().polly_config) {
                awsKey.value = configured.configured().polly_config.aws_key;
            }

            if (awsSecret && configured.configured().polly_config) {
                awsSecret.value = configured.configured().polly_config.aws_secret;
            }
        }
    })

    function onCancel(e) {
        showConfig.setShowConfig(false);
    }

    async function onApply(e) {
        e.preventDefault();

        let keyRegex = /^sk-[A-Za-z0-9]{48}$/g

        let folder = vocabularyFolder();
        let openaiToken = openaiInput.value;

        let match = openaiToken.match(keyRegex);

        if (!match) {
            setError('Invalid OpenAI API token.');
        }
        else if (!folder || folder.length == 0) {
            setError('Please choose a folder for your vocabulary.');
        }
        else {
            try {
                const workspaceData = await tauri_invoke('first_time_setup', { workspacePath: vocabularyFolder(), openaiToken: openaiInput.value, targetLang:targetLang.value, awsKey:awsKey.value.length>0?awsKey.value:null, awsSecret:awsSecret.value.length>0?awsSecret.value:null   });
                configured.setConfigured(workspaceData);
                showConfig.setShowConfig(false);
                setError(null);
            } catch (e) {
                console.log(e);
                setError('Setup error: ' + e);
            }
        }
    }

    return (
        <div class={styles.ConfigView}>
            <Show when={error()}>
                <p class={styles.Error}>Error: {error()}</p>
            </Show>
            <h3>Setup</h3>
            <p>{vocabularyFolder()}</p>
            <button class={styles.Button} onclick={onFolderSelected}>Pick a vocabulary directory</button>
            <label for="apikey">OpenAI API Key:</label>
            <input ref={openaiInput} id="apikey" type="text" />
            <label for="target">Target Language:</label>
            <select class={styles.Select} name="target" id="targetLang" ref={targetLang} >
                <option value="Chinese">Chinese</option>
                <option value="Spanish">Spanish</option>
                <option value="Japanese">Japanese</option>
                <option value="Korean">Korean</option>
                <option value="German">German</option>
                <option value="French">French</option>
                <option value="Portuguese">Portuguese</option>
            </select>
            <p>Voice Engine (AWS Polly) (Optional):</p>
            <label for="awskey">AWS Key:</label>
            <input ref={awsKey} id="awskey" type="text" />
            <label for="awssecret">AWS Secret:</label>
            <input ref={awsSecret} id="awssecret" type="text" />
            <div style="display:flex;justify-content:end;"><button class={styles.Button} onClick={onApply}>Apply</button>
            <Show when={configured.configured() !== null}>
            <button class={styles.Button} onClick={onCancel}>Cancel</button>
            </Show>
            </div>
        </div>
    );
}

export default ConfigView;