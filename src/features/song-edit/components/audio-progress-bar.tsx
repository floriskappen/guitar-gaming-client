import { useAtom, useAtomValue } from "jotai"
import { audioCurrentTimeAtom, audioDurationAtom, audioLoadingAtom, audioPausedAtom, audioRefAtom } from "../atoms/audio"
import { useEffect, useRef, useState } from "react"
import { twMerge } from "tailwind-merge"
import { Button } from "@/components/ui/button"
import { Pause, Play } from "lucide-react"
import { Skeleton } from "@/components/ui/skeleton"
import { Separator } from "@/components/ui/separator"

export const AudioProgressBar = () => {
    const audioCurrentTime = useAtomValue(audioCurrentTimeAtom)
    const audioDuration = useAtomValue(audioDurationAtom)
    const audioRef = useAtomValue(audioRefAtom)
    const audioLoading = useAtomValue(audioLoadingAtom)
    const [audioPaused, setAudioPaused] = useAtom(audioPausedAtom)
    
    const [dimensions, setDimensions] = useState({ width: 0, left: 0 });
    const [isDragging, setIsDragging] = useState(false);
    const [audioPausedBeforeDrag, setAudioPausedBeforeDrag] = useState(false);
    const barRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        const updateDimensions = () => {
            if (barRef.current) {
                const { width, left } = barRef.current.getBoundingClientRect();
                setDimensions({ width, left });
            }
        };
    
        // Create a ResizeObserver to track changes
        const resizeObserver = new ResizeObserver(updateDimensions);
        if (barRef.current) {
            resizeObserver.observe(barRef.current);
        }
    
        // Update dimensions initially and clean up observer on unmount
        updateDimensions();
        return () => {
            if (barRef.current) {
                resizeObserver.unobserve(barRef.current);
            }
        };
    }, [audioLoading]);

    function formatTime(seconds: number) {
        const minutes = Math.floor(seconds / 60);
        const secs = Math.floor(seconds % 60);
        const milliseconds = Math.floor((seconds * 1000) % 100);
      
        // Format with leading zeros if necessary
        const minutesStr = String(minutes).padStart(2, "0");
        const secondsStr = String(secs).padStart(2, "0");
        const millisecondsStr = String(milliseconds).padStart(2, "0");
      
        return `${minutesStr}:${secondsStr}:${millisecondsStr}`;
    }

    const updateProgress = (clientX: number) => {
        if (dimensions.width > 0 && audioRef?.current) {
            const clickPosition = clientX - dimensions.left;
            const clickPercentage = Math.min(
              1,
              Math.max(0, (clickPosition / dimensions.width))
            );
            audioRef.current.currentTime = audioDuration * clickPercentage
            // Use this percentage to update audio playback or other actions
        }
    };

    const handleBarMouseDown = (event: React.MouseEvent<HTMLElement>) => {
        setIsDragging(true);
        updateProgress(event.clientX);
        setAudioPausedBeforeDrag(audioPaused)
        audioRef?.current?.pause();
    };
    
    const handleBarMouseMove = (event: MouseEvent) => {
        if (isDragging) {
          updateProgress(event.clientX);
        }
    };
    
    const handleBarMouseUp = () => {
        if (isDragging) {
            setIsDragging(false);
            if (audioPausedBeforeDrag === false) {
                audioRef?.current?.play();
            }
        }
    };

    // Attach mousemove and mouseup listeners to the document when dragging
    useEffect(() => {
        if (isDragging) {
            document.addEventListener("mousemove", handleBarMouseMove);
            document.addEventListener("mouseup", handleBarMouseUp);
        } else {
            document.removeEventListener("mousemove", handleBarMouseMove);
            document.removeEventListener("mouseup", handleBarMouseUp);
        }

        return () => {
            document.removeEventListener("mousemove", handleBarMouseMove);
            document.removeEventListener("mouseup", handleBarMouseUp);
        };
    }, [isDragging]);

    return (
        <div>
            <Separator />
            <div className="flex w-full h-[48px]">
                {
                    !audioLoading ? (
                        <>
                            <div className="flex items-center w-[200px]">
                                <Button className="h-[48px] w-[48px]" onClick={(() => {
                                    setAudioPaused(!audioPaused)
                                })} variant={"ghost"}>
                                    {
                                        audioPaused ? <Play size="16" /> : <Pause size="16" />
                                    }
                                </Button>
                                <div className="w-full px-2 flex items-center justify-center">
                                    <p>{formatTime(audioCurrentTime)}</p>
                                </div>
                            </div>
                            <div className="w-full">
                                <div
                                    ref={barRef}
                                    className="w-full bg-neutral-800 h-[48px] bottom-0 flex items-center justify-center"
                                    onMouseDown={handleBarMouseDown}
                                >
                                    <div className="w-full h-[4px] bg-neutral-600"></div>
                                </div>

                                {/* Cursor */}
                                <div
                                    className={twMerge(
                                        "h-[48px] w-[4px] bg-white absolute overflow-hidden",
                                    )}
                                    style={{
                                        left: `${dimensions.left + (
                                            (dimensions.width - 4) * (audioCurrentTime / audioDuration)
                                        )}px`,
                                        transform: `translate(0%, -100%)`,
                                    }}
                                ></div>

                            </div>
                        </>
                    ) : (
                        <div className="w-full h-full">
                            <Skeleton className="w-full h-full" />
                        </div>
                    )
                }
            </div>
            <Separator />
        </div>
    )
}
