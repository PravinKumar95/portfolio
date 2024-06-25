import { createRoot } from "react-dom/client";
import { Canvas } from "@react-three/fiber";
import "./index.css";

function App() {
  return (
    <div id="canvas-container" style={{ width: "100vw", height: "100vh" }}>
      <Canvas resize={{ scroll: true }}>
        <color attach="background" args={["#353535"]} />
      </Canvas>
    </div>
  );
}

createRoot(document.getElementById("root")!).render(<App />);
