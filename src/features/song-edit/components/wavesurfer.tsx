import { useAtom } from "jotai"
import { songEditAtom } from "../atoms/song-edit"
import { SetStateAction, useEffect, useRef, useState } from "react"
import WaveSurfer from "wavesurfer.js";

export const Wavesurfer = () => {
const [songEdit, setSongEdit] = useAtom(songEditAtom)
  const waveformRef = useRef(null);
  const [wavesurfer, setWavesurfer] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (waveformRef.current) {
        const ws = WaveSurfer.create({
            container: waveformRef.current,
            waveColor: "violet",
            progressColor: "purple",
            // minPxPerSec: 100,
        });
        setWavesurfer(ws);
        ws.on("redrawcomplete", () => {
            setLoading(false)
        })
        ws.on("click", () => {
            ws.play()
        })

      const file = songEdit!.audio;
    if (file && ws) {
        console.log("ayy")
        ws.loadBlob(file);
    }

      // Cleanup wavesurfer instance on component unmount
      return () => ws.destroy();
    }
  }, []);

  return (
    <>
      <div ref={waveformRef} style={{ width: "100%", height: "100px" }}></div>

      {
        loading ? <p>loaddingg</p> : null
      }
    </>
  );
}