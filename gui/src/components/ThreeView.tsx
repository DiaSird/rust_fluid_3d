import { useEffect, useState } from "react";
import { ParameterPanel } from "./ParameterPanel";
import { ThreeCanvas } from "./ThreeCanvas";
import { LogPanel } from "./LogPanel";
import { useParameters, type Config } from "./ParameterContext";
import { runSimulation } from "../cmd";
import { listen } from "@tauri-apps/api/event";

export default function ThreeView() {
  const [activeTab, setActiveTab] = useState<"params" | "log">("params");
  const params = useParameters();

  useEffect(() => {
    const unlistenPromise = listen<string>("terra://simulation-log", (event) => {
      console.log(JSON.stringify(event.payload));
      params.setLog((prev) => [...prev, JSON.stringify(event.payload)]);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [params]);

  // Export parameters and run simulation
  const exportParameters = async () => {
    const config: Config = {
      // Model scale
      model_scale: {
        length: params.model_scale.length,
        width: params.model_scale.width,
        height: params.model_scale.height,
      },

      // Particle count
      max_n: params.max_n,
      max_near_n: params.max_near_n,

      // Boundary condition
      bc_pattern: params.bc_pattern,
      u_lid: params.u_lid,

      // SPH parameters
      smooth_length: params.smooth_length,
      cell_scale: params.cell_scale,
      beta: params.beta,
      cs_rate: params.cs_rate,

      // Resolution
      dx: { ...params.dx },

      // Time stepping
      dt: params.dt,
      out_step: params.out_step,
      max_step: params.max_step,

      // Restart file and monitoring
      restart_file: params.restart_file ?? "results/sim_checkpoint.bin",
      monitor_particle: params.monitor_particle,
    };

    await runSimulation(config);
  };

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      {/* Tab header */}
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
          Log
        </button>
      </div>

      <div style={{ flex: 1, position: "relative" }}>
        {/* Render parameter panel when activeTab is 'params' */}
        {activeTab === "params" && (
          <div
            style={{
              position: "absolute",
              top: 0,
              left: 0,
              height: "100%",
              zIndex: 1,
            }}
          >
            <ParameterPanel exportParameters={exportParameters} />
          </div>
        )}

        {/* Three.js canvas */}
        <ThreeCanvas />

        {/* Render log panel when activeTab is 'log' */}
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
            <LogPanel />
          </div>
        )}
      </div>
    </div>
  );
}
