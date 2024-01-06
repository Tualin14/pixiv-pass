import {Outlet, useLoaderData, useRouteError} from "@remix-run/react";
import {json, LoaderFunctionArgs} from "@vercel/remix";
import getArtwork from "~/pixiv/artworks/web.server";

export const loader = async ({params}: LoaderFunctionArgs) => {
    const {pid} = params;

    const body = await getArtwork.detail(pid!);

    return json(body);
};

export default function ArtworksPage() {
    const data = useLoaderData<typeof loader>();
    return (
        <div>
            <Outlet/>
            detail:
            {data.alt}
        </div>
    );
}


export function ErrorBoundary() {
    const error: any = useRouteError();

    return (
        <div>
            detail:
            {error.message}
        </div>
    );
}
