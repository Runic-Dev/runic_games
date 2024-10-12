/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    colors: {
      'white': '#FFFFFF',
      'black': '#000000',
      'theme-red': '#FF5F5D',
      'theme-dark-green': '#3F7C85',
      'theme-green': '#00CCBF',
      'theme-pale-green': '#72F2EB',
      'theme-grey': '#747E7E',
    },
    extend: {
      fontFamily: {
        heading: ['ProtestStrike', 'sans-serif']
      }
    },
  },
  plugins: [require('daisyui')],
}

