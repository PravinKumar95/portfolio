import React from "react";
import Button from "../components/Button"; // Assuming Button.tsx is in src/components/
import Card from "../components/Card"; // For consistent section styling

const ContactPage: React.FC = () => {
  return (
    <div className="space-y-12">
      <section id="contact-info" className="text-center">
        <h2 className="text-4xl font-bold text-batman-text-primary mb-8">
          Get In Touch
        </h2>
        <Card
          title="Contact Details"
          description="I'm always open to discussing new projects, creative ideas, or opportunities to be part of something amazing. Feel free to reach out!"
          tags={["Collaboration", "Opportunities", "Tech Talk"]}
        >
          <div className="mt-6 space-y-6">
            <div>
              <p className="text-lg text-batman-text-secondary mb-2">
                Email Address:
              </p>
              <Button
                href="mailto:pravinshanmugam1995@gmail.com"
                variant="primary"
                className="text-lg"
              >
                pravinshanmugam1995@gmail.com
              </Button>
            </div>

            <div>
              <p className="text-lg text-batman-text-secondary mb-2">
                GitHub Profile:
              </p>
              <Button
                href="https://github.com/PravinKumar95"
                variant="secondary"
                target="_blank"
                rel="noopener noreferrer"
                className="text-lg"
              >
                PravinKumar95 on GitHub
              </Button>
            </div>

            {/* Placeholder for LinkedIn or other contact methods if needed */}
            {/* 
            <div>
              <p className="text-lg text-batman-text-secondary mb-2">LinkedIn Profile:</p>
              <Button 
                href="YOUR_LINKEDIN_PROFILE_URL" 
                variant="secondary" 
                target="_blank" 
                rel="noopener noreferrer"
                className="text-lg"
              >
                Your Name on LinkedIn
              </Button>
            </div> 
            */}
          </div>
        </Card>
      </section>

      {/* Optional: Placeholder for a contact form for future enhancement */}
      {/* 
      <section id="contact-form-placeholder" className="text-center">
        <h2 className="text-3xl font-bold text-batman-text-primary mb-6">Or Send Me a Message Directly</h2>
        <div className="bg-batman-bg-secondary p-8 rounded-lg shadow-xl max-w-2xl mx-auto">
          <p className="text-batman-text-secondary">
            A contact form component could be implemented here in the future.
          </p>
          <form className="mt-4 space-y-4">
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-batman-text-secondary">Full Name</label>
              <input type="text" name="name" id="name" className="mt-1 block w-full px-3 py-2 bg-batman-bg-primary border border-batman-bg-primary rounded-md shadow-sm focus:outline-none focus:ring-batman-accent focus:border-batman-accent sm:text-sm" placeholder="Your Name" />
            </div>
            <div>
              <label htmlFor="email" className="block text-sm font-medium text-batman-text-secondary">Email</label>
              <input type="email" name="email" id="email" className="mt-1 block w-full px-3 py-2 bg-batman-bg-primary border border-batman-bg-primary rounded-md shadow-sm focus:outline-none focus:ring-batman-accent focus:border-batman-accent sm:text-sm" placeholder="you@example.com" />
            </div>
            <div>
              <label htmlFor="message" className="block text-sm font-medium text-batman-text-secondary">Message</label>
              <textarea id="message" name="message" rows={4} className="mt-1 block w-full px-3 py-2 bg-batman-bg-primary border border-batman-bg-primary rounded-md shadow-sm focus:outline-none focus:ring-batman-accent focus:border-batman-accent sm:text-sm" placeholder="Your message..."></textarea>
            </div>
            <div>
              <Button type="submit" variant="primary" className="w-full">Send Message</Button>
            </div>
          </form>
        </div>
      </section>
      */}
    </div>
  );
};

export default ContactPage;
