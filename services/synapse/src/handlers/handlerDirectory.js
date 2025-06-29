// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import { MESSAGE_TYPES } from "#protocols/snp/index.js";
import { handlePeer } from "./handlePeer.js";
import { handleHealth } from "./handleHealth.js";
import { handleData } from "./handleData.js";

export const handlerDirectory = {
    [MESSAGE_TYPES.PEER.REQUEST]: handlePeer,
    [MESSAGE_TYPES.PEER.RESPONSE]: handlePeer,
    [MESSAGE_TYPES.HEALTH.CHECK]: handleHealth,
    [MESSAGE_TYPES.DATA.REQUEST]: handleData,
    [MESSAGE_TYPES.DATA.RESPONSE]: handleData,
    // etc.
};

