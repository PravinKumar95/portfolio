import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import MainLayout from "./layouts/MainLayout";
import HomePage from "./pages/HomePage";
import AboutPage from "./pages/AboutPage";
import ContactPage from "./pages/ContactPage";
// Placeholder for any global styles not handled by Tailwind's base layer in index.css
// For example, if you had an App.css with specific overrides:
// import './App.css';

const App: React.FC = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<HomePage />} />
          <Route path="about" element={<AboutPage />} />
          <Route path="contact" element={<ContactPage />} />
          {/* 
            Future routes for dedicated pages can be added here:
            e.g., <Route path="experience" element={<ExperiencePage />} />
            e.g., <Route path="projects" element={<ProjectsPage />} /> 
          */}
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

export default App;
