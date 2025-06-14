import scrollbar from 'tailwind-scrollbar';

export default {
    content: [
        "./src/**/*.{js,jsx,ts,tsx}", // or wherever your files live
    ],
    plugins: [
        scrollbar(),
    ],
};