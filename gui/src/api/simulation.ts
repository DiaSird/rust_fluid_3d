import { invoke } from "@tauri-apps/api/core";
import {
  type Config,
  type Matrix3,
  type Vector3,
} from "../components/providers/parameters/types";

export const runSimulation = async (config: Config) => {
  try {
    await invoke("run_simulation", { config });
  } catch (err) {
    console.error(err);
  }
};

/** Fluid **/
export const FLUID_OPTIONS = ["Water", "Air"] as const;
export type FLUID = (typeof FLUID_OPTIONS)[number];

export interface Particle {
  // SPH parameters
  pair: number; // pair numbers per one particles
  volume: number; // [m^3]

  // physical quantity for fluid
  /// initial density [kg/m^3]
  rho0: number;
  /// density [kg/m^3]
  rho: number;
  /// viscosity [Pa*s]
  viscosity: number;
  /// sound velocity [m/s]
  sound_v: number;
  /// location vector [m]
  x: Vector3;
  /// velocity [m/s]
  v: Vector3;
  /// Cauthy stress [Pa]
  stress: Matrix3;
  /// acceleration [m/s^2]
  dvdt: Vector3;
  /// total energy [J]
  e: number;
  /// power [J/s]
  dedt: number;
  /// Temperature [K]
  temperature: number;
  /// Fluid type (Water, Air, etc.)
  fluid: FLUID;
}

export interface GuiState {
  particles: Particle[];
  step: number;
  time: number;
}

export const loadParticleState = async (
  path: string
): Promise<GuiState | undefined> => {
  return await invoke<GuiState>("load_particle_state", { path });
};
