import {NextUIProvider} from "@nextui-org/react";
import type {LinksFunction} from "@remix-run/node";
import {Links, LiveReload, Meta, Outlet, Scripts, ScrollRestoration, useNavigate,} from "@remix-run/react";

import stylesheet from "~/tailwind.css";

export const links: LinksFunction = () => [
    {rel: "stylesheet", href: stylesheet},
];

export default function App() {
    const navigate = useNavigate();

    return (
        <html lang="en">
        <head>
            <meta charSet="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <Meta/>
            <Links/>
        </head>
        <body>
        <NextUIProvider navigate={navigate}>
            <Outlet/>
            <ScrollRestoration/>
            <Scripts/>
            <LiveReload/>
        </NextUIProvider>
        </body>
        </html>
    );
}
