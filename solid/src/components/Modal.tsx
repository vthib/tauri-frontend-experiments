import { ParentProps, createEffect, onMount } from "solid-js";

export interface Props {
    show: boolean,
    hide: () => void,
}

export default function Modal(props: ParentProps<Props>) {
    let dialog: HTMLDialogElement;

    onMount(() => {
        createEffect(() => {
            if (props.show) {
                dialog.showModal();
            }
        });
    });

    return <dialog
        ref={dialog!}
        onClose={() => props.hide()}
        onClick={() => { dialog.close(); props.hide() }}
        class="backdrop:backdrop-blur-sm border-solid rounded-md"
    >
        <div onClick={(e) => e.stopPropagation()}>
            {props.children}
        </div >
    </dialog >
}
