export type SongMetadata = {
    uuid: string
    title: string
    artists: string[]
    tuning: string[6]
    duration_seconds: number
}

export type SongLibrary = {
    songs: SongMetadata[]
}