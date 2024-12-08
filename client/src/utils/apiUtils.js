export const replaceParams = (url, params) => {
    Object.keys(params).forEach((key) => {
        url = url.replace(`:${key}`, params[key]);
    });
    return url;
};
