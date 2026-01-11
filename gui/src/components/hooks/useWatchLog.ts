import { listen } from "@tauri-apps/api/event";
import { useParameters } from "../providers/parameters/useParameters";
import { useEffect } from "react";
import { particleLogToString, type ParticleLogNormalized } from "../../api/log";

export const useWatchLog = () => {
  const { dispatch } = useParameters();

  useEffect(() => {
    const unlistenPromise = listen<ParticleLogNormalized>("terra://simulation-log", (event) => {
      dispatch({ type: "APPEND_LOG", value: particleLogToString(event.payload) });
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, [dispatch]);
};
