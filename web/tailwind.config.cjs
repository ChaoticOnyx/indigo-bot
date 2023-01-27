module.exports = {
  content: ["./*.css", "../templates/**/*.html"],
  theme: {
    fontFamily: {
      sans: ["Lato", "Arial", "sans-serif"],
      mono: ["monospace"],
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    styled: true,
    themes: [
      {
        indigoDark: {
          primary: "#3267be",
          "primary-focus": "#3b82f6",
          "primary-content": "#fff",
          "base-100": "#171717",
          "base-content": "#ebeae9",
          neutral: "#1e1e1e",
          "neutral-content": "#bababa",
          error: "#b93939",
          "error-content": "#fff",
          success: "#1f9a4c",
          "success-content": "#fff",
        },
      },
    ],
    base: true,
    utils: true,
    logs: true,
    rtl: false,
    prefix: "",
    darkTheme: "dark",
  },
};
