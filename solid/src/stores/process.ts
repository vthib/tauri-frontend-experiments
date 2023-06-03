import { Store, createStore } from 'solid-js/store';
import { onCleanup } from 'solid-js';
import { getProcess } from "@/api";
import type { Process } from "@/api";

export interface Content {
    process?: Process;
}

export function processStore(pid: number): Store<Content> {
    const [process, setProcess] = createStore<Content>({});

    let interval = setInterval(() => {
        getProcess(pid).then((proc) => {
            setProcess('process', proc);
        });
    }, 1000);
    onCleanup(() => clearInterval(interval));

    return process;
}
