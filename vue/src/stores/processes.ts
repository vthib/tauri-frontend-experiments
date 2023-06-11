import { getProcesses } from "@/api";
import type { Process } from "@/api";
import { ref, onMounted, onUnmounted } from 'vue';
import type { Ref } from 'vue';

export const processes = (): Ref<Process[]> => {
    let procs: Ref<Process[]> = ref([]);

    onMounted(() => {
        let interval = setInterval(() => {
            getProcesses().then((newProcs) => {
                procs.value = newProcs;
            });
        }, 1000);

        onUnmounted(() => clearInterval(interval));
    });

    return procs;
};
