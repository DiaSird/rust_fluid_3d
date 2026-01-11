import { listen } from "@tauri-apps/api/event";
import { useParameters } from "../providers/parameters/ParameterContext";
import { useEffect } from "react";
import { particleLogToString, type ParticleLogNormalized } from "../../api/log";

export const useWatchLog = () => {
  const params = useParameters();

  useEffect(() => {
    const unlistenPromise = listen<ParticleLogNormalized>("terra://simulation-log", (event) => {
      params.setLog((prev) => [...prev, particleLogToString(event.payload)]);
      // params.setLog((prev) => [...prev, JSON.stringify(event.payload)]);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [params]);
};
