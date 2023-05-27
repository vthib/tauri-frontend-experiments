<script lang="ts">
    export let show = false;

    let dialog;
    $: if (dialog && show) dialog.showModal();
</script>

<dialog
    bind:this={dialog}
    on:close
    on:click|self={() => dialog.close()}
    on:keydown={() => {}}
    class="backdrop:backdrop-blur-sm border-solid rounded-md"
>
    <div on:click|stopPropagation on:keydown={() => {}}>
        <slot />
    </div>
</dialog>
