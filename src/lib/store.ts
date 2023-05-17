import { writable, type Writable } from "svelte/store";

export const dllPath = load();

function load(): Writable<string> {
    return writable(localStorage.getItem('last_dll') ?? "");
}

dllPath.subscribe((x) => localStorage.setItem('last_dll', x))