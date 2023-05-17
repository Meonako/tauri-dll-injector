<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { open, message } from "@tauri-apps/api/dialog";

    import { fly } from "svelte/transition";

    import { dllPath } from "./store";
    import { sessionType } from "./type/t";
    import type { TasksList } from "./type/t";

    let targetPid: number;
    let highlight: number;
    let searchTimeout: string | number | NodeJS.Timeout;

    let search = "";
    let error = "";

    $: if (error) {
        message(error, { title: "Error", type: "error" });
    }

    let filterType = sessionType[1];
    let showFilterDropdown = false;

    async function refresh(): Promise<TasksList> {
        return invoke("get_tasks_list", { filter: search });
    }

    let refreshData = refresh();
</script>

<div>
    <div class="sticky">
        <button on:click={() => (refreshData = refresh())}>Refresh</button>
        <div style="position: relative; display: flex; flex-direction: row">
            <input
                type="text"
                placeholder="Search Process Name/ID"
                bind:value={search}
                on:input={() => {
                    if (searchTimeout) {
                        clearTimeout(searchTimeout);
                    }

                    searchTimeout = setTimeout(
                        () => (refreshData = refresh()),
                        300
                    );
                }}
            />
            <div
                style="position: fixed; display: flex; flex-direction: column; justify-content: center; width: 13%; margin-left: 15rem"
            >
                <button
                    on:click={() => (showFilterDropdown = !showFilterDropdown)}
                    style="text-align: left"
                >
                    <span style="color: green">
                        {filterType.toUpperCase()}
                    </span>
                </button>
                {#if showFilterDropdown}
                    {#each sessionType as type, idx}
                        <button
                            in:fly={{ x: 200, delay: (idx + 1) * 30 }}
                            out:fly={{ x: -200, delay: (idx + 1) * 30 }}
                            on:click={() => {
                                filterType = type;
                                showFilterDropdown = false;
                            }}
                        >
                            {type.toUpperCase()}
                        </button>
                    {/each}
                {/if}
            </div>
        </div>
        <div>
            <button
                on:click={async () => {
                    let newPath = await open({
                        title: "DLL to inject",
                        filters: [
                            {
                                name: "DLL",
                                extensions: ["dll"],
                            },
                        ],
                    });

                    if (!newPath || typeof newPath != 'string') return;

                    dllPath.set(newPath);
                }}
            >
                Browse DLL
            </button>
            <input
                type="text"
                placeholder="Path to DLL"
                bind:value={$dllPath}
            />
            <button
                on:click={async () => {
                    if (!targetPid || !$dllPath) {
                        return;
                    }

                    error = "";
                    error = await invoke("inject_dll", {
                        pid: targetPid,
                        pathToDll: $dllPath,
                    });
                }}
            >
                Inject
            </button>
        </div>
    </div>
    {#await refreshData then tasklist}
        <div
            class="row"
            in:fly={{ y: 200, delay: 150 }}
            out:fly={{ y: 200, duration: 150 }}
        >
            <table>
                <thead>
                    <tr>
                        <th>Process Name</th>
                        <th>Process ID</th>
                        <th>RAM Usage</th>
                    </tr>
                </thead>
                <tbody>
                    {#each tasklist[filterType] as task}
                        <tr
                            on:click={() => {
                                targetPid = task.pid;
                                highlight = task.pid;
                            }}
                            class:highlight={highlight == task.pid}
                        >
                            <td>{task.process_name}</td>
                            <td>{task.pid}</td>
                            <td>{task.memory_usage}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/await}
</div>

<style>
    .row {
        overflow-y: scroll;
    }

    table {
        margin-top: 3rem;
        width: 100%;
    }

    thead {
        border-bottom: 1px solid rgb(102, 102, 102);
    }

    tbody tr {
        background-color: rgb(22, 22, 22);
        cursor: pointer;
    }

    tbody tr:hover {
        background-color: rgb(75, 75, 75);
    }

    tbody tr.highlight {
        background-color: rgba(59, 59, 59, 0.9);
    }

    tbody tr:nth-child(2n) {
        background-color: rgb(36, 36, 36);
    }

    tbody tr:nth-child(2n):hover {
        background-color: rgb(75, 75, 75);
    }

    tbody tr:nth-child(2n).highlight {
        background-color: rgba(75, 75, 75, 0.8);
    }

    .sticky {
        position: fixed;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        width: 100%;
        background-color: rgba(0, 0, 0, 0.6);
    }

    .sticky input {
        text-align: center;
        border: 1px solid rgba(112, 112, 252, 0.5);
    }

    .sticky input:focus {
        border: 1px solid rgba(112, 112, 252, 1);
    }
</style>
