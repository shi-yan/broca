import logo from './logo.svg';
import styles from './VocabularyArea.module.css';
import { createSignal, Show, Switch, Match, For, onMount,createEffect } from "solid-js";
import { useAppContext } from './AppContext';

function VocabularyArea() {
  let viewport;
  let dummyContainer;
  let visibleList;

  const {configured, narrowDown, allItems} = useAppContext();

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

  });

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
                {i() + 1}: {item}
              </div>
            }</For>
          </div>
        </div>
      </div>
    </div>
  );
}

export default VocabularyArea;
