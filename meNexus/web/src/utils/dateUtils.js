//Format date string
export const formatDate = (isoDateString) => {
    const date = new Date(isoDateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long', // e.g., "December"
        day: 'numeric', // e.g., "9"
        hour: '2-digit',
        minute: '2-digit',
    });
};

