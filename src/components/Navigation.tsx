import React, { useState } from "react";
import { Link, NavLink } from "react-router-dom";
import ParticleBackground from "./ParticleBackground"; // Import the new component

const Navigation: React.FC = () => {
  // State for mobile menu toggle (optional, for a simple toggle if not full hamburger)
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const navLinkClasses =
    "px-3 py-2 rounded-md text-sm font-medium text-batman-text-secondary hover:bg-batman-accent-hover hover:text-batman-bg-primary transition-colors";
  const activeNavLinkClasses = "bg-batman-accent text-batman-bg-primary";

  return (
    <nav className="bg-batman-bg-secondary shadow-lg relative">
      {" "}
      {/* relative for particle container positioning */}
      <div
        id="nav-particle-container"
        className="absolute inset-0 z-0 pointer-events-none"
      >
        <ParticleBackground /> {/* Particles rendered here */}
      </div>
      <div className="max-w-7xl mx-auto px-2 sm:px-6 lg:px-8 relative z-10">
        {" "}
        {/* z-10 to keep links above particles */}
        <div className="relative flex items-center justify-between h-16">
          {/* Mobile menu button - basic version for smaller screens if needed */}
          <div className="absolute inset-y-0 left-0 flex items-center sm:hidden">
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              type="button"
              className="inline-flex items-center justify-center p-2 rounded-md text-batman-text-secondary hover:text-white hover:bg-batman-accent-hover focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white"
              aria-controls="mobile-menu"
              aria-expanded={isMobileMenuOpen}
            >
              <span className="sr-only">Open main menu</span>
              {/* Icon when menu is closed. */}
              {!isMobileMenuOpen ? (
                <svg
                  className="block h-6 w-6"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M4 6h16M4 12h16m-7 6h7"
                  />
                </svg>
              ) : (
                // Icon when menu is open.
                <svg
                  className="block h-6 w-6"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                  aria-hidden="true"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
                </svg>
              )}
            </button>
          </div>

          {/* Logo/Brand - Left side */}
          <div className="flex-1 flex items-center justify-center sm:items-stretch sm:justify-start">
            <Link to="/" className="flex-shrink-0 flex items-center">
              {/* Placeholder for a logo, or just text */}
              <span className="text-2xl font-bold text-batman-accent">PK</span>
            </Link>
          </div>

          {/* Desktop Navigation Links - Center/Right side */}
          <div className="hidden sm:block sm:ml-6">
            <div className="flex space-x-4">
              <NavLink
                to="/"
                className={({ isActive }) =>
                  `${navLinkClasses} ${isActive ? activeNavLinkClasses : ""}`
                }
              >
                Home
              </NavLink>
              <NavLink
                to="/about"
                className={({ isActive }) =>
                  `${navLinkClasses} ${isActive ? activeNavLinkClasses : ""}`
                }
              >
                About
              </NavLink>
              <NavLink
                to="/contact"
                className={({ isActive }) =>
                  `${navLinkClasses} ${isActive ? activeNavLinkClasses : ""}`
                }
              >
                Contact
              </NavLink>
              {/* Add more links as needed */}
            </div>
          </div>
        </div>
      </div>
      {/* Mobile menu, show/hide based on menu state. */}
      {isMobileMenuOpen && (
        <div className="sm:hidden relative z-10" id="mobile-menu">
          <div className="px-2 pt-2 pb-3 space-y-1">
            <NavLink
              to="/"
              className={({ isActive }) =>
                `block ${navLinkClasses} ${
                  isActive ? activeNavLinkClasses : ""
                }`
              }
              onClick={() => setIsMobileMenuOpen(false)} // Close menu on click
            >
              Home
            </NavLink>
            <NavLink
              to="/about"
              className={({ isActive }) =>
                `block ${navLinkClasses} ${
                  isActive ? activeNavLinkClasses : ""
                }`
              }
              onClick={() => setIsMobileMenuOpen(false)}
            >
              About
            </NavLink>
            <NavLink
              to="/contact"
              className={({ isActive }) =>
                `block ${navLinkClasses} ${
                  isActive ? activeNavLinkClasses : ""
                }`
              }
              onClick={() => setIsMobileMenuOpen(false)}
            >
              Contact
            </NavLink>
          </div>
        </div>
      )}
    </nav>
  );
};

export default Navigation;
