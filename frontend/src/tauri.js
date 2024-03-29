function tauri_dialog() {
    if (window.__TAURI__) {
        return window.__TAURI__.dialog;
    }
    else {
        return {
            open: async (config) => {
                return '/Users/xxx/work/notes';
            }
        }
    }
}

async function invoke(cmd, payload) {
    switch (cmd) {
        case 'load_config':
            return '';
        case 'first_time_setup':
            return '';
        case 'scan_vocabulary':
            return '';
        case 'load_word':
            return '';
        case 'query_words':
            return '';
        case 'search':
            return '';
        case 'delete_word':
            return '';
        case 'fetch_all_words':
            return '';
        case 'generate_more_examples':
            return '';
        case 'load_usage':
            return {prompt: 0, completion: 0}
        default:
            console.error("unimplemented ", cmd);
            break;
    }
}

async function tauri_invoke(cmd, payload) {
    if (window.__TAURI__) {
        return window.__TAURI__.invoke(cmd, payload);
    }
    else {
        return invoke(cmd, payload);
    }
}


export { tauri_invoke, tauri_dialog };