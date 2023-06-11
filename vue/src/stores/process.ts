import { onMounted, onUnmounted, ref } from 'vue';
import type { Ref } from 'vue';
import { getProcess } from "@/api";
import type { Process } from "@/api";

export function generateProcessStore(pid: number): Ref<Process | undefined> {
    const proc = ref<Process | undefined>();

    onMounted(() => {
        let interval = setInterval(() => {
            getProcess(pid).then((v) => {
                proc.value = v;
            });
        }, 1000);

        onUnmounted(() => clearInterval(interval));
    });

    return proc;
}
