import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event';

export interface Process {
    info: {
        pid: number,
        name: string,
        user?: string,
        status: string,
    }
    resources: {
        memory: number,
        virtual_memory: number,
        cpu_usage: number,
    }
}

export async function getProcess(pid: number): Promise<Process | undefined> {
    return await invoke('get_process', { pid })
}

export async function getProcesses(): Promise<Process[]> {
    return await invoke('get_processes')
}

export async function subscribeToGlobalCpuUsage(cb: (number) => void): Promise<() => void> {
    await invoke('subscribe_global_cpu_usage');
    const unlisten = await listen('global_cpu_usage', (event) => {
        cb(event.payload)
    });
    return () => {
        unlisten();
        invoke('unsubscribe_global_cpu_usage');
    }
}
