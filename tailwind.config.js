/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["*.html", "./app/src/**/*.rs",],
    theme: {
        extend: {},
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}