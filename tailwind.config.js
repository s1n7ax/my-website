/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography')
  ],
}

