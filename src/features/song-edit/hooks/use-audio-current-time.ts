import { useAtomValue } from "jotai";
import { useState, useEffect, useRef } from "react";
import { audioCurrentTimeAtom, audioPausedAtom } from "../atoms/audio";

const playbackSpeed = 1.0

const useAudioCurrentTime = () => {
    const audioCurrentTime = useAtomValue(audioCurrentTimeAtom)
    const [localAudioCurrentTime, setLocalAudioCurrentTime] = useState(0);
    const startTimestampRef = useRef<number | null>(null); // Tracks the playback start time
    const lastTimeRef = useRef<number>(0); // Tracks the last time update was applied
    
    const audioPaused = useAtomValue(audioPausedAtom)

    const updateTime = () => {
        if (!audioPaused && startTimestampRef.current !== null) {
            const now = performance.now(); // High-resolution time
            const delta = (now - lastTimeRef.current) / 1000; // Time in seconds since last update

            // Update local time with playback speed adjustment
            setLocalAudioCurrentTime((prev) => prev + delta * playbackSpeed);

            lastTimeRef.current = now; // Update the last time reference

            requestAnimationFrame(updateTime); // Schedule the next frame
        }
    };

    useEffect(() => {
        if (!audioPaused) {
            // Start tracking time
            if (startTimestampRef.current === null) {
                startTimestampRef.current = performance.now();
            }
            lastTimeRef.current = performance.now(); // Initialize the last update time
            requestAnimationFrame(updateTime);
        } else {
            startTimestampRef.current = null; // Reset tracking when paused
        }
    }, [audioPaused, playbackSpeed]);

    useEffect(() => {
        setLocalAudioCurrentTime(audioCurrentTime)
    }, [audioCurrentTime])

    return localAudioCurrentTime;
};

export default useAudioCurrentTime;
