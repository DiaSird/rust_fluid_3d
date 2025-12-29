import { useEffect, useRef, useState } from "react";
import * as THREE from "three";
// import { writeTextFile } from "@tauri-apps/plugin-fs";
import { type OpenDialogOptions, open } from "@tauri-apps/plugin-dialog";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { runSimulation } from "./cmd";

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

  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  const [exportPath, setExportPath] = useState(
    `results/params_${Date.now()}.json`
  );

  const selectExportPath = async () => {
    const defaultPath = `params_${Date.now()}.json`;
    await openPath(defaultPath, {
      setPath(path) {
        setExportPath(path);
      },
      filters: [
        {
          name: "JSON",
          extensions: ["json"],
        },
      ],
    });
  };

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
    const handleResize = () => {
      camera.aspect =
        mountRef.current!.clientWidth / mountRef.current!.clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(
        mountRef.current!.clientWidth,
        mountRef.current!.clientHeight
      );
    };
    window.addEventListener("resize", handleResize);

    // Grid helper
    const grid = new THREE.GridHelper(10, 10);
    grid.position.y = -sizeY / 2;
    scene.add(grid);

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

    return () => {
      window.removeEventListener("resize", handleResize);
      renderer.dispose();
      mountRef.current?.removeChild(renderer.domElement);
    };
  }, [sizeX, sizeY, sizeZ, particlesPerAxis]);

  // Export parameters to JSON file
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
    };
    await runSimulation(config);

    //   // Write JSON file
    //   await writeTextFile(
    //     exportPath,
    //     JSON.stringify(config, null, 2),
    //     {} as any
    //   );
    //   alert(`Parameters exported: ${exportPath}`);
  };

  return (
    <div style={{ display: "flex", height: "100vh" }}>
      {/* GUI panel */}
      <div
        style={{
          width: "250px",
          padding: "10px",
          background: "#222",
          color: "#fff",
        }}
      >
        <h3>Box Parameters</h3>
        <label>
          Size X:
          <input
            type="number"
            value={sizeX}
            onChange={(e) => setSizeX(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Size Y:
          <input
            type="number"
            value={sizeY}
            onChange={(e) => setSizeY(Number(e.target.value))}
          />
        </label>
        <br />
        <label>
          Size Z:
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
          Smooth length:
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
        <br />

        {/* Pre-display export path */}
        <button onClick={selectExportPath}>保存先を選択</button>
        <p style={{ marginTop: "10px", wordBreak: "break-all" }}>
          Export JSON will be saved to: {exportPath}
        </p>

        <button onClick={exportParameters}>計算開始</button>
      </div>

      {/* Three.js canvas */}
      <div style={{ flex: 1 }}>
        <div ref={mountRef} style={{ width: "100%", height: "100%" }} />
      </div>
    </div>
  );
}
