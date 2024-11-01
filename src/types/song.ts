
export type Song = {
    uuid: string
    title: string | null
    artists: string[]
    tuning: string[]
    duration_seconds: number | null
    audio: File | null
}
