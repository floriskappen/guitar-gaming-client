import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Configuration } from "../../types/configuration";
import { Progress } from "@/components/ui/progress";
import { useNavigate } from "react-router-dom";

export const IndexRoute = () => {

    const [progress, setProgress] = useState(13)
    const navigate = useNavigate()

    useEffect(() => {
        const checkConfiguration = async () => {
            const configuration = await invoke<Configuration>("get_configuration")

            if (!!configuration.device) {
                await navigator.mediaDevices.getUserMedia({ audio: { echoCancellation: false }, video: false });
                let matchingDevices = (await navigator.mediaDevices.enumerateDevices()).filter((device) => {
                    return device.label === configuration.device
                })
                setProgress(100)

                if (matchingDevices && matchingDevices.length === 1) {
                    // The device that the user has configured can indeed be found
                    // This means we can go to the song selection route
                    navigate('/song-select')
                } else {
                    navigate('/input-device-select')
                }
            }
        }

        checkConfiguration().catch(console.error)

    }, [])

    return (
        <div className="w-screen h-screen flex items-center justify-center bg-neutral-900 text-white">
            <Progress value={progress} className="w-[60%]" />
        </div>
    )
}
