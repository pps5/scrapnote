import init, { run_app } from './pkg/scrapnote.js';

function setUpMonacoEditor() {
    require.config({ paths: { 'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.21.2/min/vs' }});
    window.MonacoEnvironment = { getWorkerUrl: () => proxy };
    let proxy = URL.createObjectURL(new Blob([`
        self.MonacoEnvironment = {
            baseUrl: 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.21.2/min'
        };
        importScripts('https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.21.2/min/vs/base/worker/workerMain.min.js');
    `], { type: 'text/javascript' }));
    require(["vs/editor/editor.main"], function () {
        window.editor = monaco.editor.create(document.getElementById('editor'), {
            automaticLayout: true,
            language: 'markdown',
            glyphMargin: false,
            lineNumbers: 'off',
            minimap: { enabled: false },
            readOnly: true,
            fontFamily: 'Noto Sans JP',
            folding: false,
            lineDecorationsWidth: 0,
            lineNumbersMinChars: 0,
            scrollbar: {
                vertical: "visible"
            },
        });
        window.editor.addCommand(monaco.KeyCode.Escape, function() {
            document.getElementById('editor')
                .dispatchEvent(new KeyboardEvent('keypress', {key: 'Escape'}));
        }, '!suggestWidgetVisible');
    });

}

async function main() {
    await init('/scrapnote_bg.wasm');
    run_app();
    setUpMonacoEditor();
}
main();
