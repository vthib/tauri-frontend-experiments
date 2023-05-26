/** @type {import('tailwindcss').Config} */
export default {
  future: {
    removeDeprecatedGapUtilities: true,
    purgeLayersByDefault: true,
    defaultLineHeights: true,
    standardFontWeights: true,
  }
  content: ["./src/**/*.svelte"],
  theme: {
    extend: {},
  },
  plugins: [],
}

