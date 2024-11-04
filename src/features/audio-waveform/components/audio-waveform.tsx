import { useEffect, useRef } from "react";
import WaveSurfer from "wavesurfer.js";

export const AudioWaveform = ({ audioBlob }: { audioBlob: Blob }) => {
    const wavesurferContainerRef = useRef<HTMLDivElement | null>(null)
    const wavesurferRef = useRef<WaveSurfer | null>(null)

    useEffect(() => {
        if (!wavesurferRef.current && wavesurferContainerRef.current) {
            const wavesurfer = WaveSurfer.create({
                container: wavesurferContainerRef.current,
                waveColor: "violet",
                progressColor: "purple",
            })
            wavesurferRef.current = wavesurfer
            console.log("yoo loading audio blob", audioBlob)
            wavesurfer.loadBlob(audioBlob).then(() => {
                "ayyy loaded"
            })
            wavesurferRef.current.on("ready", () => {
                console.log("playing da songg")
                // wavesurfer.play()
            })
        }

        // return () => {
        //     console.log("bye bye wavesurfer")
        //     wavesurferRef.current?.destroy()
        // }
    }, [])

    return (
        <div>
            <div ref={wavesurferContainerRef} />
        </div>
    )
}
