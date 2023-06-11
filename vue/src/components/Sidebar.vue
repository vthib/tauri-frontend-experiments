<script setup lang="ts">
import ListBullet from "@/icons/heroicons/ListBullet.svg";
import ChartBarSquare from "@/icons/heroicons/ChartBarSquare.svg";
import { ref, watch } from "vue";

let activeElementIndex = ref<number>(0);
let elements = [
    {
        name: "processes",
        icon: ListBullet,
    },
    {
        name: "resources",
        icon: ChartBarSquare,
    },
];

let elemStyle = (active: boolean) => {
    let style = "p-2";
    if (active) {
        style += " bg-lime-600";
    }
    return style;
};

const emit = defineEmits<{
    (e: 'active', name: string): void
}>();

watch(activeElementIndex, (index) => {
    emit("active", elements[index].name);
});
</script>

<template>
    <aside class="fixed top-0 bottom-0 left-0 w-16 bg-lime-300">
        <ul class="flex flex-col">
            <li v-for="(element, index) in elements" :class="elemStyle(activeElementIndex == index)">
                <a :href="`#${element.name}`" class="hover:invert" @click="() => activeElementIndex = index">
                    <component :is="element.icon" />
                </a>
            </li>
        </ul>
    </aside>
</template>