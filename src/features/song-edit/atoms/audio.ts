import { atom, PrimitiveAtom } from "jotai"
import { MutableRefObject } from "react";

export const audioRefAtom = atom<MutableRefObject<HTMLAudioElement | null> | null>(
    null
) as PrimitiveAtom<MutableRefObject<HTMLAudioElement | null> | null>;

export const audioBlobAtom = atom<Blob | null>(
    null
) as PrimitiveAtom<Blob | null>;
export const audioCurrentTimeAtom = atom(0)
export const audioDurationAtom = atom(0)
export const audioPausedAtom = atom(true)
export const audioLoadingAtom = atom(true)
