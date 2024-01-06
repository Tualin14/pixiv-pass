import {useLoaderData, useRouteError} from "@remix-run/react";
import {json, LoaderFunctionArgs} from "@vercel/remix";
import getArtwork from "~/pixiv/artworks/web.server";
import {Image, Link} from "@nextui-org/react";
import setting from "~/setting";

export const loader = async ({params}: LoaderFunctionArgs) => {
    const {pid} = params;

    const body = await getArtwork.pages(pid!);

    return json(body);
};

export default function ArtworksPidPage() {
    const data = useLoaderData<typeof loader>();
    return (
        <div>
            pages:
            {data.map((item, index) => {
                return (<div key={index}>
                        <Image
                            width={128}
                            height={128}
                            alt="thumb_mini"
                            src={setting.pixivMetaPass + item.urls.thumb_mini}
                        />
                        <Link href={setting.pixivMetaPass + item.urls.original} download>download original</Link></div>
                );
            })}
        </div>
    );
}


export function ErrorBoundary() {
    const error: any = useRouteError();

    return (
        <div>
            pages:
            {error.message}
        </div>
    );
}
