import { useEffect } from 'react';

const useThemeLoader = (themeName) => {
    useEffect(() => {
        const existingLink = document.getElementById('theme-style');
        const href = `/styles/${themeName}.css`;

        if (existingLink) {
            existingLink.href = href;
        } else {
            const link = document.createElement('link');
            link.id = 'theme-style';
            link.rel = 'stylesheet';
            link.href = href;
            document.head.appendChild(link);
        }
    }, [themeName]);
}

export default useThemeLoader;