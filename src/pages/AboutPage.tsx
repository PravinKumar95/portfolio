import React from "react";
import Card from "../components/Card"; // Assuming Card.tsx is in src/components/

const AboutPage: React.FC = () => {
  const professionalSummary = `
    A results-oriented Technical Lead with over 7 years of experience in architecting, developing, and deploying 
    scalable software solutions across multiple platforms. My expertise lies in microservices architecture, 
    full-stack development with a focus on React, Node.js, and TypeScript, and leading agile teams 
    to deliver complex projects in demanding environments, particularly within the automotive and technology sectors. 
    I am passionate about leveraging Domain-Driven Design principles and modern cloud technologies (AWS) 
    to build robust and efficient systems. I thrive on solving complex technical challenges and mentoring engineers 
    to foster growth and innovation.
  `;

  const experiences = [
    {
      title: "Technical Lead",
      company: "ZF Group",
      duration: "Apr 2025 - Present",
      description:
        "Building orchestration platform for fleets. Developing mobile app for drivers.",
      skills: [
        "MongoDB",
        "Domain-Driven Design (DDD)",
        "React Native",
        "Microservices",
        "System Architecture",
        "Team Leadership",
      ],
    },
    {
      title: "Senior Software Engineer",
      company: "ZF Group",
      duration: "Feb 2023 - May 2025",
      description:
        "Developed a web platform for autonomous vehicle management, focusing on front-end architecture and AWS integration.",
      skills: [
        "TypeScript",
        "React.js",
        "JavaScript",
        "Amazon Web Services (AWS)",
        "Terraform",
        "Redux.js",
        "Responsive Web Design",
        "Scrum",
        "Problem Solving",
      ],
    },
    {
      title: "Application Development Senior Analyst",
      company: "Accenture",
      duration: "Mar 2022 - Feb 2023",
      description:
        "Contributed to application development projects for various clients, focusing on front-end technologies and ensuring adherence to technical requirements and W3C accessibility standards.",
      skills: [
        "TypeScript",
        "React.js",
        "Responsive Web Design",
        "SASS",
        "HTML5",
        "W3C Accessibility",
        "Scrum",
        "Business Requirements",
      ],
    },
    {
      title: "Software Engineer",
      company: "Visual Collaboration Technologies",
      duration: "Jun 2018 - Mar 2022",
      description:
        "Developed and maintained software solutions, specializing in web technologies and high-performance computing applications for 3D visualization and collaboration.",
      skills: [
        "WebGL",
        "TypeScript",
        "React.js",
        "C++",
        "Python (Programming Language)",
        "Amazon S3",
        "High Performance Computing (HPC)",
        "Communication",
      ],
    },
  ];

  const education = [
    {
      title: "Bachelor of Engineering - Production Engineering",
      institution: "PSG College of Technology",
      duration: "2013 - 2018",
      description:
        "Graduated with a comprehensive understanding of manufacturing processes, industrial engineering, and quality control, complemented by software development electives.",
    },
    {
      title: "Full Stack Web Development",
      institution: "Zero To Mastery Academy",
      duration: "Ongoing Learning", // Or a specific year if completed
      description:
        "Completed intensive online programs covering modern web development technologies including React, Node.js, Express, databases, and advanced JavaScript concepts.",
    },
  ];

  const projects = [
    {
      title: "Fleet Orchestration Platform",
      company: "ZF Group (Tech Lead)",
      description:
        "Led the design and development of a microservices-based orchestration platform for managing large-scale vehicle fleets. Responsible for system architecture, database design (MongoDB), and guiding the development team using DDD principles. The platform significantly improved operational efficiency and data management capabilities.",
      tags: [
        "MongoDB",
        "Domain-Driven Design (DDD)",
        "System Architecture",
        "Microservices",
        "Node.js",
        "Kafka",
      ],
    },
    {
      title: "Driver Mobile Application",
      company: "ZF Group (Tech Lead)",
      description:
        "Spearheaded the creation of a cross-platform mobile application for drivers using React Native. Focused on intuitive UI/UX, real-time data synchronization with the backend fleet systems, and offline capabilities. Ensured seamless integration with existing APIs and services.",
      tags: [
        "React Native",
        "Mobile Development",
        "API Integration",
        "Cross-Platform",
        "UX/UI Design",
      ],
    },
    {
      title: "Autonomous Vehicle Management Web Platform",
      company: "ZF Group (Senior Software Engineer)",
      description:
        "Played a key role as a senior engineer in developing a web platform for the management and monitoring of autonomous vehicles. Contributed to front-end architecture using React.js and TypeScript, state management with Redux, and integration with AWS services for backend functionalities.",
      tags: [
        "React.js",
        "TypeScript",
        "Amazon Web Services (AWS)",
        "Redux.js",
        "JavaScript",
      ],
    },
  ];

  return (
    <div className="space-y-12">
      <section
        id="about-me"
        className="bg-batman-bg-secondary p-8 rounded-lg shadow-xl"
      >
        <h2 className="text-4xl font-bold text-batman-accent mb-6 text-center">
          About Me
        </h2>
        <p className="text-lg text-batman-text-secondary leading-relaxed whitespace-pre-line">
          {professionalSummary.trim()}
        </p>
      </section>

      <section id="experience">
        <h2 className="text-4xl font-bold text-batman-text-primary mb-8 text-center">
          Work Experience
        </h2>
        <div className="space-y-8">
          {experiences.map((exp, index) => (
            <Card
              key={index}
              title={`${exp.title} - ${exp.company}`}
              subtitle={exp.duration}
              description={exp.description}
              tags={exp.skills}
            />
          ))}
        </div>
      </section>

      <section id="projects">
        <h2 className="text-4xl font-bold text-batman-text-primary mb-8 text-center">
          Projects
        </h2>
        <div className="space-y-8">
          {projects.map((proj, index) => (
            <Card
              key={index}
              title={proj.title}
              subtitle={proj.company}
              description={proj.description}
              tags={proj.tags}
            />
          ))}
        </div>
      </section>

      <section id="education">
        <h2 className="text-4xl font-bold text-batman-text-primary mb-8 text-center">
          Education
        </h2>
        <div className="space-y-8">
          {education.map((edu, index) => (
            <Card
              key={index}
              title={edu.title}
              subtitle={edu.institution}
              description={`${edu.duration}. ${edu.description}`} // Combine duration and desc for card
            />
          ))}
        </div>
      </section>
    </div>
  );
};

export default AboutPage;
