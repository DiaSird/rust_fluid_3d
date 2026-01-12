import { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { useParameters } from "./providers/parameters/useParameters";
import type { Particle } from "../api/simulation";

const createScene = () => {
  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0x111111);
  return scene;
};

const createCamera = (
  container: HTMLDivElement,
  length: number,
  width: number,
  height: number
) => {
  const camera = new THREE.PerspectiveCamera(
    60,
    container.clientWidth / container.clientHeight,
    0.1,
    1000
  );

  const maxSize = Math.max(length, width, height);
  const d = maxSize * 1.2;
  camera.position.set(d, d, d);
  camera.lookAt(0, 0, 0);

  return camera;
};

const createRenderer = (container: HTMLDivElement) => {
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(container.clientWidth, container.clientHeight);
  container.innerHTML = "";
  container.appendChild(renderer.domElement);
  return renderer;
};

const createGrid = (length: number, width: number, dx: number) => {
  const divisions = Math.max(2, Math.floor(length / dx));
  const grid = new THREE.GridHelper(length, divisions);
  grid.position.y = -width / 2;
  return grid;
};

const generateParticlesGeometry = (
  nx: number,
  ny: number,
  nz: number,
  length: number,
  width: number,
  height: number
) => {
  const geometry = new THREE.BufferGeometry();
  const total = nx * ny * nz;
  const positions = new Float32Array(total * 3);

  let idx = 0;
  for (let i = 0; i < nx; i++) {
    for (let j = 0; j < ny; j++) {
      for (let k = 0; k < nz; k++) {
        positions[idx++] = (i / (nx - 1) - 0.5) * length;
        positions[idx++] = (j / (ny - 1) - 0.5) * width;
        positions[idx++] = (k / (nz - 1) - 0.5) * height;
        // positions[idx++] = i * (length / nx);
        // positions[idx++] = j * (width / ny);
        // positions[idx++] = k * (height / nz);
      }
    }
  }

  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  return geometry;
};

const resultParticlesGeometry = (particles: Particle[]) => {
  const geometry = new THREE.BufferGeometry();
  const total = particles.length;

  const positions = new Float32Array(total * 3);
  const colors = new Float32Array(total * 3);
  const MAX_SPEED = 10.0;
  const color = new THREE.Color();

  let idx = 0;
  for (let i = 0; i < total; i++) {
    positions[idx++] = particles[i].x[0] - 0.25;
    positions[idx++] = particles[i].x[1] - 0.25;
    positions[idx++] = particles[i].x[2] - 0.25;

    const vx = particles[i].v[0];
    const vy = particles[i].v[1];
    const vz = particles[i].v[2];

    const speed = Math.sqrt(vx * vx + vy * vy + vz * vz);
    const vNorm = Math.min(speed / MAX_SPEED, 1.0);

    // hue: 0.7 (blue) → 0.0 (red)
    const hue = (1.0 - vNorm) * 0.7;
    color.setHSL(hue, 1.0, 0.5);

    colors[i * 3 + 0] = color.r;
    colors[i * 3 + 1] = color.g;
    colors[i * 3 + 2] = color.b;
  }

  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  geometry.setAttribute("color", new THREE.BufferAttribute(colors, 3));

  return geometry;
};

const createParticles = (
  nx: number,
  ny: number,
  nz: number,
  length: number,
  width: number,
  height: number,
  dx: number,
  dy: number,
  dz: number,
  particles?: Particle[]
) => {
  const isResult = !!particles;

  const geometry = isResult
    ? resultParticlesGeometry(particles)
    : generateParticlesGeometry(nx, ny, nz, length, width, height);

  const material = new THREE.PointsMaterial({
    color: isResult ? 0xff5555 : 0x44aaff,
    size: Math.min(dx, dy, dz) * 0.2,
    vertexColors: isResult,
  });

  return new THREE.Points(geometry, material);
};

/* ---------- component ---------- */

export const ThreeCanvas: React.FC = () => {
  const mountRef = useRef<HTMLDivElement>(null);
  const { state } = useParameters();

  const { length, width, height } = state.model_scale;
  const { dx, dy, dz } = state.dx;

  const nx = Math.max(2, Math.floor(length / dx));
  const ny = Math.max(2, Math.floor(width / dy));
  const nz = Math.max(2, Math.floor(height / dz));

  useEffect(() => {
    if (!mountRef.current) return;

    const scene = createScene();
    const camera = createCamera(mountRef.current, length, width, height);
    const renderer = createRenderer(mountRef.current);
    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;

    let grid = createGrid(length, width, dx);
    scene.add(grid);

    const particles = createParticles(
      nx,
      ny,
      nz,
      length,
      width,
      height,
      dx,
      dy,
      dz,
      state.guiState?.particles
    );
    scene.add(particles);

    const animate = () => {
      requestAnimationFrame(animate);
      controls.update();
      if (!state.isRunning) {
        particles.rotation.y += 0.001;
      }
      renderer.render(scene, camera);
    };
    animate();

    const ro = new ResizeObserver(() => {
      if (!mountRef.current) return;

      const w = mountRef.current.clientWidth;
      const h = mountRef.current.clientHeight;

      camera.aspect = w / h;
      camera.updateProjectionMatrix();
      renderer.setSize(w, h);

      scene.remove(grid);
      grid = createGrid(length, width, dx);
      scene.add(grid);
    });

    ro.observe(mountRef.current);

    const current = mountRef.current;
    return () => {
      ro.disconnect();
      renderer.dispose();
      current?.removeChild(renderer.domElement);
    };
  }, [
    length,
    width,
    height,
    dx,
    dy,
    dz,
    nx,
    ny,
    nz,
    state.isRunning,
    state.guiState,
  ]);

  return <div ref={mountRef} style={{ width: "100%", height: "100%" }} />;
};
