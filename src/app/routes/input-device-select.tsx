import { Button } from "@/components/ui/button"
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import { useState } from "react"
import { MdChevronRight } from "react-icons/md"
import { twMerge } from "tailwind-merge"

export const InputDeviceSelectRoute = () => {
    const [inputDevice, setInputDevice] = useState<string | null>(null)
    const [channels, setChannels] = useState<boolean[]>([true, true])

    const canContinue = !!inputDevice && !!channels.find((v) => v === true)

    return (
        <div className="w-screen h-screen bg-neutral-900 px-10 py-8">
            <div className="w-full flex justify-end">
            <TooltipProvider>
                <Tooltip delayDuration={200}>
                    <TooltipTrigger>
                    <Button variant="outline dashed" aria-label="test" disabled={!canContinue} onClick={() => {}}>
                        continue
                        <MdChevronRight className="w-4 h-4" />
                    </Button>
                    </TooltipTrigger>
                    {
                        !canContinue ?
                            (<TooltipContent side="bottom">
                                <p>to continue pls select an input device <br /> and at least one audio channel</p>
                            </TooltipContent>) : null
                    }
                </Tooltip>
            </TooltipProvider>
            </div>
            <div className="w-full flex justify-center mt-16">
                <div>
                    <div className="flex space-x-4 items-center">
                        <p>input device:</p>
                        <Select onValueChange={(value) => setInputDevice(value)}>
                            <SelectTrigger className="w-[220px]">
                                <SelectValue placeholder="select input device" />
                            </SelectTrigger>
                            <SelectContent>
                                <SelectGroup>
                                    <SelectItem value="apple">Apple</SelectItem>
                                    <SelectItem value="banana">Banana</SelectItem>
                                    <SelectItem value="blueberry">Blueberry</SelectItem>
                                    <SelectItem value="grapes">Grapes</SelectItem>
                                    <SelectItem value="pineapple">Pineapple</SelectItem>
                                </SelectGroup>
                            </SelectContent>
                        </Select>
                    </div>

                    <div className="mt-16 space-y-4">
                        <p>click on the audio channels you wish to use</p>

                        <div className="flex space-x-12 justify-center">
                            {
                                channels.map((_channel, index) => {
                                    return <button className="flex flex-col items-center space-y-2" onClick={() => {
                                        let newChannels = [...channels]
                                        newChannels[index] = !newChannels[index]
                                        setChannels(newChannels)
                                    }}>
                                        <div className={twMerge(
                                            "border-4 w-16 h-[320px] flex flex-col justify-end border-red-500 transition-opacity",
                                            channels[index] === true ? "border-opacity-100" : "border-opacity-20 hover:border-opacity-40"
                                        )}>
                                            <div className="w-[56px] bg-white h-[20%]"></div>
                                        </div>
                                        <p>{index+1}</p>
                                    </button>
                                })
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}
