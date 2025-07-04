// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

import fs from 'fs/promises';

export const loadConfig = async (filePath) => {
    const configData = await fs.readFile(filePath, 'utf-8');
    return JSON.parse(configData);
};

export const saveConfig = async (filePath, config) => {
    await fs.writeFile(filePath, JSON.stringify(config, null, 2));
    console.log(`Configuration saved to ${filePath}`);
};
