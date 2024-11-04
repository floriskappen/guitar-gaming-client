import { getSongAudioBlobByUuid } from "@/functions/get-song-audio-blob-by-uuid"
import { useSetAtom } from "jotai"
import { useEffect, useRef, useState } from "react"
import { useParams } from "react-router-dom"
import { audioBlobAtom, audioCurrentTimeAtom, audioDurationAtom, audioLoadingAtom, audioPausedAtom, audioRefAtom } from "../atoms/audio"

export const Audio = () => {
    let { uuid } = useParams()
    const [audioUrl, setAudioUrl] = useState<string | null>(null);
    const audioRef = useRef<HTMLAudioElement | null>(null);

    const setAudioRef = useSetAtom(audioRefAtom);
    const setAudioBlob = useSetAtom(audioBlobAtom);
    const setAudioCurrentTime = useSetAtom(audioCurrentTimeAtom);
    const setAudioDuraton = useSetAtom(audioDurationAtom);
    const setAudioPaused = useSetAtom(audioPausedAtom);
    const setAudioLoading = useSetAtom(audioLoadingAtom);

    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.code === "Space") {
            const activeElement = document.activeElement as HTMLElement;
            const isInputFocused =
                activeElement &&
                (activeElement.tagName === "INPUT" ||
                activeElement.tagName === "TEXTAREA" ||
                activeElement.isContentEditable);

            if (!isInputFocused) {
                event.preventDefault(); // Prevents scrolling on space press
                if (audioRef.current) {
                    if (audioRef.current.paused) {
                        audioRef.current.play();
                    } else {
                        audioRef.current.pause();
                    }
                }
            }
        }
    };
  

    useEffect(() => {
        setAudioCurrentTime(0)

        if (!audioUrl) {
            setAudioLoading(true)
            getSongAudioBlobByUuid(uuid!).then((blob) => {
                setAudioUrl(URL.createObjectURL(blob))
                setAudioBlob(blob)
                setAudioLoading(false)
            })
        }

        window.addEventListener("keydown", handleKeyDown)
        
        return () => {
            window.removeEventListener("keydown", handleKeyDown)
            setAudioLoading(true)
        }
    }, [uuid])

    useEffect(() => {
        if (audioUrl && audioRef.current) {
            setAudioRef(audioRef)
            setAudioPaused(true)
        }
    }, [audioUrl, setAudioRef, setAudioPaused]);

    const handleTimeUpdate = () => {
        if (audioRef.current) {
            setAudioCurrentTime(audioRef.current.currentTime);
        }
    };

    const handleLoadedMetadata = () => {
        setTimeout(() => {
            if (audioRef.current) {
                setAudioDuraton(audioRef.current.duration);
            }
        }, 50)
    };

    const handlePause = () => {
        setAudioPaused(true)
    }

    const handlePlay = () => {
        setAudioPaused(false)
    }
    
    return <>
        {
            audioUrl ? (
                <audio
                    onPause={handlePause}
                    onPlay={handlePlay}
                    ref={audioRef}
                    src={audioUrl}
                    preload="auto"
                    onTimeUpdate={handleTimeUpdate}
                    onLoadedMetadata={handleLoadedMetadata}
                />
            ) : null
        }
    </>
}
