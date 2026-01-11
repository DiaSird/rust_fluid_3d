import { useState, type ReactNode } from "react";
import { ParameterContext } from "./ParameterContext";
import type { BC, ModelScale, Resolution } from "./types";

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
  const [restart_file, setRestartFile] = useState<string>("sim_checkpoint.bin");
  const [monitor_particle, setMonitorParticle] = useState(0);

  const [log, setLog] = useState([""]);

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
