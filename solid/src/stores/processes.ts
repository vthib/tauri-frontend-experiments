import { Store, createStore, reconcile } from 'solid-js/store';
import { onCleanup } from 'solid-js';
import { getProcesses } from "@/api";
import type { Process } from "@/api";

export function processes(): Store<Process[]> {
    const [processes, setProcesses] = createStore<Process[]>([]);

    let interval = setInterval(() => {
        getProcesses().then((procs) => {
            setProcesses(reconcile(procs))
        });
    }, 1000);
    onCleanup(() => clearInterval(interval));

    return processes;
};
