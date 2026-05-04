# auto-agents — AI-agent monorepo

A monorepo that bundles openclaw, pi and hermes agents in a docker compose stack.

| Agent | Language | Source | Description |
|-------|----------|--------|-------------|
| **openclaw** | TypeScript / Node 24 | [openclaw/openclaw](https://github.com/openclaw/openclaw) | Personal AI gateway — Telegram, Discord, Slack, WhatsApp, Signal & 20+ more channels |
| **hermes** | Python 3.13 | [NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent) | Self-improving agent with skill creation, memory, and scheduled automations |
| **pi** | TypeScript / Node 22 | [badlogic/pi-mono](https://github.com/badlogic/pi-mono/tree/main/packages/coding-agent) | Minimal extensible terminal coding harness |
---

## Quick start

```bash
# 1. Clone this repo with submodules (fetches openclaw & hermes source)
git clone --recurse-submodules https://github.com/yohannesHL/auto-agents
# -- or, if you already cloned without submodules --
git submodule update --init --recursive

# 2. Copy the env template and fill in at least one LLM API key
cp .env.example .env
$EDITOR .env

# 3. Build images and start all services
docker compose up --build -d

# 4. Follow logs
docker compose logs -f
```

To start only one agent:

```bash
docker compose up --build -d openclaw-gateway
docker compose up --build -d hermes-gateway hermes-dashboard
docker compose up --build -d pi
```

---

## Repository layout

```
auto-agents/
├── agents/
│   ├── openclaw/          # git submodule → github.com/openclaw/openclaw        (TypeScript)
│   ├── hermes/            # git submodule → github.com/NousResearch/hermes-agent (Python)
│   └── pi/                # TypeScript — pi coding-agent (pi-mono package)
│       └── Dockerfile     # added by this monorepo
├── .gitmodules            # submodule declarations for openclaw & hermes
├── docker-compose.yml     # root compose — all agents + stubs
├── .env.example           # all environment variables with defaults
└── README.md              # this file
```

---

## Tracking & telemetry — disabled by default

All external tracking is disabled at the Docker Compose level.
The table below documents every mechanism and how it is suppressed.

| Agent | Mechanism | How disabled |
|-------|-----------|--------------|
| **openclaw** | OpenTelemetry (OTEL) — exports metrics/traces/logs to an OTLP collector | All `OTEL_EXPORTER_OTLP_*` env vars are set to `""` in `docker-compose.yml`. Export only occurs when an endpoint is explicitly configured. |
| **openclaw** | Bonjour/mDNS broadcast — announces the gateway on LAN | `OPENCLAW_DISABLE_BONJOUR=1` in `docker-compose.yml`. |
| **hermes** | Weights & Biases (W&B) experiment tracking | `WANDB_API_KEY` is intentionally absent. W&B only activates when the key is present. |
| **hermes** | Dashboard analytics | The `/api/analytics/*` endpoints read a **local SQLite database only** — no data leaves the container. |
| **pi** | Install-version ping (`https://pi.dev/api/report-install`) | `PI_TELEMETRY=0` in `docker-compose.yml`. Source: `agents/pi/src/core/telemetry.ts`. |
| **go-service** | None | Stub service — no analytics code. |
| **rust-service** | None | Stub service — no analytics code. |

> **To re-enable OTEL for openclaw** (e.g. for your own observability stack),
> set `OTEL_EXPORTER_OTLP_ENDPOINT` in `.env`. No other change is required.

---

## Environment variables

Copy `.env.example` to `.env` and edit it.  Every variable is documented
there with a sensible default.  The sections below are a structured reference.

### Shared (passed to all agents)

| Variable | Default | Description |
|----------|---------|-------------|
| `OPENAI_API_KEY` | _(empty)_ | OpenAI API key |
| `ANTHROPIC_API_KEY` | _(empty)_ | Anthropic Claude API key |
| `GEMINI_API_KEY` | _(empty)_ | Google Gemini API key |
| `GOOGLE_API_KEY` | _(empty)_ | Alias for `GEMINI_API_KEY` |
| `OPENROUTER_API_KEY` | _(empty)_ | OpenRouter — access to 200+ models |

Set at least one key to give the agents a working LLM backend.

---

### openclaw

#### Auth & paths

| Variable | Default | Description |
|----------|---------|-------------|
| `OPENCLAW_GATEWAY_TOKEN` | _(auto-generated)_ | Bearer token for the gateway API. Leave blank to auto-generate on first start; generate manually with `openssl rand -hex 32`. |
| `OPENCLAW_GATEWAY_PASSWORD` | _(empty)_ | Password-based auth alternative. Use TOKEN **or** PASSWORD, not both. |
| `OPENCLAW_TZ` | `UTC` | Timezone (IANA, e.g. `America/New_York`). |
| `OPENCLAW_CONFIG_DIR` | Docker volume `openclaw-config` | Host path mounted at `/home/node/.openclaw`. |
| `OPENCLAW_WORKSPACE_DIR` | Docker volume `openclaw-workspace` | Host path mounted at `/home/node/.openclaw/workspace`. |
| `OPENCLAW_INCLUDE_ROOTS` | _(empty)_ | Colon-separated extra paths for `$include` directives. |
| `OPENCLAW_LOAD_SHELL_ENV` | _(empty)_ | Set `1` to import missing keys from the login shell profile. |
| `OPENCLAW_SHELL_ENV_TIMEOUT_MS` | `15000` | Timeout for shell-env import in ms. |

#### Networking

| Variable | Default | Description |
|----------|---------|-------------|
| `OPENCLAW_GATEWAY_PORT` | `18789` | Host port for the gateway HTTP/WebSocket API. |
| `OPENCLAW_BRIDGE_PORT` | `18790` | Host port for the channel bridge. |
| `OPENCLAW_GATEWAY_BIND` | `lan` | Bind mode: `lan`, `localhost`, or `all`. |
| `OPENCLAW_DISABLE_BONJOUR` | `1` (hardcoded) | Disables mDNS/Bonjour LAN discovery. Set `0` only on host/macvlan networks. |
| `OPENCLAW_ALLOW_INSECURE_PRIVATE_WS` | _(empty)_ | Allow insecure WebSocket on private networks. |

#### OpenTelemetry (OTEL) — all disabled by default

| Variable | Default | Description |
|----------|---------|-------------|
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `""` | Base OTLP endpoint. **Empty = OTEL disabled.** |
| `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` | `""` | Trace-specific OTLP endpoint (overrides base). |
| `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT` | `""` | Metrics-specific OTLP endpoint (overrides base). |
| `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT` | `""` | Logs-specific OTLP endpoint (overrides base). |
| `OTEL_EXPORTER_OTLP_PROTOCOL` | `""` | `http/protobuf` or `grpc`. |
| `OTEL_SERVICE_NAME` | `""` | Service name in observability dashboards. |
| `OTEL_SEMCONV_STABILITY_OPT_IN` | `""` | OTel semantic-conventions stability opt-in. |
| `OPENCLAW_OTEL_PRELOADED` | `""` | Internal flag — leave blank. |

> openclaw only exports OTEL data when `OTEL_EXPORTER_OTLP_ENDPOINT` or one
> of the signal-specific endpoints is non-empty.

#### Messaging channels

| Variable | Description |
|----------|-------------|
| `TELEGRAM_BOT_TOKEN` | Telegram bot token from @BotFather. |
| `DISCORD_BOT_TOKEN` | Discord bot token. |
| `SLACK_BOT_TOKEN` | Slack bot token (`xoxb-…`). |
| `SLACK_APP_TOKEN` | Slack app-level token for Socket Mode (`xapp-…`). |

#### Claude web/browser session (optional)

| Variable | Description |
|----------|-------------|
| `CLAUDE_AI_SESSION_KEY` | Claude.ai session key. |
| `CLAUDE_WEB_SESSION_KEY` | Claude web session key. |
| `CLAUDE_WEB_COOKIE` | Claude web cookie string. |

#### Additional provider keys

| Variable | Provider |
|----------|----------|
| `BRAVE_API_KEY` | Brave Search |
| `PERPLEXITY_API_KEY` | Perplexity |
| `FIRECRAWL_API_KEY` | Firecrawl web crawler |
| `ELEVENLABS_API_KEY` | ElevenLabs TTS |
| `DEEPGRAM_API_KEY` | Deepgram STT |

---

### hermes

#### Container & runtime

| Variable | Default | Description |
|----------|---------|-------------|
| `HERMES_UID` | `10000` | UID the container process runs as. Use `$(id -u)` to match your host user. |
| `HERMES_GID` | `10000` | GID the container process runs as. Use `$(id -g)` to match your host user. |
| `HERMES_DATA_DIR` | Docker volume `hermes-data` | Host path mounted at `/opt/data` (hermes home). |
| `HERMES_DASHBOARD_PORT` | `9119` | Host port for the dashboard web UI (bound to `127.0.0.1` for security). |

#### Messaging channels

| Variable | Description |
|----------|-------------|
| `TELEGRAM_BOT_TOKEN` | Telegram bot token. |
| `TELEGRAM_ALLOWED_USERS` | Comma-separated Telegram user IDs. |
| `TELEGRAM_HOME_CHANNEL` | Default chat ID for cron delivery. |
| `SLACK_BOT_TOKEN` | Slack bot token. |
| `SLACK_APP_TOKEN` | Slack app-level token. |
| `DISCORD_BOT_TOKEN` | Discord bot token. |
| `WHATSAPP_ENABLED` | `false` — set `true` and run `hermes whatsapp` to pair. |
| `WHATSAPP_ALLOWED_USERS` | Comma-separated phone numbers (E.164). |
| `TEAMS_CLIENT_ID` | Azure AD App (client) ID for Microsoft Teams. |
| `TEAMS_CLIENT_SECRET` | Azure AD client secret. |
| `TEAMS_TENANT_ID` | Azure AD tenant ID. |
| `TEAMS_ALLOWED_USERS` | Comma-separated AAD object IDs or UPNs. |
| `TEAMS_PORT` | `3978` — Bot Framework webhook port. |

#### Additional provider & tool keys

| Variable | Description |
|----------|-------------|
| `GLM_API_KEY` | z.ai / ZhipuAI GLM models. |
| `KIMI_API_KEY` | Kimi / Moonshot models. |
| `MINIMAX_API_KEY` | MiniMax models. |
| `ARCEEAI_API_KEY` | Arcee AI Trinity models. |
| `HF_TOKEN` | Hugging Face inference providers. |
| `XIAOMI_API_KEY` | Xiaomi MiMo models. |
| `EXA_API_KEY` | Exa AI web search. |
| `FIRECRAWL_API_KEY` | Firecrawl web crawl/extract. |
| `FAL_KEY` | fal.ai image generation. |
| `HONCHO_API_KEY` | Honcho cross-session user modeling. |
| `BROWSERBASE_API_KEY` | Browserbase cloud browser automation. |
| `BROWSERBASE_PROJECT_ID` | Browserbase project ID. |
| `GROQ_API_KEY` | Groq Whisper STT (free tier). |
| `VOICE_TOOLS_OPENAI_KEY` | OpenAI Whisper + TTS (direct, not via OpenRouter). |
| `GITHUB_TOKEN` | GitHub PAT for higher skill-search rate limits. |

#### RL training (opt-in, disabled by default)

| Variable | Default | Description |
|----------|---------|-------------|
| `WANDB_API_KEY` | _(absent)_ | Weights & Biases experiment tracking. **Intentionally absent** — set only if you use Atropos RL training. |
| `TINKER_API_KEY` | _(empty)_ | Tinker RL training service API key. |
| `RL_API_URL` | `http://localhost:8080` | RL API server URL. |

#### Debug flags

| Variable | Default | Description |
|----------|---------|-------------|
| `WEB_TOOLS_DEBUG` | `false` | Verbose logging for web tools. |
| `VISION_TOOLS_DEBUG` | `false` | Verbose logging for vision tools. |
| `IMAGE_TOOLS_DEBUG` | `false` | Verbose logging for image tools. |

---

### pi

| Variable | Default | Description |
|----------|---------|-------------|
| `PI_TELEMETRY` | `0` (hardcoded) | Anonymous install-version ping to `pi.dev/api/report-install`. **Hard-disabled** (`0`). Set `1` to re-enable. |
| `PI_DATA_DIR` | Docker volume `pi-data` | Host path mounted at `/home/pi/.pi` (sessions, settings, extensions). |
| `ANTHROPIC_API_KEY` | _(shared)_ | Preferred key for pi (Claude models). |
| `OPENAI_API_KEY` | _(shared)_ | OpenAI models. |
| `OPENROUTER_API_KEY` | _(shared)_ | OpenRouter models. |

---

## Building individual images

```bash
# openclaw
docker build -t auto-agents/openclaw:local ./agents/openclaw

# hermes
docker build -t auto-agents/hermes:local ./agents/hermes

# pi
docker build -t auto-agents/pi:local ./agents/pi

```

## Volumes

Named Docker volumes are created automatically when no host paths are set:

| Volume | Used by | Contents |
|--------|---------|----------|
| `openclaw-config` | openclaw | `openclaw.json`, channel configs, credentials |
| `openclaw-workspace` | openclaw | Agent workspace files |
| `hermes-data` | hermes | `config.yaml`, sessions, skill library, SQLite |
| `pi-data` | pi | `.pi/` sessions, settings, extensions |

To persist state on the host instead of in Docker volumes, set the
corresponding `*_DIR` variables in `.env`.


## License

Each bundled agent retains its own upstream licence (MIT for all three).
See `agents/openclaw/LICENSE`, `agents/hermes/LICENSE`, and
`agents/pi/LICENSE`.
