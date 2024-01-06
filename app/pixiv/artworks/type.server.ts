export interface ArtworkDetail {
    /**
     * 是否 AI 0: 否 1: 是
     */
    aiType: number;
    /**
     * 作品标识
     */
    alt: string;
    /**
     * 创建日期
     */
    createDate: string;
    /**
     * 作品描述 HTML
     */
    description: string;
    /**
     * 作品 ID
     */
    illustId: string;
    /**
     * 作品标题
     */
    illustTitle: string;
    /**
     * 作品类型 0: 插画 1: 漫画 2: 动图
     */
    illustType: Record<0 | 1 | 2, number>;
    /**
     * 点赞数
     */
    likeCount: number;
    /**
     * 标签组
     */
    tags: {
        /**
         * 标签名称
         */
        tag: string;
    }[];
    /**
     * 作者账号
     */
    userAccount: string;
    /**
     * 作者 用户ID
     */
    userId: string;
    /**
     * 作者 用户名
     */
    userName: string;
    /**
     * 浏览数量
     */
    viewCount: number;
}

export interface ArtworkPages {
    urls: Record<"thumb_mini" | "small" | "regular" | "original", string>;
    width: number;
    height: number;
}