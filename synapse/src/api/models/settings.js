// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import meNexus from "../config/mysql.js";
import {getGlobalUsersDB, getUserByPublicKeyFromDB} from "#src/orbitdb/globalUsers.js";


export const uploadProfilePicture = async (profilePicturePath, publicKey) => {
    const db = await getGlobalUsersDB();
    const [updatedUser] = await db.query(doc => doc._id === publicKey);
    if (updatedUser) {
        updatedUser.profilePicture = profilePicturePath;
        await db.put(updatedUser);
    }
}

export default {
    uploadProfilePicture,
};