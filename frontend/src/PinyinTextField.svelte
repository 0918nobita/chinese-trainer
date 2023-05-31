<script lang="ts">
  let input: HTMLInputElement;

  let visible = false;

  function getCurrentChar(): [number, string] {
    const selectionStart = input.selectionStart ?? input.value.length;

    return [
      selectionStart - 1,
      input.value.substring(selectionStart - 1, selectionStart),
    ];
  }

  function focusin() {
    visible = true;
  }

  function close() {
    visible = false;
  }

  function tone1() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      input.value =
        input.value.substring(0, i) + 'ā' + input.value.substring(i + 1);
    }
  }

  function tone2() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      input.value =
        input.value.substring(0, i) + 'á' + input.value.substring(i + 1);
    }
  }

  function tone3() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      input.value =
        input.value.substring(0, i) + 'ǎ' + input.value.substring(i + 1);
    }
  }

  function tone4() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      input.value =
        input.value.substring(0, i) + 'à' + input.value.substring(i + 1);
    }
  }

  function handleInputEvent(e: InputEvent) {
    const [i, currentChar] = getCurrentChar();

    if (e.data === '1' && currentChar === 'a') {
      e.preventDefault();
      input.value =
        input.value.substring(0, i) + 'ā' + input.value.substring(i + 1);
      input.setSelectionRange(i + 1, i + 1);
      return;
    }

    if (e.data === '2' && currentChar === 'a') {
      e.preventDefault();
      input.value =
        input.value.substring(0, i) + 'á' + input.value.substring(i + 1);
      input.setSelectionRange(i + 1, i + 1);
      return;
    }

    if (e.data === '3' && currentChar === 'a') {
      e.preventDefault();
      input.value =
        input.value.substring(0, i) + 'ǎ' + input.value.substring(i + 1);
      input.setSelectionRange(i + 1, i + 1);
      return;
    }

    if (e.data === '4' && currentChar === 'a') {
      e.preventDefault();
      input.value =
        input.value.substring(0, i) + 'à' + input.value.substring(i + 1);
      input.setSelectionRange(i + 1, i + 1);
      return;
    }
  }
</script>

<input
  class="textField"
  type="text"
  placeholder="ピンインを入力してください"
  on:beforeinput={handleInputEvent}
  on:focusin={focusin}
  bind:this={input}
/>

{#if visible}
  <div class="toneSelector">
    <button type="button" on:click={close}>閉じる</button>
    <button type="button" on:click={tone1}>1声</button>
    <button type="button" on:click={tone2}>2声</button>
    <button type="button" on:click={tone3}>3声</button>
    <button type="button" on:click={tone4}>4声</button>
  </div>
{/if}

<style>
  .textField {
    background-color: transparent;
    border: 2px solid var(--pinyin-text-field-blue-300);
    border-radius: 5px;
    padding: 4px 6px;
    color: var(--pinyin-text-field-slate-100);
    outline: none;
  }

  .toneSelector button {
    background-color: var(--pinyin-text-field-gray-400);
    border: none;
  }
</style>
