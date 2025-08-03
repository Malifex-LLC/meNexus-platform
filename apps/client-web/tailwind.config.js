// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import scrollbar from 'tailwind-scrollbar';

export default {
    content: [
        "./src/**/*.{js,jsx,ts,tsx}",
    ],
    safelist: [
        'text-foreground-alt',
        'text-foreground-message',
        'text-foreground-message-owner'
    ],
    plugins: [
        scrollbar(),
    ],
    theme: {
        extend: {
            colors: {
                background: 'rgb(var(--color-background) / <alpha-value>)',
                surface: 'rgb(var(--color-surface) / <alpha-value>)',
                border: 'rgb(var(--color-border) / <alpha-value>)',
                'header-bg': 'rgb(var(--color-header-bg) / <alpha-value>)',
                foreground: 'rgb(var(--color-foreground) / <alpha-value>)',
                'foreground-alt': 'rgb(var(--color-foreground-alt) / <alpha-value>)',
                'foreground-message': 'rgb(var(--color-foreground-message) / <alpha-value>)',
                'foreground-message-owner': 'rgb(var(--color-foreground-message-owner) / <alpha-value>)',
                owner: 'rgb(var(--color-owner) / <alpha-value>)',
                neutral: 'rgb(var(--color-neutral) / <alpha-value>)',
                brand: 'rgb(var(--color-brand) / <alpha-value>)',
                primary: 'rgb(var(--color-primary) / <alpha-value>)',
                accent: 'rgb(var(--color-accent) / <alpha-value>)',
                secondary: 'rgb(var(--color-secondary) / <alpha-value>)',
                edit: 'rgb(var(--color-edit) / <alpha-value>)',
                'edit-hover': 'rgb(var(--color-edit-hover) / <alpha-value>)',
                'is-editing': 'rgb(var(--color-is-editing) / <alpha-value>)',
                save: 'rgb(var(--color-save) / <alpha-value>)',
                'save-hover': 'rgb(var(--color-save-hover) / <alpha-value>)',
                delete: 'rgb(var(--color-delete) / <alpha-value>)',
                'delete-hover': 'rgb(var(--color-delete-hover) / <alpha-value>)',
            }

        }
    }
};