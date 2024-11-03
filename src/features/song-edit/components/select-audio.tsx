import { Drawer, DrawerContent, DrawerDescription, DrawerHeader, DrawerTitle } from "@/components/ui/drawer"
import { useCallback, useState } from "react"
import { FileRejection, useDropzone } from 'react-dropzone'
import { BiLoaderAlt } from "react-icons/bi"
import { twMerge } from "tailwind-merge"

export const SelectAudio = ({ children, onSelect, open, setOpen, loading }: { children: React.ReactNode, onSelect: (file: File) => void, open: boolean, setOpen: (value: boolean) => void, loading?: boolean}) => {
    const [wrongFile, setWrongFile] = useState(false)

    const onDrop = useCallback((acceptedFiles: any[], fileRejections: FileRejection[]) => {
        // Do something with the files
        if (fileRejections.length > 0) {
            setWrongFile(true)
        } else {
            const file = acceptedFiles[0]
            onSelect(file)
        }
    }, [])

    const { getRootProps, getInputProps, isDragActive } = useDropzone({
        onDrop,
        accept: {
            "audio/mpeg": []
        },
        maxFiles: 1
    })

    return (
        <Drawer open={open} onClose={() => {
            setOpen(false)
            setWrongFile(false)
        }}>
            <div>
                {children}
            </div>
            <DrawerContent>
                <DrawerHeader>
                    <DrawerTitle className="w-full text-center">
                        select an audio file
                    </DrawerTitle>

                    <div className="w-full flex items-center justify-center px-4 py-4">
                        <div className="h-[96px]">
                            {
                                loading ? (
                                    <div className="h-full flex items-center justify-center">
                                        <BiLoaderAlt className="animate-spin" />
                                    </div>
                                ) : (
                                    <div {...getRootProps({ className: 'dropzone' })} className={twMerge(
                                        "w-[600px] border-4 h-full border-dashed flex justify-center px-4 py-8",
                                        isDragActive ? "border-neutral-100" : "border-neutral-600"
                                    )}>
                                        <input {...getInputProps()} />
                                        <p>drop an mp3 file, or click here to browse</p>
                                    </div>
                                )
                            }
                        </div>
                    </div>

                    <p className={twMerge(
                        "w-full text-center text-red-600",
                        wrongFile ? "visible" : "invisible"
                    )}>please only select a single mp3 file</p>
                </DrawerHeader>
            </DrawerContent>
        </Drawer>
    )
}
