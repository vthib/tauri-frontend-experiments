import { Icon } from "solid-heroicons";
import { chartBarSquare, listBullet } from "solid-heroicons/outline";
import { For, createEffect, createSignal } from "solid-js";

export interface Props {
    onActive: (name: string) => void,
}

export default function Sidebar(props: Props) {
    const elements = [
        {
            name: "processes",
            iconPath: listBullet,
        },
        {
            name: "resources",
            iconPath: chartBarSquare
        },
    ];

    const [activeElementIndex, setActiveElementIndex] = createSignal(0);
    createEffect(() => {
        props.onActive(elements[activeElementIndex()].name);
    })

    const elemStyle = (active: boolean) => {
        let style = "p-2";
        if (active) {
            style += " bg-lime-600";
        }
        return style;
    };

    return (
        <aside class="fixed top-0 bottom-0 left-0 w-16 bg-lime-300">
            <ul class="flex flex-col">
                <For each={elements}>{(element, index) =>
                    <li class={elemStyle(activeElementIndex() == index())}>
                        <a
                            href={`#${element.name}`}
                            class="hover:invert"
                            onClick={() => setActiveElementIndex(index())}
                        >
                            <Icon path={element.iconPath} />
                        </a>
                    </li>
                }</For>
            </ul>
        </aside >
    );
}
