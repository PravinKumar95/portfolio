// tsParticles implementation for bat animation in the header

document.addEventListener("DOMContentLoaded", () => {
  tsParticles.load("particles-js-header", {
    autoPlay: true,
    background: {
      color: {
        value: "transparent", // Make canvas background transparent
      },
    },
    fpsLimit: 60,
    interactivity: {
      events: {
        onHover: {
          enable: false, // No hover effect for now
        },
        onClick: {
          enable: false, // No click effect for now
        },
      },
    },
    particles: {
      color: {
        value: "#333333", // Dark gray/black for bats
      },
      links: {
        enable: false, // No links between particles
      },
      move: {
        direction: "left",
        enable: true,
        outModes: {
          default: "out", // Particles disappear when they go off screen
        },
        random: true,
        speed: { min: 1, max: 3 },
        straight: false,
        trail: {
          enable: false,
        },
      },
      number: {
        density: {
          enable: true,
          area: 800, // Adjust for desired density
        },
        value: 40, // Number of particles
      },
      opacity: {
        value: { min: 0.3, max: 0.8 },
        animation: {
          enable: true,
          speed: 1,
          minimumValue: 0.1,
          sync: false,
        },
      },
      shape: {
        type: "image",
        image: {
          src: "bat.svg",
          width: 100, // Intrinsic width of SVG particle
          height: 66, // Intrinsic height of SVG particle
        },
      },
      size: {
        value: { min: 8, max: 16 }, // Bat size
        animation: {
          enable: false,
        },
      },
    },
    detectRetina: true,
    // Ensure tsParticles canvas is behind other header content
    // This is often handled by CSS, but zLayers can be an option.
    // For this setup, we'll primarily use CSS for positioning.
    // fullScreen: { enable: false } // Not using fullScreen mode for a specific div
  });

  // Smooth scrolling for navigation links
  const navLinks = document.querySelectorAll('header nav ul li a[href^="#"]');

  navLinks.forEach((link) => {
    link.addEventListener("click", function (event) {
      event.preventDefault();
      const targetId = this.getAttribute("href");
      const targetElement = document.querySelector(targetId);

      if (targetElement) {
        targetElement.scrollIntoView({
          behavior: "smooth",
        });
      }
    });
  });
});
