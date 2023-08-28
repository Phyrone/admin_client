<script>
    import TerminalContainer from "$components/TerminalContainer.svelte";
    import {Terminal} from "xterm";
    import {invoke} from "@tauri-apps/api";

    let terminal = new Terminal()

    async function click() {
        let result = await invoke('hi')
        terminal.writeln(JSON.stringify(result))
    }
</script>

<div class="flex-auto max-h-full">
    <TerminalContainer terminal={terminal}/>
</div>
<button class="btn btn-primary" on:click={click}>Click</button>


