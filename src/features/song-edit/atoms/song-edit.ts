import { Song } from '@/types/song';
import { atom, PrimitiveAtom } from 'jotai'

export const songEditAtom = atom<Song | null>(
    null
) as PrimitiveAtom<Song | null>;
