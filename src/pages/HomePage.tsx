import React from "react";
import Card from "../components/Card";
import Button from "../components/Button";
import { Link } from "react-router-dom"; // For internal navigation buttons

const HomePage: React.FC = () => {
  return (
    <div className="space-y-12">
      {/* Hero Section */}
      <section
        id="hero"
        className="min-h-[40vh] bg-batman-bg-secondary p-8 rounded-lg shadow-2xl flex flex-col justify-center items-center text-center"
      >
        <h1 className="text-5xl sm:text-6xl md:text-7xl font-bold text-batman-accent mb-4 animate-fade-in-down">
          Pravin Kumar
        </h1>
        <p className="text-xl sm:text-2xl text-batman-text-secondary max-w-2xl animate-fade-in-up">
          Technical Lead | SCALAR | Building Scalable Solutions | Microservices
          | Multi-platform
        </p>
        <div className="mt-8 animate-fade-in-up animation-delay-300">
          <Button href="#experience-preview" variant="primary" className="mr-4">
            View My Work
          </Button>
          <Button href="/contact" variant="secondary">
            Get In Touch
          </Button>
        </div>
      </section>

      {/* Experience Preview */}
      <section id="experience-preview">
        <h2 className="text-4xl font-bold text-batman-text-primary mb-8 text-center">
          Recent Experience
        </h2>
        <div className="grid md:grid-cols-2 gap-8">
          <Card
            title="Technical Lead - ZF Group"
            subtitle="Apr 2025 - Present"
            description="Building orchestration platform for fleets. Developing mobile app for drivers."
            tags={[
              "MongoDB",
              "Domain-Driven Design (DDD)",
              "React Native",
              "Microservices",
              "System Architecture",
            ]}
          />
          <Card
            title="Senior Software Engineer - ZF Group"
            subtitle="Feb 2023 - May 2025"
            description="Developing a web platform for autonomous vehicle management."
            tags={[
              "TypeScript",
              "React.js",
              "JavaScript",
              "AWS",
              "Terraform",
              "Redux.js",
              "Responsive Web Design",
            ]}
          />
        </div>
        <div className="text-center mt-8">
          <Link to="/about#experience">
            {" "}
            {/* Direct link to experience section in about page */}
            <Button variant="secondary">View All Experience</Button>
          </Link>
        </div>
      </section>

      {/* Projects Preview */}
      <section id="projects-preview">
        <h2 className="text-4xl font-bold text-batman-text-primary mb-8 text-center">
          Featured Projects
        </h2>
        <div className="grid md:grid-cols-2 gap-8">
          <Card
            title="Fleet Orchestration Platform (ZF Group)"
            subtitle="Role: Tech Lead"
            description="Led the development of a comprehensive orchestration platform for managing vehicle fleets, enhancing operational efficiency and enabling advanced fleet management capabilities."
            tags={[
              "MongoDB",
              "Domain-Driven Design (DDD)",
              "System Architecture",
              "Microservices",
            ]}
          />
          <Card
            title="Driver Mobile Application (ZF Group)"
            subtitle="Role: Tech Lead"
            description="Spearheaded the creation of a cross-platform mobile application for drivers, focusing on intuitive UI/UX, real-time data synchronization, and integration with backend fleet systems."
            tags={[
              "React Native",
              "Mobile Development",
              "API Integration",
              "Cross-Platform",
            ]}
          />
        </div>
        <div className="text-center mt-8">
          {/* This button can link to a future dedicated projects page or the projects section on About page */}
          <Link to="/about#projects">
            <Button variant="secondary">View All Projects</Button>
          </Link>
        </div>
      </section>
    </div>
  );
};

// Basic animation classes (add to your CSS or a global style if preferred)
// For simplicity, including here. Ideally, these would be in index.css or App.css
const animationStyles = `
  @keyframes fade-in-down {
    0% {
      opacity: 0;
      transform: translateY(-20px);
    }
    100% {
      opacity: 1;
      transform: translateY(0);
    }
  }
  .animate-fade-in-down {
    animation: fade-in-down 0.5s ease-out forwards;
  }

  @keyframes fade-in-up {
    0% {
      opacity: 0;
      transform: translateY(20px);
    }
    100% {
      opacity: 1;
      transform: translateY(0);
    }
  }
  .animate-fade-in-up {
    animation: fade-in-up 0.5s ease-out forwards;
  }
  .animation-delay-300 {
    animation-delay: 0.3s;
  }
`;

// Inject animation styles into the head
const styleSheet = document.createElement("style");
styleSheet.type = "text/css";
styleSheet.innerText = animationStyles;
document.head.appendChild(styleSheet);

export default HomePage;
