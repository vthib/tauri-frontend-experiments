import { invoke } from '@tauri-apps/api/tauri'

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

export async function getProcesses(): Promise<Process[]> {
    return await invoke('get_processes')
}
