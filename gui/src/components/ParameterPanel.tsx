import { useState } from "react";
import { useParameters } from "./ParameterContext";

interface Props {
  exportParameters: () => void;
}

export const ParameterPanel: React.FC<Props> = ({ exportParameters }) => {
  const {
    // Model scale
    model_scale,
    setModelScale,

    // Particle counts
    max_n,
    setMaxN,
    max_near_n,
    setMaxNearN,

    // Boundary conditions
    bc_pattern,
    setBCPattern,
    u_lid,
    setULid,

    // SPH parameters
    smooth_length,
    setSmoothLength,
    cell_scale,
    setCellScale,
    beta,
    setBeta,
    cs_rate,
    setCsRate,

    // Resolution
    dx,
    setDx,

    // Time stepping
    dt,
    setDt,
    out_step,
    setOutStep,
    max_step,
    setMaxStep,

    // Optional
    restart_file,
    setRestartFile,
    monitor_particle,
    setMonitorParticle,
  } = useParameters();

  const [errorMessage] = useState<string | null>(null);

  return (
    <div
      style={{
        width: "250px",
        padding: "10px",
        background: "#222",
        color: "#fff",
        overflowY: "auto",
      }}
    >
      {/* Run simulation button */}
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

      {/* Box parameters */}
      <h3>Box Parameters</h3>
      <label>
        Max Particles:
        <input
          type="number"
          value={max_n}
          onChange={(e) => setMaxN(Number(e.target.value))}
        />
      </label>
      <br />
      <label>
        Max Near Particles:
        <input
          type="number"
          value={max_near_n}
          onChange={(e) => setMaxNearN(Number(e.target.value))}
        />
      </label>
      <br />

      <label>
        Size X [m]:
        <input
          type="number"
          value={model_scale.length}
          onChange={(e) =>
            setModelScale({ ...model_scale, length: Number(e.target.value) })
          }
        />
      </label>
      <br />
      <label>
        Size Y [m]:
        <input
          type="number"
          value={model_scale.width}
          onChange={(e) =>
            setModelScale({ ...model_scale, width: Number(e.target.value) })
          }
        />
      </label>
      <br />
      <label>
        Size Z [m]:
        <input
          type="number"
          value={model_scale.height}
          onChange={(e) =>
            setModelScale({ ...model_scale, height: Number(e.target.value) })
          }
        />
      </label>
      <br />
      {errorMessage && (
        <p style={{ color: "red", marginTop: "8px" }}>{errorMessage}</p>
      )}

      {/* Boundary conditions */}
      <h3>Boundary Conditions</h3>
      <label>
        BC Pattern:
        <select
          value={bc_pattern}
          onChange={(e) => setBCPattern(e.target.value as any)}
        >
          <option value="Cavity-Flow">Cavity-Flow</option>
          <option value="Poiseuille-Flow">Poiseuille-Flow</option>
          <option value="Periodic-Flow">Periodic-Flow</option>
          <option value="LidDrivenCavity">LidDrivenCavity</option>
        </select>
      </label>
      <br />
      <label>
        U_lid [m/s]:
        <input
          type="number"
          value={u_lid}
          onChange={(e) => setULid(Number(e.target.value))}
        />
      </label>
      <br />

      {/* SPH parameters */}
      <h3>SPH Parameters</h3>
      <label>
        Smooth length [m]:
        <input
          type="number"
          value={smooth_length}
          onChange={(e) => setSmoothLength(Number(e.target.value))}
        />
      </label>
      <br />
      <label>
        Cell size:
        <input
          type="number"
          value={cell_scale}
          onChange={(e) => setCellScale(Number(e.target.value))}
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
          value={cs_rate}
          onChange={(e) => setCsRate(Number(e.target.value))}
        />
      </label>
      <br />

      {/* Resolution */}
      <h3>Resolution [m]</h3>
      <label>
        DX:
        <input
          type="number"
          value={dx.dx}
          onChange={(e) => setDx({ ...dx, dx: Number(e.target.value) })}
        />
      </label>
      <br />
      <label>
        DY:
        <input
          type="number"
          value={dx.dy}
          onChange={(e) => setDx({ ...dx, dy: Number(e.target.value) })}
        />
      </label>
      <br />
      <label>
        DZ:
        <input
          type="number"
          value={dx.dz}
          onChange={(e) => setDx({ ...dx, dz: Number(e.target.value) })}
        />
      </label>
      <br />

      {/* Time stepping */}
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
        Step to display:
        <input
          type="number"
          value={out_step}
          onChange={(e) => setOutStep(Number(e.target.value))}
        />
      </label>
      <br />
      <label>
        Max step:
        <input
          type="number"
          value={max_step}
          onChange={(e) => setMaxStep(Number(e.target.value))}
        />
      </label>
      <br />

      {/* Optional parameters */}
      <h3>Optional</h3>
      <label>
        Restart file:
        <input
          type="text"
          value={restart_file}
          onChange={(e) => setRestartFile(e.target.value)}
        />
      </label>
      <br />
      <label>
        Monitor particle index:
        <input
          type="number"
          value={monitor_particle}
          onChange={(e) => setMonitorParticle(Number(e.target.value))}
        />
      </label>
    </div>
  );
};
