export function get_value() {
    return window.editor.getValue();
}

export function set_value(value) {
    window.editor.setValue(value);
}

export function focus() {
    window.editor.focus();
}

export function set_editable(editable) {
    window.editor.updateOptions({
        readOnly: !editable,
    });
}
