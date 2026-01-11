import { useEffect, useRef } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { useParameters } from "./ParameterContext";

export const ThreeCanvas: React.FC = () => {
  const mountRef = useRef<HTMLDivElement>(null);
  const { model_scale, dx: resolution } = useParameters();

  const { length, width, height } = model_scale;
  const { dx, dy, dz } = resolution;

  const { nx, ny, nz } = {
    nx: Math.max(2, Math.floor(length / dx)),
    ny: Math.max(2, Math.floor(width / dy)),
    nz: Math.max(2, Math.floor(height / dz)),
  };

  useEffect(() => {
    if (!mountRef.current) return;

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x111111);

    const camera = new THREE.PerspectiveCamera(
      60,
      mountRef.current.clientWidth / mountRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.set(10, 10, 10);
    camera.lookAt(0, 0, 0);

    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(
      mountRef.current.clientWidth,
      mountRef.current.clientHeight
    );
    mountRef.current.appendChild(renderer.domElement);

    const controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;

    let grid = new THREE.GridHelper(10, 10);
    grid.position.y = -width / 2;
    scene.add(grid);

    const generateParticles = () => {
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
          }
        }
      }
      geometry.setAttribute(
        "position",
        new THREE.BufferAttribute(positions, 3)
      );
      return geometry;
    };

    const material = new THREE.PointsMaterial({ color: 0x44aaff, size: 0.05 });
    let particles = new THREE.Points(generateParticles(), material);
    scene.add(particles);

    const animate = () => {
      requestAnimationFrame(animate);
      controls.update();
      particles.rotation.y += 0.001;
      renderer.render(scene, camera);
    };
    animate();

    const ro = new ResizeObserver(() => {
      const width = mountRef.current!.clientWidth;
      const height = mountRef.current!.clientHeight;

      camera.aspect = width / height;
      camera.updateProjectionMatrix();
      renderer.setSize(width, height);

      particles.scale.set(
        width / mountRef.current!.offsetWidth,
        height / mountRef.current!.offsetHeight,
        width / mountRef.current!.offsetWidth
      );
      particles.material.needsUpdate = true;

      scene.remove(grid);
      grid = new THREE.GridHelper(length, Math.ceil(length));
      grid.position.y = -width / 2;
      scene.add(grid);
    });
    ro.observe(mountRef.current!);

    return () => {
      ro.disconnect();
      renderer.dispose();
      mountRef.current?.removeChild(renderer.domElement);
    };
  }, [length, width, height, dx, dy, dz]);

  return <div ref={mountRef} style={{ width: "100%", height: "100%" }} />;
};
