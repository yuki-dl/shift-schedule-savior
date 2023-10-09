/** @type {import('tailwindcss').Config} */

const plugin = require('tailwindcss/plugin')
const backfaceVisibility = plugin(function({addUtilities}) {
  addUtilities({
    '.backface-visible': {
      'backface-visibility': 'visible',
      '-moz-backface-visibility': 'visible',
      '-webkit-backface-visibility': 'visible',
      '-ms-backface-visibility': 'visible'
    },
    '.backface-hidden': {
      'backface-visibility': 'hidden',
      '-moz-backface-visibility': 'hidden',
      '-webkit-backface-visibility': 'hidden',
      '-ms-backface-visibility': 'hidden'
    }
  })
});

module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      height: {
        'screen': [
          '100vh', '100dvh'
        ]
      },
      minHeight: {
        'screen': [
          '100vh', '100dvh'
        ]
      },
      maxHeight: {
        'screen': [
          '100vh', '100dvh'
        ]
      }
    },
  },
  plugins: [backfaceVisibility],
}

