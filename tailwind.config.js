/** @type {import('tailwindcss').Config} */
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
  plugins: ["prettier-plugin-tailwindcss"],
}

