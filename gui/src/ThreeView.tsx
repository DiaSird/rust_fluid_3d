import { useEffect, useRef, useState } from "react";
import * as THREE from "three";
import { type OpenDialogOptions, open } from "@tauri-apps/plugin-dialog";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { runSimulation } from "./cmd";
import { listen } from "@tauri-apps/api/event";

type OpenOptions = {
  /**
   * path setter.
   * - If we don't get the result within this function, somehow the previous value comes in.(React component)
   * @param path
   * @returns
   */
  setPath?: (path: string) => void;
} & OpenDialogOptions;

/**
 * Open a file or Dir
 * @returns selected path or cancelled null
 * @throws
 */
export async function openPath(
  path: string,
  options: OpenOptions = {}
): Promise<string | string[] | null> {
  const { setPath, ...dialogOptions } = options;

  const res = await (async () => {
    return await open({ defaultPath: path, ...dialogOptions });
  })();

  typeof res === "string" && setPath?.(res);
  return res;
}

export default function ThreeView() {
  const mountRef = useRef<HTMLDivElement>(null);

  type Tab = "params" | "log";
  const [activeTab, setActiveTab] = useState<Tab>("params");

  const [logs, setLogs] = useState<string[]>([]);

  // Box size and particle count
  const [sizeX, setSizeX] = useState(5);
  const [sizeY, setSizeY] = useState(5);
  const [sizeZ, setSizeZ] = useState(5);
  const [particlesPerAxis, setParticlesPerAxis] = useState(15);

  // SPH parameters
  const [smoothLength, setSmoothLength] = useState(0.0324);
  const [cellSize, setCellSize] = useState(0.0648);
  const [beta, setBeta] = useState(0.3);
  const [csRate, setCsRate] = useState(0.05);
  const [dx, setDx] = useState(0.027);
  const [dy, setDy] = useState(0.027);
  const [dz, setDz] = useState(0.027);

  // Simulation
  const [dt, setDt] = useState(0.00001);
  const [outStep, setOutStep] = useState(10);
  const [maxStep, setMaxStep] = useState(100);

  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const [exportPath, _setExportPath] = useState(
    `results/params_${Date.now()}.json`
  );

  // Tab Header
  const renderTabHeader = () => (
    <div
      style={{
        display: "flex",
        background: "#111",
        borderBottom: "1px solid #333",
      }}
    >
      <button
        onClick={() => setActiveTab("params")}
        style={{
          padding: "8px 16px",
          background: activeTab === "params" ? "#333" : "transparent",
          color: "#fff",
          border: "none",
          cursor: "pointer",
        }}
      >
        Parameters
      </button>

      <button
        onClick={() => setActiveTab("log")}
        style={{
          padding: "8px 16px",
          background: activeTab === "log" ? "#333" : "transparent",
          color: "#fff",
          border: "none",
          cursor: "pointer",
        }}
      >
        Log ({logs.length})
      </button>
    </div>
  );

  // GUI Render
  const renderParams = () => (
    <div style={{ display: "flex", height: "100%" }}>
      {/* Run Simulation Button */}
      <div style={{ position: "absolute", top: 10, left: 10, zIndex: 100 }}>
        <button
          onClick={exportParameters}
          style={{
            padding: "8px 16px",
            background: "#0a0",
            color: "#fff",
            border: "none",
            borderRadius: "4px",
            cursor: "pointer",
          }}
        >
          Run
        </button>
      </div>

      {/* GUI panel */}
      <div
        style={{
          width: "250px",
          padding: "10px",
          background: "#222",
          color: "#fff",
          overflowY: "auto",
        }}
      >
        <h3>Box Parameters</h3>
        <label>
          Size X [m]:
          <input
            type="number"
            value={sizeX}
            onChange={(e) => setSizeX(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Size Y [m]:
          <input
            type="number"
            value={sizeY}
            onChange={(e) => setSizeY(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Size Z [m]:
          <input
            type="number"
            value={sizeZ}
            onChange={(e) => setSizeZ(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Particles per axis:
          <input
            type="number"
            value={particlesPerAxis}
            onChange={(e) => {
              const value = Number(e.target.value);

              if (value > 500) {
                setErrorMessage("Particle count per axis must be 500 or less.");
                return;
              }

              setErrorMessage(null);
              setParticlesPerAxis(value);
            }}
          />
        </label>

        {errorMessage && (
          <p style={{ color: "red", marginTop: "8px" }}>{errorMessage}</p>
        )}

        <h3>SPH Parameters</h3>
        <label>
          Smooth length [m]:
          <input
            type="number"
            value={smoothLength}
            onChange={(e) => setSmoothLength(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Cell size:
          <input
            type="number"
            value={cellSize}
            onChange={(e) => setCellSize(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Beta:
          <input
            type="number"
            value={beta}
            onChange={(e) => setBeta(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          CS_RATE:
          <input
            type="number"
            value={csRate}
            onChange={(e) => setCsRate(Number(e.target.value))}
          />
        </label>
        <br />

        <h3>Resolution [m]</h3>
        <label>
          DX:
          <input
            type="number"
            value={dx}
            onChange={(e) => setDx(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          DY:
          <input
            type="number"
            value={dy}
            onChange={(e) => setDy(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          DZ:
          <input
            type="number"
            value={dz}
            onChange={(e) => setDz(Number(e.target.value))}
          />
        </label>
        <br />

        <h3>Simulation</h3>
        <label>
          Dt [s]:
          <input
            type="number"
            value={dt}
            onChange={(e) => setDt(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          step to display:
          <input
            type="number"
            value={outStep}
            onChange={(e) => setOutStep(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          max step:
          <input
            type="number"
            value={maxStep}
            onChange={(e) => setMaxStep(Number(e.target.value))}
          />
        </label>
        <br />
        <br />

        {/* <button onClick={exportParameters}>計算開始</button>
        <button onClick={exportParameters}>Run</button> */}
      </div>
    </div>
  );

  // Log Panel
  const renderLog = () => (
    <div
      style={{
        background: "#111",
        color: "#0f0",
        padding: "8px",
        height: "100%",
        overflowY: "auto",
        fontFamily: "monospace",
        fontSize: "12px",
      }}
    >
      {logs.map((log, i) => (
        <div key={i} style={{ whiteSpace: "pre-wrap" }}>
          {log}
        </div>
      ))}
    </div>
  );

  useEffect(() => {
    const unlistenPromise = listen<string>("simulation-log", (event) => {
      setLogs((prev) => [...prev, event.payload]);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    if (!mountRef.current) return;

    // Scene setup
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x111111);

    // Camera setup
    const camera = new THREE.PerspectiveCamera(
      60,
      mountRef.current.clientWidth / mountRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.set(10, 10, 10);
    camera.lookAt(0, 0, 0);

    // Renderer setup
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(
      mountRef.current.clientWidth,
      mountRef.current.clientHeight
    );
    mountRef.current.innerHTML = "";
    mountRef.current.appendChild(renderer.domElement);

    // Controls setup
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;

    // Resize handling
    // const handleResize = () => {
    //   camera.aspect =
    //     mountRef.current!.clientWidth / mountRef.current!.clientHeight;
    //   camera.updateProjectionMatrix();
    //   renderer.setSize(
    //     mountRef.current!.clientWidth,
    //     mountRef.current!.clientHeight
    //   );
    // };
    // window.addEventListener("resize", handleResize);
    // Resize handling with particle & grid scaling
    const ro = new ResizeObserver(() => {
      const width = mountRef.current!.clientWidth;
      const height = mountRef.current!.clientHeight;

      // Update camera
      camera.aspect = width / height;
      camera.updateProjectionMatrix();

      // Update renderer
      renderer.setSize(width, height);

      // Update particle scale based on container size
      const scaleX = width / mountRef.current!.offsetWidth;
      const scaleY = height / mountRef.current!.offsetHeight;
      particles.scale.set(scaleX, scaleY, scaleX); // scale-z = scale-x
      particles.material.needsUpdate = true;

      // Update grid size and position
      scene.remove(grid);
      const newGrid = new THREE.GridHelper(sizeX, Math.ceil(sizeX));
      newGrid.position.y = -sizeY / 2;
      scene.add(newGrid);

      // Keep reference for cleanup
      grid = newGrid;
    });
    ro.observe(mountRef.current!);

    // Grid helper
    let grid = new THREE.GridHelper(10, 10);
    grid.position.y = -sizeY / 2;
    scene.add(grid);

    // const grid = new THREE.GridHelper(10, 10);
    // grid.position.y = -sizeY / 2;
    // scene.add(grid);

    // Function to generate particle geometry
    const generateParticles = () => {
      const geometry = new THREE.BufferGeometry();
      const total = particlesPerAxis ** 3;
      const positions = new Float32Array(total * 3);
      let idx = 0;

      for (let i = 0; i < particlesPerAxis; i++) {
        for (let j = 0; j < particlesPerAxis; j++) {
          for (let k = 0; k < particlesPerAxis; k++) {
            positions[idx++] = (i / (particlesPerAxis - 1) - 0.5) * sizeX;
            positions[idx++] = (j / (particlesPerAxis - 1) - 0.5) * sizeY;
            positions[idx++] = (k / (particlesPerAxis - 1) - 0.5) * sizeZ;
          }
        }
      }
      geometry.setAttribute(
        "position",
        new THREE.BufferAttribute(positions, 3)
      );
      return geometry;
    };

    const material = new THREE.PointsMaterial({ color: 0x44aaff, size: 0.05 });
    let particles = new THREE.Points(generateParticles(), material);
    scene.add(particles);

    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate);
      controls.update();
      particles.rotation.y += 0.001;
      renderer.render(scene, camera);
    };
    animate();

    // return () => {
    //   window.removeEventListener("resize", handleResize);
    //   renderer.dispose();
    //   mountRef.current?.removeChild(renderer.domElement);
    // };
    return () => {
      ro.disconnect();
      renderer.dispose();
      mountRef.current?.removeChild(renderer.domElement);
    };
  }, [sizeX, sizeY, sizeZ, particlesPerAxis]);

  // Export parameters to JSON file and Run simulation
  const exportParameters = async () => {
    if (!exportPath) {
      alert("保存先パスを選択してください！");
      return;
    }

    const config = {
      length: sizeX,
      width: sizeY,
      height: sizeZ,

      n_axis: particlesPerAxis,
      smooth_length: smoothLength,
      cell_size: cellSize,
      beta: beta,
      cs_rate: csRate,

      dx: dx,
      dy: dy,
      dz: dz,

      dt: dt,
      out_step: outStep,
      max_step: maxStep,
    };
    await runSimulation(config);

    //   // Write JSON State file
    //   await writeTextFile(
    //     exportPath,
    //     JSON.stringify(config, null, 2),
    //     {} as any
    //   );
    //   alert(`Parameters exported: ${exportPath}`);
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      {renderTabHeader()}

      <div style={{ flex: 1, position: "relative" }}>
        {/* GUI panel */}
        {activeTab === "params" && (
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              width: "250px",
              height: "100%",
              padding: "10px",
              background: "#222",
              color: "#fff",
              overflowY: "auto",
              zIndex: 1,
            }}
          >
            {renderParams()}
          </div>
        )}

        {/* Three.js canvas */}
        <div ref={mountRef} style={{ width: "100%", height: "100%" }} />

        {/* Log panel */}
        {activeTab === "log" && (
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              right: 0,
              bottom: 0,
              zIndex: 2,
            }}
          >
            {renderLog()}
          </div>
        )}
      </div>
    </div>
  );
}
