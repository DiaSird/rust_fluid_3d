import { createContext, useContext, useState, type ReactNode } from "react";

export interface ModelScale {
  length: number;
  width: number;
  height: number;
}

export interface Resolution {
  dx: number;
  dy: number;
  dz: number;
}

export interface Config {
  // Max particles
  max_n: number;
  max_near_n: number;

  // Model size
  model_scale: ModelScale;

  // Boundary condition
  bc_pattern: "Cavity-Flow" | "Poiseuille-Flow" | "Periodic-Flow" | "LidDrivenCavity";
  u_lid: number;

  // SPH parameters
  smooth_length: number;
  cell_scale: number;
  beta: number;
  cs_rate: number;

  // Resolution
  dx: Resolution;

  // Time stepping
  dt: number;
  out_step: number;
  max_step: number;

  // Checkpoint / Monitoring
  restart_file?: string; // Option<PathBuf>
  monitor_particle: number;
}

type BC = "Cavity-Flow" | "Poiseuille-Flow" | "Periodic-Flow" | "LidDrivenCavity";

interface ParameterContextProps {
  max_n: number;
  setMaxN: (v: number) => void;
  max_near_n: number;
  setMaxNearN: (v: number) => void;

  model_scale: ModelScale;
  setModelScale: (v: ModelScale) => void;

  bc_pattern: BC;
  setBCPattern: (v: BC) => void;
  u_lid: number;
  setULid: (v: number) => void;

  smooth_length: number;
  setSmoothLength: (v: number) => void;
  cell_scale: number;
  setCellScale: (v: number) => void;
  beta: number;
  setBeta: (v: number) => void;
  cs_rate: number;
  setCsRate: (v: number) => void;

  dx: Resolution;
  setDx: (v: Resolution) => void;

  dt: number;
  setDt: (v: number) => void;
  out_step: number;
  setOutStep: (v: number) => void;
  max_step: number;
  setMaxStep: (v: number) => void;
  restart_file?: string;

  setRestartFile: (v: string) => void;
  monitor_particle: number;
  setMonitorParticle: (v: number) => void;

  log: string;
  setLog: (v: string) => void;
}

export const ParameterContext = createContext<ParameterContextProps | undefined>(undefined);

export const useParameters = (): ParameterContextProps => {
  const context = useContext(ParameterContext);
  if (!context) throw new Error("useParameters must be used within a ParameterProvider");
  return context;
};

export const ParameterProvider = ({ children }: { children: ReactNode }) => {
  // Max particles
  const [max_n, setMaxN] = useState(60000);
  const [max_near_n, setMaxNearN] = useState(100);

  // Model scale
  const [model_scale, setModelScale] = useState<ModelScale>({
    length: 0.5,
    width: 0.5,
    height: 0.5,
  });

  // Boundary condition
  const [bc_pattern, setBCPattern] = useState<BC>("Cavity-Flow");
  const [u_lid, setULid] = useState(5.0);

  // SPH parameters
  const [smooth_length, setSmoothLength] = useState(0.0324);
  const [cell_scale, setCellScale] = useState(2.0);
  const [beta, setBeta] = useState(0.3);
  const [cs_rate, setCsRate] = useState(0.05);

  // Resolution
  const [dx, setDx] = useState<Resolution>({ dx: 0.027, dy: 0.027, dz: 0.027 });

  // Time stepping
  const [dt, setDt] = useState(0.001);
  const [out_step, setOutStep] = useState(10);
  const [max_step, setMaxStep] = useState(1000);

  // Checkpoint / monitoring
  const [restart_file, setRestartFile] = useState<string | undefined>("sim_checkpoint.bin");
  const [monitor_particle, setMonitorParticle] = useState(0);

  const [log, setLog] = useState("");

  return (
    <ParameterContext.Provider
      value={{
        max_n,
        setMaxN,
        max_near_n,
        setMaxNearN,
        model_scale,
        setModelScale,

        bc_pattern,
        setBCPattern,
        u_lid,
        setULid,

        smooth_length,
        setSmoothLength,
        cell_scale,
        setCellScale,
        beta,
        setBeta,
        cs_rate,
        setCsRate,

        dx,
        setDx,

        dt,
        setDt,
        out_step,
        setOutStep,
        max_step,
        setMaxStep,

        restart_file,
        setRestartFile,
        monitor_particle,
        setMonitorParticle,

        log,
        setLog,
      }}
    >
      {children}
    </ParameterContext.Provider>
  );
};
