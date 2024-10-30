import { useParams } from "react-router-dom"
import { Canvas } from '@react-three/fiber'

export const SongPlayRoute = () => {
    let { uuid } = useParams()

    return (
        <div>
            <Canvas>
                <ambientLight intensity={0.1} />
                <directionalLight color="red" position={[0, 0, 5]} />
                <mesh>
                    <boxGeometry args={[2, 2, 2]} />
                    <meshStandardMaterial />
                </mesh>
            </Canvas>
        </div>
    )
}
