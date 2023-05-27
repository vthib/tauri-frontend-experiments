import { readable } from 'svelte/store';
import type { Readable } from 'svelte/store';
import { getProcess } from "@/api";
import type { Process } from "@/api";

export function generateProcessStore(pid: number): Readable<Process | undefined> {
    return readable(undefined, (set) => {
        let interval = setInterval(() => {
            getProcess(pid).then((proc) => {
                set(proc);
            });
        }, 1000);

        return () => clearInterval(interval);
    });
}
