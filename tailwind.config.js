/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        container: {
            center: true,
            padding: "2rem",
            screens: {
                "2xl": "1440px",
            }
        }
    },

    daisyui: {
        themes: [
            {
                erallie: {


                    "primary": "#2c22e0",


                    "secondary": "#6529a1",


                    "accent": "#f9b928",


                    "neutral": "#1c1934",


                    "base-100": "#131123",


                    "info": "#a5f3fc",


                    "success": "#a3e635",


                    "warning": "#ef4444",


                    "error": "#facc15",
                },
            },
        ],
        base: true,
    },
    plugins: [
        require('@tailwindcss/typography'),
        require('daisyui'),
    ],
}