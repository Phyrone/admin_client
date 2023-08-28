<script lang="ts">
    import type {IEvent, Terminal} from "xterm";
    import {FitAddon} from "xterm-addon-fit";
    import {watchResize} from "svelte-watch-resize";
    import {onMount} from "svelte";


    let fit_addon = new FitAddon();
    export let terminal: Terminal

    let terminal_element: HTMLElement;

    onMount(()=> {
        terminal.loadAddon(fit_addon)
        terminal.open(terminal_element)
        fit_addon.fit()
    })

    function resize() {
        console.log("resize")
        terminal.resize(5, 5)
        fit_addon.fit()
    }

</script>
<svelte:window on:resize={resize}/>
<div class="overflow-clip h-full" use:watchResize={resize} bind:this={terminal_element}></div>