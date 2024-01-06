import {ArtworkDetail, ArtworkPages} from "~/pixiv/artworks/type.server";

export type {ArtworkDetail, ArtworkPages};

export default interface PixivWebResponse<T> {
    error: boolean;
    message: string;
    body: T;
}
