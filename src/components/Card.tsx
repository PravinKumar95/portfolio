import React from "react";

interface CardProps {
  title: string;
  subtitle?: string;
  description: string;
  tags?: string[];
  link?: string;
  linkText?: string;
}

const Card: React.FC<CardProps> = ({
  title,
  subtitle,
  description,
  tags,
  link,
  linkText = "Learn More",
}) => {
  return (
    <div className="bg-batman-bg-secondary p-6 rounded-lg shadow-lg border-l-4 border-batman-accent transition-all duration-300 hover:shadow-xl hover:border-batman-accent-hover">
      {link ? (
        <a
          href={link}
          target="_blank"
          rel="noopener noreferrer"
          className="text-2xl font-bold text-batman-accent hover:text-batman-accent-hover transition-colors"
        >
          {title}
        </a>
      ) : (
        <h3 className="text-2xl font-bold text-batman-accent">{title}</h3>
      )}
      {subtitle && (
        <p className="text-sm text-batman-text-secondary mt-1 mb-3 italic">
          {subtitle}
        </p>
      )}
      <p className="text-batman-text-primary my-4">{description}</p>
      {tags && tags.length > 0 && (
        <div className="mt-4 flex flex-wrap gap-2">
          {tags.map((tag, index) => (
            <span
              key={index}
              className="bg-batman-bg-primary text-batman-accent text-xs font-semibold px-3 py-1 rounded-full"
            >
              {tag}
            </span>
          ))}
        </div>
      )}
      {link &&
        linkText &&
        !title.includes(linkText) && ( // Avoid duplicate "Learn More" if title is already a link
          <div className="mt-4">
            <a
              href={link}
              target="_blank"
              rel="noopener noreferrer"
              className="text-batman-accent hover:text-batman-accent-hover font-semibold transition-colors"
            >
              {linkText}
            </a>
          </div>
        )}
    </div>
  );
};

export default Card;
