import { useParameters } from "./providers/parameters/useParameters";
import { BC_OPTIONS, type BC } from "./providers/parameters/types";
import * as React from "react";

/* -----------------------------
 * Small reusable helpers
 * ----------------------------- */

type NumberFieldProps = {
  label: string;
  value: number;
  onChange: (v: number) => void;
};

/**
 * NumberField
 *
 * A numeric input component with internal state.
 * - Displays temporary input while typing.
 * - Validates that value > 0.
 * - Shows inline error if invalid.
 * - Calls `onChange` only when input is valid.
 */
const NumberField: React.FC<NumberFieldProps> = ({ label, value, onChange }) => {
  const [local, setLocal] = React.useState<string>(String(value));
  const [error, setError] = React.useState(false);

  React.useEffect(() => {
    setLocal(String(value));
    setError(value <= 0);
  }, [value]);

  const handleChange = (raw: string) => {
    setLocal(raw);

    const n = Number(raw);

    if (!Number.isFinite(n) || n <= 0) {
      setError(true);
      return;
    }

    setError(false);
    onChange(n);
  };

  return (
    <div style={{ display: "block", marginBottom: "6px", width: "50%" }}>
      <label>
        {label}
        <input
          type="number"
          value={local}
          onChange={(e) => handleChange(e.target.value)}
          style={{
            width: "100%",
            border: error ? "1px solid #ff6060" : undefined,
          }}
        />
      </label>

      {error && <div style={{ color: "#ff8080", fontSize: "0.8em" }}>Value must be greater than 0</div>}
    </div>
  );
};

/* -----------------------------
 * Parameter sections
 * ----------------------------- */

const BoxParameters: React.FC = () => {
  const { state, dispatch } = useParameters();

  return (
    <>
      <h3>Box Parameters</h3>

      <NumberField
        label="Max Particles:"
        value={state.max_n}
        onChange={(v) => dispatch({ type: "SET_MAX_N", value: v })}
      />

      <NumberField
        label="Max Near Particles:"
        value={state.max_near_n}
        onChange={(v) => dispatch({ type: "SET_MAX_NEAR_N", value: v })}
      />

      <NumberField
        label="Size X [m]:"
        value={state.model_scale.length}
        onChange={(v) =>
          dispatch({
            type: "SET_MODEL_SCALE",
            value: { ...state.model_scale, length: v },
          })
        }
      />

      <NumberField
        label="Size Y [m]:"
        value={state.model_scale.width}
        onChange={(v) =>
          dispatch({
            type: "SET_MODEL_SCALE",
            value: { ...state.model_scale, width: v },
          })
        }
      />

      <NumberField
        label="Size Z [m]:"
        value={state.model_scale.height}
        onChange={(v) =>
          dispatch({
            type: "SET_MODEL_SCALE",
            value: { ...state.model_scale, height: v },
          })
        }
      />
    </>
  );
};

const BoundaryConditions: React.FC = () => {
  const { state, dispatch } = useParameters();

  return (
    <>
      <h3>Boundary Conditions</h3>

      <label style={{ display: "block", marginBottom: "6px" }}>
        BC Pattern:
        <select
          value={state.bc_pattern}
          onChange={(e) =>
            dispatch({
              type: "SET_BC_PATTERN",
              value: e.target.value as BC,
            })
          }
          style={{ width: "100%" }}
        >
          {BC_OPTIONS.map((bc) => (
            <option key={bc} value={bc}>
              {bc}
            </option>
          ))}
        </select>
      </label>

      <NumberField
        label="U_lid [m/s]:"
        value={state.u_lid}
        onChange={(v) => dispatch({ type: "SET_U_LID", value: v })}
      />
    </>
  );
};

const SPHParameters: React.FC = () => {
  const { state, dispatch } = useParameters();

  return (
    <>
      <h3>SPH Parameters</h3>

      <NumberField
        label="Smooth length [m]:"
        value={state.smooth_length}
        onChange={(v) =>
          dispatch({
            type: "SET_SPH_PARAMS",
            value: { smooth_length: v },
          })
        }
      />

      <NumberField
        label="Cell size:"
        value={state.cell_scale}
        onChange={(v) =>
          dispatch({
            type: "SET_SPH_PARAMS",
            value: { cell_scale: v },
          })
        }
      />

      <NumberField
        label="Beta:"
        value={state.beta}
        onChange={(v) =>
          dispatch({
            type: "SET_SPH_PARAMS",
            value: { beta: v },
          })
        }
      />

      <NumberField
        label="CS_RATE:"
        value={state.cs_rate}
        onChange={(v) =>
          dispatch({
            type: "SET_SPH_PARAMS",
            value: { cs_rate: v },
          })
        }
      />
    </>
  );
};

const ResolutionParameters: React.FC = () => {
  const { state, dispatch } = useParameters();

  return (
    <>
      <h3>Resolution [m]</h3>

      <NumberField
        label="DX:"
        value={state.dx.dx}
        onChange={(v) =>
          dispatch({
            type: "SET_DX",
            value: { ...state.dx, dx: v },
          })
        }
      />

      <NumberField
        label="DY:"
        value={state.dx.dy}
        onChange={(v) =>
          dispatch({
            type: "SET_DX",
            value: { ...state.dx, dy: v },
          })
        }
      />

      <NumberField
        label="DZ:"
        value={state.dx.dz}
        onChange={(v) =>
          dispatch({
            type: "SET_DX",
            value: { ...state.dx, dz: v },
          })
        }
      />
    </>
  );
};

const SimulationParameters: React.FC = () => {
  const { state, dispatch } = useParameters();

  return (
    <>
      <h3>Simulation</h3>

      <NumberField
        label="Dt [s]:"
        value={state.dt}
        onChange={(v) =>
          dispatch({
            type: "SET_TIME_STEPPING",
            value: { dt: v },
          })
        }
      />

      <NumberField
        label="Step to display:"
        value={state.out_step}
        onChange={(v) =>
          dispatch({
            type: "SET_TIME_STEPPING",
            value: { out_step: v },
          })
        }
      />

      <NumberField
        label="Max step:"
        value={state.max_step}
        onChange={(v) =>
          dispatch({
            type: "SET_TIME_STEPPING",
            value: { max_step: v },
          })
        }
      />
    </>
  );
};

const OptionalParameters: React.FC = () => {
  const { state, dispatch } = useParameters();

  return (
    <>
      <h3>Optional</h3>

      <label style={{ display: "block", marginBottom: "6px", width: "50%" }}>
        Restart file:
        <input
          type="text"
          value={state.restart_file ?? ""}
          onChange={(e) =>
            dispatch({
              type: "SET_RESTART_FILE",
              value: e.target.value,
            })
          }
        />
      </label>

      <NumberField
        label="Monitor particle index:"
        value={state.monitor_particle}
        onChange={(v) => dispatch({ type: "SET_MONITOR_PARTICLE", value: v })}
      />
    </>
  );
};

/* -----------------------------
 * Panel root
 * ----------------------------- */

export const ParameterPanel: React.FC = () => {
  return (
    <div style={panelStyle}>
      <BoxParameters />
      <BoundaryConditions />
      <SPHParameters />
      <ResolutionParameters />
      <SimulationParameters />
      <OptionalParameters />
    </div>
  );
};

/* -----------------------------
 * Styles
 * ----------------------------- */

const panelStyle: React.CSSProperties = {
  padding: "10px",
  background: "#41403e88",
  color: "#fff",
  overflowY: "auto",
};
