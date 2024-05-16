/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["./style/input.css", "./src/**/*.rs"],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
    },
  },

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
