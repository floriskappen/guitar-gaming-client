import { SongLibrary } from '@/features/song-select/types/song-library';
import { atom, PrimitiveAtom } from 'jotai'

export const songLibraryAtom = atom<SongLibrary | null>(
    null
) as PrimitiveAtom<SongLibrary | null>;
