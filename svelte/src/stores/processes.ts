import { readable } from 'svelte/store';
import { getProcesses } from "@/api";
import type { Process } from "@/api";

export const processes = readable<Process[]>([], (set) => {
    let interval = setInterval(() => {
        getProcesses().then((procs) => {
            set(procs);
        });
    }, 1000);

    return () => clearInterval(interval);
});
