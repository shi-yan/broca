import logo from './logo.svg';
import styles from './ConfigView.module.css';
import { render } from 'solid-js/web';
import { useAppContext } from './AppContext';

function ConfigView(prop) {

    const {configured} = useAppContext();

    function onFolderSelected(e) {
        console.log('onFolderSelected');
        e.preventDefault(); // prevent the default upload behavior
 
    }

    let myDiv;

    function onApply(e) {
        e.preventDefault();
        configured.setConfigured(true);

        //render(() => <p >test</p>, myDiv);
    }

    

    return (
        <div class={styles.ConfigView}>
            <button onclick={onFolderSelected}>Pick a vocabulary directory</button>
            <label for="apikey">OpenAI API Key</label>
            <input id="apikey" type="text" />
            <button onClick={onApply}>Apply</button>
            <div ref={myDiv}>My Element</div>

        </div>
    );
}

export default ConfigView;