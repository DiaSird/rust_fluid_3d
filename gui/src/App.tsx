import "./App.css";
import ThreeView from "./components/ThreeView";
import { ParameterProvider } from "./components/providers/parameters/ParameterProvider";
import { RunStopButton } from "./components/RunStopButton";
import { useWatchLog } from "./components/hooks/useWatchLog";
import { DrawParticleButton } from "./components/DrawParticleButton";

export default function App() {
  return (
    <ParameterProvider>
      <AppInner />
    </ParameterProvider>
  );
}

/**
 *  # Why separate `App` and `AppInner`?
 *`Provider` and `useContext` cannot be placed in the same React Component.
 */
const AppInner = () => {
  useWatchLog();

  return (
    <div style={root}>
      {/* Main view (Parameters + Canvas + Log) */}
      <ThreeView />

      {/* Bottom Run / Stop bar */}
      <div style={runBar}>
        <RunStopButton /> <DrawParticleButton />
      </div>
    </div>
  );
};

const RUN_BAR_HEIGHT = 50;

const root: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  height: "100vh",
  width: "100vw",
  overflow: "hidden",
};

const runBar: React.CSSProperties = {
  position: "fixed",
  bottom: 0,
  left: 0,
  right: 0,
  height: RUN_BAR_HEIGHT,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  background: "#111",
  borderTop: "1px solid #333",
  zIndex: 1000,
};
