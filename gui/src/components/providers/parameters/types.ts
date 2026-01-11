import type { Dispatch, SetStateAction } from "react";

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

/** Boundary Condition */
export const BC_OPTIONS = ["Cavity-Flow", "Poiseuille-Flow", "Periodic-Flow", "LidDrivenCavity"] as const;

export type BC = (typeof BC_OPTIONS)[number];

export interface Config {
  // Max particles
  max_n: number;
  max_near_n: number;

  // Model size
  model_scale: ModelScale;

  // Boundary condition
  bc_pattern: BC;
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

export interface ParameterContextProps {
  max_n: number;
  setMaxN: Dispatch<SetStateAction<number>>;

  max_near_n: number;
  setMaxNearN: Dispatch<SetStateAction<number>>;

  model_scale: ModelScale;
  setModelScale: Dispatch<SetStateAction<ModelScale>>;

  bc_pattern: BC;
  setBCPattern: Dispatch<SetStateAction<BC>>;

  u_lid: number;
  setULid: Dispatch<SetStateAction<number>>;

  smooth_length: number;
  setSmoothLength: Dispatch<SetStateAction<number>>;

  cell_scale: number;
  setCellScale: Dispatch<SetStateAction<number>>;

  beta: number;
  setBeta: Dispatch<SetStateAction<number>>;

  cs_rate: number;
  setCsRate: Dispatch<SetStateAction<number>>;

  dx: Resolution;
  setDx: Dispatch<SetStateAction<Resolution>>;

  dt: number;
  setDt: Dispatch<SetStateAction<number>>;

  out_step: number;
  setOutStep: Dispatch<SetStateAction<number>>;

  max_step: number;
  setMaxStep: Dispatch<SetStateAction<number>>;

  restart_file: string;
  setRestartFile: Dispatch<SetStateAction<string>>;

  monitor_particle: number;
  setMonitorParticle: Dispatch<SetStateAction<number>>;

  log: string[];
  setLog: Dispatch<SetStateAction<string[]>>;
}
