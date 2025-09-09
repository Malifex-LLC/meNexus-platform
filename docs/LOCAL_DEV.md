## 🛠️ Local Quickstart

<details>
<summary><strong>1. Clone the repository</strong></summary>

```bash
git clone https://github.com/Malifex-LLC/meNexus-platform.git
cd meNexus-platform
```

</details>

<details>
<summary><strong>2. Create the MySQL schema & OrbitDB root</strong></summary>

Generate the MySQL schema via the provided SQL script

```bash 
mysql -u root -p < services/synapse/db/init/01-menexus_schema.sql
```

Generate the OrbitDB root (prints the address)

```bash
node services/synapse/src/utils/createGlobalUsersDB.js
```

📋 Copy the OrbitDB address shown in the terminal — you’ll need it for the `.env` file in step 3.

⚠️  You only need to generate the OrbitDB root once per development environment.

</details>

<details>
<summary><strong>3. Configure environment files</strong></summary>

```bash
cp services/synapse/.env.sample services/synapse/.env
```

Edit services/synapse/.env to reflect your environment, making sure to paste the GLOBAL_USERS_DB_ADDR from step 2:

```env
# OrbitDB global users database
GLOBAL_USERS_DB_ADDR=/orbitdb/...   # paste from step 2
```

Also create apps/client-web/.env:

```env
VITE_API_BASE_URL=http://localhost:3001
VITE_WS_BASE_URL=ws://localhost:3001
```

</details>

<details>
<summary><strong>4. Generate the Synapse configuration</strong></summary>

```bash
pnpm run init:synapse-config:local
```
This creates a new synapse-config.json and a local private key (stored in services/synapse/secrets/, which is gitignored).

</details>

<details>
<summary><strong>5. Install dependencies</strong></summary>

```bash
pnpm install             # install all packages in the monorepo
pnpm approve-builds      # approve native builds for @ipshipyard/node-datachannel
```

</details>

<details>
<summary><strong>6. Run Synapse and the web client</strong></summary>

```bash
# Terminal 1 – Start back-end
pnpm run dev:synapse     # Synapse API on http://localhost:3001

# Terminal 2 – Start front-end
pnpm run dev:client      # Vite dev server on http://localhost:5173
```

🌐 Open [http://localhost:5173](http://localhost:5173) in your browser, register a user, and start exploring.

⚠️ Keep both terminals open; the backend and frontend need to run simultaneously.
</details>



🎉 You now have a single-node meNexus instance running entirely on your machine.
