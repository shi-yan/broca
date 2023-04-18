import { createSignal, For } from 'solid-js';



function WordList() {

    const [cats, setCats] = createSignal([
        { id: 'J---aiyznGQ', name: 'Keyboard Cat' },
        { id: 'z_AbfPXTKms', name: 'Maru' },
        { id: 'OUtn3pvWmpg', name: 'Henri The Existential Cat' }
    ]);

    return (
        <div class="wordlist-viewport">
            <div class="wordlist-dummy-container">
                <div class="wordlist-actual-list">
                    <For each={cats()}>{(cat, i) =>
                        <div>
                            <a target="_blank" href={`https://www.youtube.com/watch?v=${cat.id}`}>
                                {i() + 1}: {cat.name}
                            </a>
                        </div>
                    }</For>
                </div>
            </div>
        </div>
    );
}

export default WordList;
