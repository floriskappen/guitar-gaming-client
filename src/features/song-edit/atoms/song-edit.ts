import { Song } from '@/types/song';
import { atom, PrimitiveAtom } from 'jotai'

export const audioFileAtom = atom<File | null>(
    null
) as PrimitiveAtom<File | null>;

export const songEditAtom = atom<Song | null>(
    null
) as PrimitiveAtom<Song | null>;
