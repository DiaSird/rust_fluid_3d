import { useParameters } from "./providers/parameters/useParameters";
import { useCallback } from "react";
import { loadParticleState } from "../api/simulation";

export const DrawParticleButton: React.FC = () => {
  const { state, dispatch } = useParameters();

  const handleDraw = useCallback(async () => {
    try {
      if (state.restart_file == undefined || state.restart_file == "") {
        console.log("File Not Found Error.");
        return;
      }
      const guiState = await loadParticleState(state.restart_file);

      if (guiState) {
        dispatch({
          type: "SET_GUI_STATE",
          value: guiState,
        });
      } else {
        console.log("SET_GUI_STATE: Failed to read state.");
      }
    } catch (e) {
      dispatch({
        type: "APPEND_LOG",
        value: e as string,
      });
    }
  }, [dispatch, state]);

  return (
    <button style={stopStyle} onClick={handleDraw}>
      Draw Model
    </button>
  );
};

const stopStyle: React.CSSProperties = {
  width: "100%",
  padding: "8px",
  background: "rgba(39, 213, 236, 0.84)",
  color: "#fff",
  border: "none",
  marginBottom: "10px",
};
