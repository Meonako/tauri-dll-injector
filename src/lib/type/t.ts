export type Task = {
    process_name: string,
    pid: number,
    memory_usage: string,
}

export type TasksList = {
    all: Task[]
    console: Task[]
    services: Task[]
    other: Task[]
}

export const sessionType = ["all", "console", "services", "other"];