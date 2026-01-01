import { useRef } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";
import ThreeView from "./ThreeView";

function App() {
  const threeViewRef = useRef<any>(null);

  return (
    <div style={{ display: "flex", flexDirection: "column", height: "100vh" }}>
      {/* Header section */}
      <div
        style={{
          height: "120px",
          display: "flex",
          alignItems: "center",
          padding: "0 20px",
          background: "#222",
          color: "#fff",
        }}
      >
        <div style={{ display: "flex", alignItems: "center", gap: "20px" }}>
          {/* Vite logo */}
          <a href="https://vite.dev" target="_blank" rel="noreferrer">
            <img src={viteLogo} className="logo" alt="Vite logo" />
          </a>
          {/* React logo */}
          <a href="https://react.dev" target="_blank" rel="noreferrer">
            <img src={reactLogo} className="logo react" alt="React logo" />
          </a>
          {/* App title */}
          <h1 style={{ margin: 0 }}>Terra Solver</h1>
        </div>

        {/* Run Simulation */}
        {/* <div style={{ marginLeft: "auto" }}>
          <button
            onClick={async () => {
              // Run Simulation
              threeViewRef.current?.exportParameters?.();
            }}
            style={{ padding: "5px 10px" }}
          >
            Run
          </button>
        </div> */}
      </div>

      {/* Three.js rendering area */}
      <div style={{ flex: 1 }}>
        <ThreeView />
      </div>
    </div>
  );
}

export default App;
