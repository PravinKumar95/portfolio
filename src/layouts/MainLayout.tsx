import React from "react";
import { Outlet } from "react-router-dom";
import Navigation from "../components/Navigation";

const MainLayout: React.FC = () => {
  return (
    <div className="flex flex-col min-h-screen bg-batman-bg-primary text-batman-text-primary">
      <Navigation />

      <main className="flex-grow container mx-auto px-4 py-8">
        <Outlet /> {/* This is where the routed page components will render */}
      </main>

      <footer className="bg-batman-bg-secondary text-batman-text-secondary p-4 text-center shadow-md mt-auto">
        <p>
          &copy; {new Date().getFullYear()} Pravin Kumar. All rights reserved.
        </p>
        <p className="text-sm">Powered by React & Tailwind CSS</p>
      </footer>
    </div>
  );
};

export default MainLayout;
