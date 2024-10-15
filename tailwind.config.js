/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        themeRed: '#FF5F5D',
        themeDarkGreen: '#3F7C85',
        themeGreen: '#00CCBF',
        themePaleGreen: '#72F2EB',
        themeGrey: '#747E7E',
      },
      fontFamily: {
        heading: ['ProtestStrike', 'sans-serif']
      }
    },
  },
  plugins: [require('daisyui')],
}

