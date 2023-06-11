import {
    subscribeToGlobalCpuUsage,
} from "@/api";
import { Ref, onMounted, onUnmounted, ref } from 'vue';

export function globalCpuUsage(): Ref<number[]> {
    let arr = Array(180).fill(undefined);
    let cpu_usage = ref<number[]>(arr);

    const stop = ref<Function | undefined>(undefined);
    onMounted(async () => {
        stop.value = await subscribeToGlobalCpuUsage((value) => {
            const [, ...rest] = cpu_usage.value;
            cpu_usage.value = [...rest, value];
        });
    });

    onUnmounted(() => {
        if (stop.value) {
            stop.value();
        }
    });

    return cpu_usage;
}
