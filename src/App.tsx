import {
  ContactShadows,
  Effects,
  Environment,
  Html,
  Lightformer,
  OrbitControls,
} from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import {
  Bloom,
  DepthOfField,
  EffectComposer,
  Noise,
  Vignette,
} from "@react-three/postprocessing";

function App() {
  return (
    <>
      <div id="canvas-container" className="relative">
        <Canvas
          gl={{ logarithmicDepthBuffer: true, antialias: false }}
          dpr={[1, 1.5]}
          camera={{ position: [0, 0, 15], fov: 25 }}
        >
          <color attach="background" args={["#15151a"]} />
          <mesh>
            <sphereGeometry />
            <meshStandardMaterial emissive="red" emissiveIntensity={0.1} />
          </mesh>
          <hemisphereLight intensity={0.5} />
          <ContactShadows
            resolution={1024}
            frames={1}
            position={[0, -1.16, 0]}
            scale={15}
            blur={0.5}
            opacity={1}
            far={20}
          />

          <Environment preset="night" resolution={512} />
          <OrbitControls
            enablePan={false}
            enableZoom={false}
            minPolarAngle={Math.PI / 2.2}
            maxPolarAngle={Math.PI / 2.2}
          />
          <EffectComposer>
            <DepthOfField
              focusDistance={0}
              focalLength={0.02}
              bokehScale={2}
              height={480}
            />
            <Bloom
              luminanceThreshold={0}
              luminanceSmoothing={0.9}
              height={300}
            />
            {/* <Noise opacity={0.02} /> */}
            <Vignette eskil={false} offset={0.1} darkness={1.1} />
          </EffectComposer>
        </Canvas>
      </div>
      <div className="absolute top-0 left-0 flex-1  flex-col justify-center align-middle">
        <span className="text-7xl font-bold text-blue-200 flex ">
          Hi, I'm Pravin!
        </span>
        <span className="text-3xl text-purple-300 flex">
          Click here to start
        </span>
      </div>
    </>
  );
}

export default App;
