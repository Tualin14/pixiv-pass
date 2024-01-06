const PIXIV_WEB_URL = "https://www.pixiv.net";

// 封装的Fetch请求函数
const requests = (method = "GET", url: string, data = null, headers = {}) => {
    const options = {
        method,
        headers: {
            "Referer": "https://www.pixiv.net/",
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
            ...headers,
        },
        body: data ? JSON.stringify(data) : null
    };

    const fullUrl = PIXIV_WEB_URL + url;
    return fetch(fullUrl, options);
};

export default requests;