<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import Taskslist from "./lib/Taskslist.svelte";

    import Minimized from "./lib/assets/minimized.png"
    import Maxmimized from "./lib/assets/maximized.png"
    import Close from "./lib/assets/close.png"
</script>

<div data-tauri-drag-region class="titlebar">
    <button
        class="titlebar-button"
        id="titlebar-minimize"
        on:click={() => appWindow.minimize()}
    >
        <img
            src="{Minimized}"
            alt="maximize"
        />
    </button>
    <button
        class="titlebar-button"
        id="titlebar-maximize"
        on:click={() => appWindow.toggleMaximize()}
    >
        <img
            src="{Maxmimized}"
            alt="maximize"
        />
    </button>
    <button
        class="titlebar-button"
        id="titlebar-close"
        on:click={() => appWindow.close()}
    >
        <img src="{Close}" alt="close" width="16" height="16" />
    </button>
</div>
<main class="container">
    <Taskslist />
</main>

<style>
    .titlebar {
        height: 30px;
        background: #202020;
        user-select: none;
        display: flex;
        justify-content: flex-end;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
    }

    .titlebar-button {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        width: 30px;
        height: 30px;
        border-radius: 0;
        transition: 0.3s all;
        background-color: rgb(51, 51, 51);
    }

    .titlebar-button:hover {
        background: #5bbec3;
    }

    main {
        margin-top: 30px;
    }
</style>
