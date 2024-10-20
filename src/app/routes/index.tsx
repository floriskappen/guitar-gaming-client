import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Configuration } from "../../types/configuration";
import { Progress } from "@/components/ui/progress";

export const IndexRoute = () => {

    const [progress, setProgress] = useState(13)

    useEffect(() => {
        const checkConfiguration = async () => {
            const configuration = await invoke<Configuration>("get_configuration")

            if (!!configuration.device) {
                await navigator.mediaDevices.getUserMedia({ audio: true });
                let matchingDevices = (await navigator.mediaDevices.enumerateDevices()).filter((device) => {
                    return device.label === configuration.device
                })
                setProgress(100)

                if (matchingDevices && matchingDevices.length === 1) {
                    // The device that the user has configured can indeed be found
                    // This means we can go to the song selection route
                }
            }
        }

        checkConfiguration().catch(console.error)

    }, [])

    return (
        <div className="w-screen h-screen flex items-center justify-center bg-gray-900 text-white">
            <Progress value={progress} className="w-[60%]" />
        </div>
    )
}
