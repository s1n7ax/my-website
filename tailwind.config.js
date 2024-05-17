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

      keyframes: {
        typing: {
          "0%": {
            left: "0%",
          },
          "100%": {
            left: "100%",
          },
        },

        "blink-caret": {
          "from, to": { "border-left-width": 0 },
          "50%": { "border-left-width": "2px" },
        },
      },
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
