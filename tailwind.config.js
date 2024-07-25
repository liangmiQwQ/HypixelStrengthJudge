/** @type {import('tailwindcss').Config} */
import colors from "./color.tailwind.js"

export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: colors, // 使用 colors 作为 colors 的扩展
    },
  },
  plugins: [],
}
