import logo from './logo.svg';
import styles from './VocabularyArea.module.css';
import { createSignal, Show, Switch, Match, For, onMount,createEffect, catchError } from "solid-js";
import { useAppContext } from './AppContext';
import { tauri_invoke } from './tauri';

function VocabularyArea() {
  let viewport;
  let dummyContainer;
  let visibleList;

  const {configured, narrowDown, allItems, setAllItems, detail} = useAppContext();

  const itemHeight = 32;
  const nodePadding = 10;

  const [overallHeight, setOverallHeight] = createSignal(0);
  const [visibleItems, setVisibleItems] = createSignal([]);
  const [offsetY, setOffsetY] = createSignal(0);

  function render(st) {
    const viewportHeight = viewport.getBoundingClientRect().height;
    let startNode = Math.floor(st / itemHeight) - nodePadding;
    startNode = Math.max(0, startNode);

    let visibleNodesCount = Math.ceil(viewportHeight / itemHeight) + 2 * nodePadding;
    visibleNodesCount = Math.min(allItems().length - startNode, visibleNodesCount);

    requestAnimationFrame(() => {
      setOffsetY(startNode * itemHeight);
      setVisibleItems(allItems().slice(startNode, startNode + visibleNodesCount));
    });
  }

  function onScroll(e) {
    render(e.target.scrollTop);
  }

  createEffect(() => {
    setOverallHeight(allItems().length * itemHeight);
    render(0);
    viewport.scrollTop = 0;
  });


  onMount(async () => {
    try {
      const words = await tauri_invoke('scan_vocabulary');
      console.log("scan vocabulary", words);
      setAllItems(words);
    }
    catch(err) {
      console.log(err);
    }
  });

  async function onLoadWord(query)  {
    console.log(query);
    try {
      const word = await tauri_invoke('load_word', {query: query});
      const parsed = JSON.parse(word);
      console.log(parsed);
      
      detail.setDetail(parsed);
    }
    catch(err) {
      console.log(err);
    }
  };

  return (
    <div class={styles.VocabularyArea}>
      <div ref={viewport} class={styles.Viewport} onScroll={onScroll}>
        <div ref={dummyContainer} class={styles.Dummy} style={{
          "height": `${overallHeight()}px`
        }}>
          <div ref={visibleList} style={{
            willChange: "transform",
            transform: `translateY(${offsetY()}px)`
          }}>
            <For each={visibleItems()}>{(item, i) =>
              <div class={styles.Item}>
                <a href="#" onClick={(e) => {onLoadWord(item);}}>{item}</a>
              </div>
            }</For>
          </div>
        </div>
      </div>
    </div>
  );
}

export default VocabularyArea;
