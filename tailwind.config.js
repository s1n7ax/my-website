/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./style/input.css", "./src/**/*.rs"],
  theme: {
    extend: {
      aspectRatio: {
        "4/3": "4 / 3",
        "3/4": "3 / 4",
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
