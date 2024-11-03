import { useParams } from "react-router-dom"

export const SongEditMidiEditorRoute = () => {
    let { uuid } = useParams()

    return (
        <div className="w-full px-8 ml-6">
            <p>song edit - midi editor: {uuid}</p>
        </div>
    )   
}
