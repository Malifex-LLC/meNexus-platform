// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import scrollbar from 'tailwind-scrollbar';

export default {
    content: [
        "./src/**/*.{js,jsx,ts,tsx}", // or wherever your files live
    ],
    plugins: [
        scrollbar(),
    ],
    theme: {
        extend: {
            colors: {
                background: 'rgb(var(--color-background) / <alpha-value>)',
            }
        }
    }
};