/**
 * power-music-backend
 * 0.1.0
 * DO NOT MODIFY - This file has been generated using oazapfts.
 * See https://www.npmjs.com/package/oazapfts
 */
import * as Oazapfts from "oazapfts/lib/runtime";
import * as QS from "oazapfts/lib/runtime/query";
export const defaults: Oazapfts.RequestOpts = {
    baseUrl: "/",
};
const oazapfts = Oazapfts.runtime(defaults);
export const servers = {};
export type Playlist = {
    count: number;
    editable: boolean;
    id: string;
    link: string;
    source: string;
    thumbnail: string;
    title: string;
};
export type ProviderUserData = {
    email: string;
    id: string;
    image: string;
    name: string;
};
export type ProviderData = {
    name: string;
    user_data: ProviderUserData;
};
export type User = {
    email: string;
    id: string;
    name: string;
    providers: ProviderData[];
};
export function getUserPlaylists(opts?: Oazapfts.RequestOpts) {
    return oazapfts.fetchJson<{
        status: 200;
        data: Playlist[];
    } | {
        status: 403;
    }>("/playlists", {
        ...opts
    });
}
export function getAuthUser(opts?: Oazapfts.RequestOpts) {
    return oazapfts.fetchJson<{
        status: 200;
        data: User;
    } | {
        status: 403;
    }>("/user", {
        ...opts
    });
}
