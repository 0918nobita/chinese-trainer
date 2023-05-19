<script lang="ts">
  let input: HTMLInputElement;

  let visible = false;

  let value = '';

  function getCurrentChar(): [number, string] {
    const selectionStart = input.selectionStart ?? value.length;
    return [
      selectionStart - 1,
      value.substring(selectionStart - 1, selectionStart),
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
      value = value.substring(0, i) + 'ā' + value.substring(i + 1);
    }
  }

  function tone2() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      value = value.substring(0, i) + 'á' + value.substring(i + 1);
    }
  }

  function tone3() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      value = value.substring(0, i) + 'ǎ' + value.substring(i + 1);
    }
  }

  function tone4() {
    input.focus();
    const [i, currentChar] = getCurrentChar();
    if (currentChar === 'a') {
      value = value.substring(0, i) + 'à' + value.substring(i + 1);
    }
  }
</script>

<input
  class="textField"
  type="text"
  placeholder="ピンインを入力してください"
  on:focusin={focusin}
  bind:this={input}
  bind:value
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
  :root {
    --pinyin-text-field-slate-100: #f1f5f9;
    --pinyin-text-field-gray-400: #9ca3af;
    --pinyin-text-field-blue-300: #93c5fd;
  }

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
