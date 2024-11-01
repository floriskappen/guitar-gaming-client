import { useParams } from "react-router-dom"
import { Layout } from "@/features/song-edit/components/layout"

export const SongEditTimingEditor = () => {
    let { uuid } = useParams()

    return (
        <Layout>
            <div className="w-full px-8 ml-6">
                <p>song edit - timing editor: {uuid}</p>
            </div>
        </Layout>
    )   
}
