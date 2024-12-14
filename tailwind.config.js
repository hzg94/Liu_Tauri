/** @type {import('tailwindcss').Config} */
import tailwindcss_primeui from 'tailwindcss-primeui'
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {},
  },
  plugins: [tailwindcss_primeui],
}

