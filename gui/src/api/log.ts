export type Vector3 = [number, number, number];

export type ParticleLogNormalized =
  | {
      kind: "RestartInfo";
      data: {
        step?: number | null;
        time: number;
        message: string;
      };
    }
  | {
      kind: "LogInfo";
      data: string;
    }
  | {
      kind: "Info3";
      data: {
        monitor_particle: number;
        step: number;
        time: number;
        x: Vector3;
        v: Vector3;
        dvdt: Vector3;
      };
    };

/**
 * To fixed 3 digits float string.
 */
const f3 = (v: number): string => {
  return v.toFixed(3);
};

/**
 * To Formatted log event string.
 */
export const particleLogToString = (log: ParticleLogNormalized): string => {
  switch (log.kind) {
    case "RestartInfo": {
      const { step, time, message } = log.data;
      const stepStr = step != null ? `step=${step} ` : "";
      return `[Restart] ${stepStr}time=${time.toFixed(6)} ${message}`;
    }

    case "LogInfo":
      return `[Info] ${log.data}`;

    case "Info3": {
      const { monitor_particle, step, time, x, v, dvdt } = log.data;

      return (
        "------------------------------------------\n" +
        `Step ${step}, time = ${f3(time * 1000)} [ms]\n` +
        `    Particle: ${monitor_particle}\n` +
        `    (x, y, z) = ${f3(x[0])}, ${f3(x[1])}, ${f3(x[2])}\n` +
        `    (vx, vy, vz) = ${f3(v[0])}, ${f3(v[1])}, ${f3(v[2])}\n` +
        `    (ax, ay, az) = ${f3(dvdt[0])}, ${f3(dvdt[1])}, ${f3(dvdt[2])}\n` +
        "------------------------------------------"
      );
    }

    default: {
      const _exhaustive: never = log;
      return _exhaustive;
    }
  }
};
