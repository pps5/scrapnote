export function scroll_list() {
  let selected = document.querySelector("#list div.selected");
  if (selected) {
    selected.scrollIntoView({
      block: 'center',
    });
  }
}
