import logo from './logo.svg';
import styles from './SearchArea.module.css';
import { useAppContext } from "./AppContext";


function SearchArea(prop) {

    const {configured, narrowDown} = useAppContext();

    function onInput(e) {
        const input = e.target.value;
        narrowDown(input);
    }

    return (
        <div class={styles.SearchArea}>
            <input type="text" onInput={onInput} />
        </div>
    );
}

export default SearchArea;