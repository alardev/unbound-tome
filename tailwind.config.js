/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html", "./*.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
