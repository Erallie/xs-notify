/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        container: {
            center: true,
            padding: "2rem",
            screens: {
                "2xl": "1440px",
            },
        },
    },

    daisyui: {
        themes: [
            {
                erallie: {
                    primary: "#c998ff",
                    secondary: "#741265",
                    accent: "#fb9ca3",
                    neutral: "#220548",
                    "base-100": "#0f0220",
                    info: "#93c5fd",
                    success: "#a3e635",
                    warning: "#fef08a",
                    error: "#dc2626",
                },
            },
        ],
        base: true,
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
};