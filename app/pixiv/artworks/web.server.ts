import requests from "~/pixiv/requests.server";
import PixivWebResponse, {ArtworkDetail, ArtworkPages} from "~/pixiv/pixivWebType.server";
import {encrypt} from "crypto14";


const getArtworkDetail = async (pid: string) => {
    const res = await requests("get", `/ajax/illust/${pid}`);

    if (!res.ok) throw new Error("Requests Fail");

    const data: PixivWebResponse<ArtworkDetail> = await res.json();

    if (data.error) throw new Error(data.message);

    const {
        aiType,
        alt,
        createDate,
        description,
        illustId,
        illustTitle,
        illustType,
        likeCount,
        tags,
        userAccount,
        userId,
        userName,
        viewCount
    } = data.body;

    return {
        aiType,
        alt,
        createDate,
        description,
        illustId,
        illustTitle,
        illustType,
        likeCount,
        tags,
        userAccount,
        userId,
        userName,
        viewCount
    };
};

const getArtworkPages = async (pid: string) => {
    const res = await requests("get", `/ajax/illust/${pid}/pages`);

    if (!res.ok) throw new Error("Requests Fail");

    const data: PixivWebResponse<ArtworkPages[]> = await res.json();

    if (data.error) throw new Error(data.message);

    const pages = data.body;

    const secret = process.env.CRYPTO_SECRET!;
    pages.forEach(i => {
        i.urls.thumb_mini = encrypt(secret, i.urls.thumb_mini);
        i.urls.original = encrypt(secret, i.urls.original);
    });

    return pages;
};

const getArtwork = {
    async detail(pid: string) {
        return await getArtworkDetail(pid);
    },

    async pages(pid: string) {
        return await getArtworkPages(pid);
    }
};

export default getArtwork;