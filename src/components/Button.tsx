import React from "react";

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  href?: string;
  variant?: "primary" | "secondary";
  type?: "button" | "submit" | "reset";
  className?: string;
  target?: string;
  rel?: string;
}

const Button: React.FC<ButtonProps> = ({
  children,
  onClick,
  href,
  variant = "primary",
  type = "button",
  className = "",
  target,
  rel,
}) => {
  const baseStyles =
    "px-6 py-3 rounded-md font-semibold transition-all duration-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-batman-bg-primary";

  const variants = {
    primary:
      "bg-batman-accent text-batman-bg-primary hover:bg-batman-accent-hover focus:ring-batman-accent",
    secondary:
      "bg-transparent border-2 border-batman-accent text-batman-accent hover:bg-batman-accent hover:text-batman-bg-primary focus:ring-batman-accent",
  };

  const combinedClassName = `${baseStyles} ${variants[variant]} ${className}`;

  if (href) {
    return (
      <a
        href={href}
        className={combinedClassName}
        onClick={onClick} // Allow onClick for analytics or other purposes even on links
        target={target}
        rel={rel}
      >
        {children}
      </a>
    );
  }

  return (
    <button type={type} className={combinedClassName} onClick={onClick}>
      {children}
    </button>
  );
};

export default Button;
