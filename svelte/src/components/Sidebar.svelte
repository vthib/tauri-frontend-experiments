<script lang="ts">
    import ListBullet from "@/icons/heroicons/list-bullet.svelte";
    import ChartBarSquare from "@/icons/heroicons/chart-bar-square.svelte";
    import { createEventDispatcher } from "svelte";

    let activeElementIndex: number = 0;
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

    let elemStyle = (active) => {
        let style = "p-2";
        if (active) {
            style += " bg-lime-600";
        }
        return style;
    };

    const dispatch = createEventDispatcher();
    $: {
        dispatch("active", elements[activeElementIndex].name);
    }
</script>

<aside class="fixed top-0 bottom-0 left-0 w-16 bg-lime-300">
    <ul class="flex flex-col">
        {#each elements as element, index}
            <li class={elemStyle(activeElementIndex == index)}>
                <a
                    href={`#${element.name}`}
                    class="hover:invert"
                    on:click={() => (activeElementIndex = index)}
                >
                    <svelte:component this={element.icon} />
                </a>
            </li>
        {/each}
    </ul>
</aside>

<style lang="postcss">
    .active {
    }
</style>
