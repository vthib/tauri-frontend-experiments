import {
    subscribeToGlobalCpuUsage,
} from "@/api";
import { Accessor, createSignal, getOwner, onCleanup, runWithOwner } from 'solid-js';

export async function globalCpuUsage(): Promise<Accessor<Array<number | undefined>>> {
    let arr = Array(180).fill(undefined);
    let [history, setHistory] = createSignal<Array<number | undefined>>(arr);

    const owner = getOwner();
    const stop = await subscribeToGlobalCpuUsage((value) => {
        setHistory(([, ...cdr]) => [...cdr, value]);
    });
    runWithOwner(owner, () => onCleanup(stop));

    return history;
}
