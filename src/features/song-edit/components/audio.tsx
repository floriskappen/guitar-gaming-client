import { getSongAudioBlobByUuid } from "@/functions/get-song-audio-blob-by-uuid"
import { useAtom, useSetAtom } from "jotai"
import { useEffect, useRef, useState } from "react"
import { useParams } from "react-router-dom"
import { audioCurrentTimeAtom, audioDurationAtom, audioLoadingAtom, audioPausedAtom } from "../atoms/audio"

export const Audio = () => {
    const { uuid } = useParams()
    const audioContextRef = useRef<AudioContext | null>(null)
    const sourceRef = useRef<AudioBufferSourceNode | null>(null)
    const startTimestampRef = useRef<number | null>(null) // Tracks when playback started
    const elapsedTimeRef = useRef<number>(0); // Tracks total elapsed time on pause

    const setAudioCurrentTime = useSetAtom(audioCurrentTimeAtom)
    const setAudioDuration = useSetAtom(audioDurationAtom)
    const [audioPaused, setAudioPaused] = useAtom(audioPausedAtom)
    const setAudioLoading = useSetAtom(audioLoadingAtom)

    const [audioUrl, setAudioUrl] = useState<string | null>(null)
    const [audioBuffer, setAudioBuffer] = useState<AudioBuffer | null>(null)

    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.code === "Space") {
            const activeElement = document.activeElement as HTMLElement;
            const isInputFocused =
                activeElement &&
                (activeElement.tagName === "INPUT" ||
                activeElement.tagName === "TEXTAREA" ||
                activeElement.isContentEditable);
            if (!isInputFocused) {
                event.preventDefault()
                setAudioPaused((prev) => !prev)
            }
        }
    }

    const playAudioContext = () => {
        if (!audioContextRef.current || !audioBuffer) return

        // Create a new BufferSource node
        const source = audioContextRef.current.createBufferSource()
        source.buffer = audioBuffer
        source.connect(audioContextRef.current.destination)

        // Resume playback from the correct position
        source.start(0, elapsedTimeRef.current);
        sourceRef.current = source;

        // Mark the start timestamp for tracking elapsed time
        startTimestampRef.current = audioContextRef.current.currentTime - elapsedTimeRef.current;

        source.onended = () => setAudioPaused(true)
    }

    const pauseAudioContext = () => {
        if (sourceRef.current) {
            sourceRef.current.stop()
            sourceRef.current = null

            if (audioContextRef.current && startTimestampRef.current !== null) {
                elapsedTimeRef.current = audioContextRef.current.currentTime - startTimestampRef.current;
            }
        }
    }

    useEffect(() => {
        if (audioPaused) {
            pauseAudioContext()
        } else {
            playAudioContext()
        }
    }, [audioPaused])

    useEffect(() => {
        setAudioCurrentTime(0);

        if (!audioUrl) {
            setAudioLoading(true);
            getSongAudioBlobByUuid(uuid!).then((blob) => {
                setAudioUrl(URL.createObjectURL(blob));
                setAudioLoading(false);
            });
        }

        return () => {
            setAudioLoading(true);

            if (audioContextRef.current) {
                audioContextRef.current.close();
            }
        };
    }, [uuid]);

    useEffect(() => {
        window.addEventListener("keydown", handleKeyDown);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    }, []);

    useEffect(() => {
        if (audioUrl) {
            audioContextRef.current = new window.AudioContext();
            fetch(audioUrl)
                .then(response => response.arrayBuffer())
                .then(arrayBuffer => {
                    return audioContextRef.current!.decodeAudioData(arrayBuffer);
                })
                .then(buffer => {
                    setAudioBuffer(buffer);
                    setAudioDuration(buffer.duration);
                    setAudioPaused(true);
                });
        }
    }, [audioUrl]);
    
    return <></>
}
