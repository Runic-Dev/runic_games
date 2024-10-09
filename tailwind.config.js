/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      fontFamily: {
        custom: ['ProtestStrike', 'sans-serif']
      }
    },
  },
  plugins: [],
}

