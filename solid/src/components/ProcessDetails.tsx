import { processStore } from "@/stores/process";
import type { Process } from "@/api";
import CpuChart from "@/components/CpuChart";
import { createEffect, createSignal } from "solid-js";

export interface Props {
    process: Process;
}

export default function ProcessDetails(props: Props) {
    const [values, setValues] = createSignal(Array(180).fill(undefined));

    let process = processStore(props.process.info.pid);

    createEffect(() => {
        const proc = process.process;
        if (proc) {
            setValues(([, ...cdr]) => [...cdr, proc.resources.cpu_usage]);
        }
    });
    return <>
        <h1>Process details: {props.process.info.name}</h1>
        <CpuChart values={values()} />
    </>
}

