<script setup lang="ts">
import { ref, watchEffect } from 'vue';

const props = defineProps<{
    show: boolean,
}>();

const dialog = ref<HTMLDialogElement | undefined>(undefined);
watchEffect(() => {
    if (dialog.value && props.show) {
        dialog.value.showModal();
    }
});

const emit = defineEmits<{
    (e: 'close'): void
}>();
</script>

<template>
    <dialog ref="dialog" @close="emit('close')" @click.self="() => dialog!.close()"
        class="backdrop:backdrop-blur-sm border-solid rounded-md">
        <div @click.stop>
            <slot />
        </div>
    </dialog>
</template>
