import { globalCpuUsage } from "@/stores/resources";
import CpuChart from "@/components/CpuChart";
import { Accessor, Show, createEffect, createSignal, getOwner, runWithOwner } from "solid-js";

export default function Resources() {
    const [values, setValues] = createSignal<Accessor<Array<number | undefined>> | undefined>(undefined);

    globalCpuUsage().then((values) => setValues(() => values));

    return (
        <Show when={values() !== undefined} fallback={<p>Loading...</p>}>
            <CpuChart values={values()!()} />
        </Show>
    );
}
