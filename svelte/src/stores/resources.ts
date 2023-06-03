import { writable } from 'svelte/store';
import {
    subscribeToGlobalCpuUsage,
} from "@/api";

export function createGlobalCpuUsageStore() {
    let arr = Array(180).fill(undefined);
    let unlisten: () => void | undefined;
    const { subscribe, update } = writable<number[]>(arr);

    return {
        subscribe: (...args: Parameters<typeof subscribe>) => {
            let subscribed = true;

            const unsub = subscribe(...args);
            subscribeToGlobalCpuUsage((value) => update(old => {
                old.shift();
                old.push(value);
                return old;
            }))
                .then((stop) => {
                    if (subscribed) {
                        unlisten = stop;
                    } else {
                        stop();
                    }
                });

            return () => {
                subscribed = false;
                unsub();
                if (unlisten !== undefined) {
                    unlisten();
                }
            }
        },
    }
}

export const globalCpuUsage = createGlobalCpuUsageStore();
