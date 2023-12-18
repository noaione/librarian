import forms from "@tailwindcss/forms";
import colors from "tailwindcss/colors";
import plugin from "tailwindcss/plugin";

import svgToDataUri from "mini-svg-data-uri";

function approxRemToPx(rem) {
  return Math.round(rem * 16).toString();
}

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,vue,ts}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        gray: colors.neutral,
      },
      backgroundImage: (theme) => ({
        "multiselect-caret": `url("${svgToDataUri(
          `<svg viewBox="0 0 320 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M31.3 192h257.3c17.8 0 26.7 21.5 14.1 34.1L174.1 354.8c-7.8 7.8-20.5 7.8-28.3 0L17.2 226.1C4.6 213.5 13.5 192 31.3 192z"></path></svg>`
        )}")`,
        "multiselect-spinner": `url("${svgToDataUri(
          `<svg viewBox="0 0 512 512" fill="${theme(
            "colors.green.500"
          )}" xmlns="http://www.w3.org/2000/svg"><path d="M456.433 371.72l-27.79-16.045c-7.192-4.152-10.052-13.136-6.487-20.636 25.82-54.328 23.566-118.602-6.768-171.03-30.265-52.529-84.802-86.621-144.76-91.424C262.35 71.922 256 64.953 256 56.649V24.56c0-9.31 7.916-16.609 17.204-15.96 81.795 5.717 156.412 51.902 197.611 123.408 41.301 71.385 43.99 159.096 8.042 232.792-4.082 8.369-14.361 11.575-22.424 6.92z"></path></svg>`
        )}")`,
        "multiselect-remove": `url("${svgToDataUri(
          `<svg viewBox="0 0 320 512" fill="currentColor" xmlns="http://www.w3.org/2000/svg"><path d="M207.6 256l107.72-107.72c6.23-6.23 6.23-16.34 0-22.58l-25.03-25.03c-6.23-6.23-16.34-6.23-22.58 0L160 208.4 52.28 100.68c-6.23-6.23-16.34-6.23-22.58 0L4.68 125.7c-6.23 6.23-6.23 16.34 0 22.58L112.4 256 4.68 363.72c-6.23 6.23-6.23 16.34 0 22.58l25.03 25.03c6.23 6.23 16.34 6.23 22.58 0L160 303.6l107.72 107.72c6.23 6.23 16.34 6.23 22.58 0l25.03-25.03c6.23-6.23 6.23-16.34 0-22.58L207.6 256z"></path></svg>`
        )}")`,
      }),
    },
  },
  plugins: [
    forms({ strategy: "class" }),
    plugin(function ({ addComponents, addUtilities, matchUtilities, theme }) {
      // Variable fonts

      addComponents({
        ".font-variable": {
          "font-variation-settings":
            "'wght' var(--font-variable-wght), 'slnt' var(--font-variable-slnt), 'opsz' var(--font-variable-opsz), 'wdth' var(--font-variable-wdth) 'ital' var(--font-variable-ital)",
          "font-weight": "var(--font-variable-wght)",
          "font-style": "oblique var(--font-variable-slnt)deg",
          "font-optical-sizing": "var(--font-variable-opsz)",
          "font-stretch": "var(--font-variable-wdth)%",
          // default values
          "--font-variable-wght": "400",
          "--font-variable-slnt": "0",
          "--font-variable-opsz": "100",
          "--font-variable-wdth": "100",
          "--font-variable-ital": "0",
        },
      });

      addUtilities({
        ".font-variable-italic": {
          "font-style": "italic",
          "--font-variable-ital": "1",
        },
      });

      matchUtilities(
        {
          "variation-weight": (value) => ({
            "--font-variable-wght": value,
          }),
        },
        {
          values: theme("fontWeight"),
        }
      );

      matchUtilities(
        {
          "variation-slant": (value) => ({
            "--font-variable-slnt": value,
          }),
        },
        {
          // 0 to 20, step 5
          values: {
            ...Object.fromEntries(Array.from({ length: 5 }, (_, i) => [i * 5, (i * 5).toString()])),
          },
        }
      );

      matchUtilities(
        {
          "variation-optical": (value) => ({
            "--font-variable-opsz": value,
          }),
        },
        {
          // similar to fontSize
          values: {
            ...Object.fromEntries(
              Object.entries(theme("fontSize")).map(([key, value]) => [
                key,
                approxRemToPx(Number.parseFloat(value[0].replace("rem", ""))),
              ])
            ),
          },
        }
      );

      matchUtilities(
        {
          "variation-width": (value) => ({
            "--font-variable-wdth": value,
          }),
        },
        {
          values: {
            normal: "100",
            condensed: "75",
            compressed: "50",
            stretch: "125",
            extended: "150",
          },
        }
      );
    }),
  ],
};
