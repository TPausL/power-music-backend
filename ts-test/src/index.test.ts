import { expect, test } from '@jest/globals';
import axios from 'axios';
import * as dotenv from 'dotenv';
import { omit } from 'lodash';
dotenv.config();

test('openapi path', async () => {
    const res = (await axios.get(process.env.BACKEND + '/openapi')).data;
    expect(res).toHaveProperty('paths');
    expect(
        omit(res.paths, [
            "['/playlists'].get.description",
            "['/playlists'].get.responses['200'].description",
            "['/playlists'].get.responses['403'].description",
            "['/user'].get.responses['200'].description",
            "['/user'].get.responses['403'].description",
            "['/user'].get.description",
        ]),
    ).toEqual({
        '/playlists': {
            get: {
                deprecated: false,
                operationId: 'getUserPlaylists',
                responses: {
                    '200': {
                        content: {
                            'application/json': {
                                schema: {
                                    items: {
                                        $ref: '#/components/schemas/Playlist',
                                    },
                                    type: 'array',
                                },
                            },
                        },
                    },
                    '403': {
                        content: {
                            'application/json': {
                                schema: {
                                    $ref: '#/components/schemas/ErrorResponse',
                                },
                            },
                        },
                    },
                },
                tags: ['playlists'],
            },
        },
        '/user': {
            get: {
                deprecated: false,
                operationId: 'getAuthUser',
                responses: {
                    '200': {
                        content: {
                            'application/json': {
                                schema: {
                                    $ref: '#/components/schemas/User',
                                },
                            },
                        },
                    },
                    '403': {
                        content: {
                            'application/json': {
                                schema: {
                                    $ref: '#/components/schemas/ErrorResponse',
                                },
                            },
                        },
                    },
                },
                tags: ['user'],
            },
        },
    });
});

test('openapi components', async () => {
    const res = (await axios.get(process.env.BACKEND + '/openapi')).data;
    expect(res).toHaveProperty('components');

    expect(res.components).toEqual({
        schemas: {
            ErrorResponse: {
                properties: {
                    message: {
                        type: 'string',
                    },
                    details: {
                        type: 'string',
                    }
                },
                required: ['message'],
                type: 'object',
            },
            Playlist: {
                properties: {
                    count: {
                        format: 'int32',
                        type: 'integer',
                    },
                    editable: {
                        type: 'boolean',
                    },
                    hidden: {
                        type: 'boolean',
                    },
                    id: {
                        type: 'string',
                    },
                    link: {
                        type: 'string',
                    },
                    source: {
                        type: 'string',
                    },
                    thumbnail: {
                        type: 'string',
                    },
                    title: {
                        type: 'string',
                    },
                },
                required: [
                    'id',
                    'title',
                    'source',
                    'link',
                    'count',
                    'thumbnail',
                    'editable',
                    'hidden',
                ],
                type: 'object',
            },
            ProviderData: {
                properties: {
                    name: {
                        type: 'string',
                    },
                    user_data: {
                        $ref: '#/components/schemas/ProviderUserData',
                    },
                },
                required: ['name', 'user_data'],
                type: 'object',
            },
            ProviderUserData: {
                properties: {
                    email: {
                        type: 'string',
                    },
                    id: {
                        type: 'string',
                    },
                    image: {
                        type: 'string',
                    },
                    name: {
                        type: 'string',
                    },
                },
                required: ['image', 'name', 'email', 'id'],
                type: 'object',
            },
            User: {
                properties: {
                    email: {
                        type: 'string',
                    },
                    id: {
                        type: 'string',
                    },
                    name: {
                        type: 'string',
                    },
                    providers: {
                        items: {
                            $ref: '#/components/schemas/ProviderData',
                        },
                        type: 'array',
                    },
                },
                required: ['id', 'name', 'email', 'providers'],
                type: 'object',
            },
        },
    });
});
