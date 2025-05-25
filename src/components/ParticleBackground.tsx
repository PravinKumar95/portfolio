import React, { useCallback, useEffect, useState } from "react";
import Particles, { initParticlesEngine } from "@tsparticles/react";
import type { Container, Engine, ISourceOptions } from "@tsparticles/engine";
// import { loadFull } from "tsparticles"; // if you are going to use `loadFull`, install the "tsparticles" package too.
import { loadSlim } from "@tsparticles/slim"; // if you are going to use `loadSlim`, install the "@tsparticles/slim" package too.

const ParticleBackground: React.FC = () => {
  const [init, setInit] = useState(false);

  // This should be run only once per application lifetime
  useEffect(() => {
    initParticlesEngine(async (engine: Engine) => {
      // You can initiate the tsParticles instance (engine) here, adding custom shapes or presets
      // This loads the tsparticles package bundle, it's a convenience method to load everything
      // It's recommended to load only the features you need reducing bundle size
      // await loadFull(engine);
      await loadSlim(engine);
    }).then(() => {
      setInit(true);
    });
  }, []);

  const particlesLoaded = useCallback(
    async (container?: Container): Promise<void> => {
      // console.log(container); // For debugging if needed
    },
    []
  );

  const particleOptions: ISourceOptions = {
    autoPlay: true,
    background: {
      color: {
        value: "transparent",
      },
    },
    fpsLimit: 60,
    interactivity: {
      events: {
        onHover: {
          enable: true,
          mode: "repulse",
        },
      },
      modes: {
        repulse: {
          distance: 50, // Reduced distance for subtle effect in nav
          duration: 0.4,
        },
      },
    },
    particles: {
      color: {
        value: "#FDE047", // Tailwind yellow-400 equivalent for bat color
      },
      links: {
        enable: false,
      },
      move: {
        enable: true,
        speed: { min: 0.5, max: 1.5 }, // Slower speed for background
        direction: "left",
        straight: true,
        outModes: {
          default: "out",
        },
      },
      number: {
        density: {
          enable: true,
          area: 1000, // Adjusted area for nav bar density
        },
        value: 20, // Reduced number for nav bar
      },
      opacity: {
        value: { min: 0.2, max: 0.6 }, // More subtle opacity
      },
      shape: {
        type: "image",
        image: {
          src: "/bat.svg", // Path to bat.svg in public folder
          width: 100, // Intrinsic width (doesn't affect particle size directly)
          height: 100, // Intrinsic height
        },
      },
      size: {
        value: { min: 8, max: 20 }, // Slightly smaller bats for nav
      },
    },
    detectRetina: true,
  };

  if (init) {
    return (
      <Particles
        id="tsparticles-nav" // Unique ID for these particles instance
        particlesLoaded={particlesLoaded}
        options={particleOptions}
        className="w-full h-full" // Ensure it fills its container
      />
    );
  }

  return <></>; // Render nothing until initialized
};

export default ParticleBackground;
