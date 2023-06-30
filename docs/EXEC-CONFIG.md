# Execution configuration for Aegistrate

Aegistrate requires some keys and data to work with before even being able to run.
Execution configuration (abbreviated to **exec-config**) is used in order to do this.

## What does it contain?

The configuration will contain key-value pairs in either form:
1. TOML file keys. (takes priority)
2. Environment variables (as a **fallback** option, not recommended)

The keys required to run the program is:
- `discord-bot-token` (TOML) or `DISCORD_BOT_TOKEN` (env): The bot token in order to authenticate with the Discord API.
- `mongodb-uri` (TOML) or `MONGODB_URI` (env): The MongoDB URI to connect to the database used in Aegistrate.
- `timeout-seconds` (TOML) or `TIMEOUT_SECONDS` (env): How many seconds to wait for Aegistrate to start up before exiting with an error. If this key is not found, 10 seconds is the default value.

### Examples of what it looks like

#### `~/.config/aegistrate/aegistrate.toml`

```toml
discord_bot_token = "<insert bot token here>"
mongodb_uri = "<insert mongodb uri here>"
timeout_seconds = 10
```

#### `/.env`

```env
DISCORD_BOT_TOKEN="<insert bot token here>"
MONGODB_URI="<insert mongodb uri here>"
TIMEOUT_SECONDS=10
```

## Where is the exec-config?

No, it is not some sort of package you need to `apt-get install`, it is just a term.

Aegistrate will look for its configuration in the following locations only:
- `~/.config/aegistrate/aegistrate.toml`
- `.env` or environment variables (fallback option, **not recommended**)

## If you came here because Aegistrate failed to run...

You can manually put a config file with all the entries required:
1. Create the aegistrate.toml file inside the repository folder.
2. Put the template provided above into the TOML file.
3. Customize and fill out the keys for the Discord bot token and MongoDB URI.