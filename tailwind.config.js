/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./style/input.css", "./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography')
  ],
}

