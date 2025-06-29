// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { handlerDirectory } from './handlerDirectory.js'

export const handleSnpMessage = async (libp2p, message) => {
    const handler = handlerDirectory[message.messageType];
    if (handler) return await handler(libp2p, message);
    console.warn(`No handler found for: ${message.messageType}`);
};
