const { fontFamily } = require("tailwindcss/defaultTheme");
const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter var", ...fontFamily.sans],
        fjalla: ["Fjalla One", "sans-serif"],
      },
    },
    colors: {
      // you can either spread `colors` to apply all the colors
      ...colors,
      'verd': '#515932',
      'tgietschen': '#843634',
    },
  },
  safelist: [
    {
      pattern:
        /(bg|text|border)-(transparent|current|white|purple|midnight|metal|tahiti|silver|bermuda|verd|tgietschen)/,
    },
  ],
};
