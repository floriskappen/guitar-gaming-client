import { useParams } from "react-router-dom"

export const SongEditTimingEditor = () => {
    let { uuid } = useParams()

    return (
        <div className="w-full px-8 ml-6">
            <p>song edit - timing editor: {uuid}</p>
        </div>
    )   
}
