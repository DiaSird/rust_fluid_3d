import { useParameters } from "./providers/parameters/ParameterContext";
import { BC_OPTIONS, type BC } from "./providers/parameters/types";

/* -----------------------------
 * Small reusable helpers
 * ----------------------------- */

type NumberFieldProps = {
  label: string;
  value: number;
  onChange: (v: number) => void;
};

const NumberField: React.FC<NumberFieldProps> = ({ label, value, onChange }) => (
  <label style={{ display: "block", marginBottom: "6px", width: "50%" }}>
    {label}
    <input type="number" value={value} onChange={(e) => onChange(Number(e.target.value))} style={{ width: "100%" }} />
  </label>
);

function updateObject<T extends object>(value: T, setValue: React.Dispatch<React.SetStateAction<T>>) {
  return <K extends keyof T>(key: K, v: T[K]) => setValue({ ...value, [key]: v });
}

/* -----------------------------
 * Parameter sections
 * ----------------------------- */

const BoxParameters: React.FC = () => {
  const { max_n, setMaxN, max_near_n, setMaxNearN, model_scale, setModelScale } = useParameters();

  const setScale = updateObject(model_scale, setModelScale);

  return (
    <>
      <h3>Box Parameters</h3>

      <NumberField label="Max Particles:" value={max_n} onChange={setMaxN} />
      <NumberField label="Max Near Particles:" value={max_near_n} onChange={setMaxNearN} />

      <NumberField label="Size X [m]:" value={model_scale.length} onChange={(v) => setScale("length", v)} />
      <NumberField label="Size Y [m]:" value={model_scale.width} onChange={(v) => setScale("width", v)} />
      <NumberField label="Size Z [m]:" value={model_scale.height} onChange={(v) => setScale("height", v)} />
    </>
  );
};

const BoundaryConditions: React.FC = () => {
  const { bc_pattern, setBCPattern, u_lid, setULid } = useParameters();

  return (
    <>
      <h3>Boundary Conditions</h3>

      <label style={{ display: "block", marginBottom: "6px" }}>
        BC Pattern:
        <select value={bc_pattern} onChange={(e) => setBCPattern(e.target.value as BC)} style={{ width: "100%" }}>
          {BC_OPTIONS.map((bc) => (
            <option key={bc} value={bc}>
              {bc}
            </option>
          ))}
        </select>
      </label>

      <NumberField label="U_lid [m/s]:" value={u_lid} onChange={setULid} />
    </>
  );
};

const SPHParameters: React.FC = () => {
  const { smooth_length, setSmoothLength, cell_scale, setCellScale, beta, setBeta, cs_rate, setCsRate } =
    useParameters();

  return (
    <>
      <h3>SPH Parameters</h3>

      <NumberField label="Smooth length [m]:" value={smooth_length} onChange={setSmoothLength} />
      <NumberField label="Cell size:" value={cell_scale} onChange={setCellScale} />
      <NumberField label="Beta:" value={beta} onChange={setBeta} />
      <NumberField label="CS_RATE:" value={cs_rate} onChange={setCsRate} />
    </>
  );
};

const ResolutionParameters: React.FC = () => {
  const { dx, setDx } = useParameters();
  const set = updateObject(dx, setDx);

  return (
    <>
      <h3>Resolution [m]</h3>

      <NumberField label="DX:" value={dx.dx} onChange={(v) => set("dx", v)} />
      <NumberField label="DY:" value={dx.dy} onChange={(v) => set("dy", v)} />
      <NumberField label="DZ:" value={dx.dz} onChange={(v) => set("dz", v)} />
    </>
  );
};

const SimulationParameters: React.FC = () => {
  const { dt, setDt, out_step, setOutStep, max_step, setMaxStep } = useParameters();

  return (
    <>
      <h3>Simulation</h3>

      <NumberField label="Dt [s]:" value={dt} onChange={setDt} />
      <NumberField label="Step to display:" value={out_step} onChange={setOutStep} />
      <NumberField label="Max step:" value={max_step} onChange={setMaxStep} />
    </>
  );
};

const OptionalParameters: React.FC = () => {
  const { restart_file, setRestartFile, monitor_particle, setMonitorParticle } = useParameters();

  return (
    <>
      <h3>Optional</h3>

      <label style={{ display: "block", marginBottom: "6px", width: "50%" }}>
        Restart file:
        <input type="text" value={restart_file} onChange={(e) => setRestartFile(e.target.value)} />
      </label>

      <NumberField label="Monitor particle index:" value={monitor_particle} onChange={setMonitorParticle} />
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
