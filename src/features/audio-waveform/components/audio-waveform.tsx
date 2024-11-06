import { Button } from "@/components/ui/button";
import { WaveformData } from "@/types/waveform_data";
import { useEffect, useRef, useState } from "react";

export const AudioWaveform = ({ waveformData, currentTime, duration }: { waveformData: WaveformData, currentTime: number, duration: number }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const [containerDimensions, setContainerDimensions] = useState({ width: 0, height: 0, left: 0 });
    const [zoom, setZoom] = useState(1);
    const [width, setWidth] = useState(800);
    const [height, setHeight] = useState(200);

    const visibleWindow = 10; // seconds to display around currentTime
    const halfWindow = visibleWindow / 2;

    useEffect(() => {
        const updateDimensions = () => {
            if (containerRef.current) {
                const { width, height, left } = containerRef.current.getBoundingClientRect();
                setContainerDimensions({ width, height, left });
            }
        };
    
        // Create a ResizeObserver to track changes
        const resizeObserver = new ResizeObserver(updateDimensions);
        if (containerRef.current) {
            resizeObserver.observe(containerRef.current);
        }
    
        // Update dimensions initially and clean up observer on unmount
        updateDimensions();
        return () => {
            if (containerRef.current) {
                resizeObserver.unobserve(containerRef.current);
            }
        }
    }, []);

    useEffect(() => {
        if (waveformData && canvasRef.current) {
            const canvas = canvasRef.current;
            const ctx = canvas.getContext('2d')!;
    
            // Set canvas dimensions with higher pixel density for smooth rendering
            canvas.width = containerDimensions.width * 2;
            canvas.height = containerDimensions.height * 2;
            ctx.scale(2, 2);
    
            // Clear the canvas
            ctx.clearRect(0, 0, containerDimensions.width, containerDimensions.height);
    
            // Configure bar style
            ctx.fillStyle = '#0099ff';

            // Calculate the index range for the 10-second window around currentTime
            const samplesPerSecond = waveformData.amplitudes.length / duration;
            const startSample = Math.max(0, Math.floor((currentTime - halfWindow) * samplesPerSecond));
            const endSample = Math.min(waveformData.amplitudes.length, Math.floor((currentTime + halfWindow) * samplesPerSecond));

            // Calculate step size and bar containerDimensions.width based on zoom level
            const visibleData = waveformData.amplitudes.slice(startSample, endSample);
            const step = Math.max(1, Math.floor(visibleData.length / containerDimensions.width));
            const barWidth = containerDimensions.width / (visibleData.length / step);

            // Draw each bar for the waveform
            for (let i = 0; i < containerDimensions.width; i++) {
                const dataIndex = Math.floor(i * step);
                const amplitude = visibleData[dataIndex] || 0;
                const barHeight = amplitude * (containerDimensions.height / 2);

                // Draw bar for the top half
                ctx.fillRect(i * barWidth, containerDimensions.height / 2 - barHeight, barWidth, barHeight);

                // Draw mirrored bar for the bottom half
                ctx.fillRect(i * barWidth, containerDimensions.height / 2, barWidth, barHeight);
            }
        }
    }, [waveformData, containerDimensions, zoom, currentTime]);

    const handleZoomIn = () => setZoom((z) => Math.min(10, z * 1.2));
    const handleZoomOut = () => setZoom((z) => Math.max(0.5, z / 1.2));

    return (
        <div className="w-full flex items-center justify-center">
            <div className="w-full h-[48px]" ref={containerRef}>
                <canvas ref={canvasRef} className="h-[48px] w-full absolute" style={{ width: containerDimensions.width, left: containerDimensions.left }} />
                <div className="w-full flex justify-center items-center">
                    <div className="h-[48px] w-[4px] bg-white"></div>
                </div>
            </div>
            <div className="ml-2">
                <Button className="w-full" variant="secondary" onClick={handleZoomIn}>Zoom In</Button>
                <Button className="w-full" variant="secondary" onClick={handleZoomOut}>Zoom Out</Button>
            </div>
        </div>
    );
};
