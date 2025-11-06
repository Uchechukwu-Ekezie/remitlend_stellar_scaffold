/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Dark Theme Base Colors
        dark: {
          bg: {
            primary: "#0a0b0f",
            secondary: "#13151a",
            tertiary: "#1a1d24",
            card: "#1e2129",
            hover: "#252830",
          },
          border: {
            primary: "#2d3139",
            secondary: "#3f4451",
            accent: "#4b5563",
          },
        },
        // Accent Colors
        primary: {
          50: "#eff6ff",
          100: "#dbeafe",
          200: "#bfdbfe",
          300: "#93c5fd",
          400: "#60a5fa",
          500: "#3b82f6",
          600: "#2563eb",
          700: "#1d4ed8",
          800: "#1e40af",
          900: "#1e3a8a",
        },
        success: {
          50: "#ecfdf5",
          100: "#d1fae5",
          200: "#a7f3d0",
          300: "#6ee7b7",
          400: "#34d399",
          500: "#10b981",
          600: "#059669",
          700: "#047857",
          800: "#065f46",
          900: "#064e3b",
        },
        warning: {
          50: "#fffbeb",
          100: "#fef3c7",
          200: "#fde68a",
          300: "#fcd34d",
          400: "#fbbf24",
          500: "#f59e0b",
          600: "#d97706",
          700: "#b45309",
          800: "#92400e",
          900: "#78350f",
        },
        danger: {
          50: "#fef2f2",
          100: "#fee2e2",
          200: "#fecaca",
          300: "#fca5a5",
          400: "#f87171",
          500: "#ef4444",
          600: "#dc2626",
          700: "#b91c1c",
          800: "#991b1b",
          900: "#7f1d1d",
        },
      },
      fontFamily: {
        sans: [
          "Inter",
          "-apple-system",
          "BlinkMacSystemFont",
          "Segoe UI",
          "sans-serif",
        ],
        mono: ["JetBrains Mono", "Courier New", "monospace"],
      },
      boxShadow: {
        soft: "0 2px 8px rgba(0, 0, 0, 0.3)",
        medium: "0 4px 12px rgba(0, 0, 0, 0.4)",
        strong: "0 8px 24px rgba(0, 0, 0, 0.5)",
        primary: "0 4px 20px rgba(59, 130, 246, 0.4)",
        success: "0 4px 20px rgba(16, 185, 129, 0.4)",
        glow: "0 0 20px rgba(59, 130, 246, 0.6)",
      },
      borderRadius: {
        xl: "1rem",
        "2xl": "1.5rem",
      },
    },
  },
  plugins: [],
};
