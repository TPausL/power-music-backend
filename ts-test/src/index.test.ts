import * as api from './api';
import { describe, expect, test } from '@jest/globals'

import * as dotenv from 'dotenv'
dotenv.config()


api.defaults.baseUrl = "http://127.0.0.1:8000/";
test("no-auth", async () => {
    expect((await api.getAuthUser()).status).toBe(403)
})


test("user", async () => {
    api.defaults.headers = { Cookie: process.env.COOKIE }
    const res = (await api.getAuthUser()).data;
    expect(res).toHaveProperty("id");
    expect(res).toHaveProperty("name");
    expect(res).toHaveProperty("providers");
    expect(res).toHaveProperty("email");
})

test("playlists", async () => {
    api.defaults.headers = { Cookie: process.env.COOKIE }
    const res = (await api.getUserPlaylists()).data;
    expect(Array.isArray(res)).toBe(true);
    if (res.length > 0) {
        const l = res[0];
        expect(l).toHaveProperty("id");
        expect(l).toHaveProperty("hidden");
        expect(l).toHaveProperty("title");
        expect(l).toHaveProperty("count");
        expect(l.count).toBeGreaterThan(0);
        expect(l).toHaveProperty("link");
        expect(l).toHaveProperty("source");
        expect(l).toHaveProperty("thumbnail");
        expect(l).toHaveProperty("editable")
    }
})

