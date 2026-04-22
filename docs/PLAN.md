# Project Plan: Cukiller Web Version

This document outlines the technical plan for the web version of Cukiller, featuring a Rust Axum backend and a SvelteKit frontend.

## 1. Database Schema Redesign

Since the original schema had limitations and we are moving to a single infinite game instance, the new implementation will focus on:
- **Postgres-First Architecture**: Leverage `JSONB` for flexible attributes (perks, objectives), `Window Functions` for real-time rankings, and `Triggers` for automatic rating updates.
- **Simplified Model**: Remove game-specific tables; all agents are part of one global game state.
- **Rating History**: Moving ratings from a single column to a history table to track progression and prevent data loss.
- **State Machine for Kills**: Implement a robust state machine for kill events (Reported $\rightarrow$ Victim Confirmed $\rightarrow$ Admin Approved).
- **Indexing**: Optimized indexes for fast global leaderboard queries.
- **Audit Logs**: Tracking administrative actions for better transparency.

### Future-Proof System Configurations

#### Perks System
To allow adding new perks without schema migrations:
- **`perk_definitions`**: (id, slug, display_name, description, base_duration, config [JSONB])
- **`agent_perks`**: (agent_id, perk_id, activated_at, expires_at, instance_metadata [JSONB])
- Logic is driven by the `slug` in the backend, allowing the behavior to be updated or new perks added solely via data insertion.

#### Loot & Chests System
- **`chest_types`**: (id, slug, rarity, base_drop_rate)
- **`loot_tables`**: (chest_type_id, item_id, chance)
- **`items`**: (id, slug, item_type [GOLD, PERK, COSMETIC], value [JSONB])
- This structure separates the chest "container" from the "loot table", allowing for dynamic drop rate adjustments.

#### "Smite" (Global Events) System
A generalized system for admin-triggered global modifiers:
- **`global_events`**: (id, event_type [ENUM: GLOBAL_TARGET, NIGHT_HUNT, etc.], trigger_id [Admin], target_id [Agent, optional], start_time, end_time, payload [JSONB])
- **Event-Effect Pipeline**: Backend middleware checks active `global_events` and applies modifiers to the game state (e.g., overriding target assignments or changing kill rules) in real-time.

## 2. API Endpoints (Backend - Rust Axum)


### Authentication & User Management
- `POST /api/auth/register` - Register a new agent.
- `POST /api/auth/login` - Authenticate and receive JWT.
- `GET /api/user/me` - Get current authenticated agent's profile.
- `PUT /api/user/me` - Update agent's personal details (name, photo, about).

### Kill Events
- `POST /api/kills` - Report a new kill event (Killer ID, Victim ID).
- `POST /api/kills/{kill_id}/confirm` - Confirm a kill (must be either the killer or the victim).
- `POST /api/kills/{kill_id}/moderate` - Approve or reject a kill event (Admin only).
- `GET /api/kills/my-pending` - Get kill events awaiting current user's confirmation.
- `GET /api/kills` - List all approved kills.

### Rankings & Stats
- `GET /api/rankings` - Get the global leaderboard.
- `GET /api/stats/user/{user_id}` - Get lifetime statistics for an agent.

## 3. Authorization

- **JWT-based Authentication**: Use JSON Web Tokens for stateless session management.
- **Admin Privileges**: Admins possess all `AGENT` capabilities plus full administrative access (moderate kill events, manage users).
- **Middleware**: Implement Axum middleware to verify JWTs and check for the `is_admin` flag on protected endpoints.

## 4. CI/CD Configuration (GitHub Actions)

### Backend Pipeline
1. **CI**: On every push/PR to `main`:
    - Run `cargo fmt --check`.
    - Run `cargo clippy`.
    - Run `cargo test`.
2. **CD**: On merge to `main`:
    - Build Docker image.
    - Push image to registry.
    - Deploy to production server via SSH/Docker Compose.

### Frontend Pipeline
1. **CI**: On every push/PR to `main`:
    - Run `npm run lint`.
    - Run `npm run test` (Vitest).
2. **CD**: On merge to `main`:
    - Build SvelteKit project.
    - Deploy to static hosting or Node.js server.

### Database
- Use a migration tool (e.g., `sqlx-cli` or custom scripts) to apply schema changes during the CD process.

## 5. Frontend Screens

### Existing/Planned Routes
- `/agent-id`: Agent ID assignment and display.
- `/auth/dev-register`: Developer registration for testing.
- `/dossier`: Agent profile and personal statistics.
- `/dossier-verification`: Interface for confirming a kill event.
- `/operational-boundaries`: Game area and rule definitions.
- `/rankings`: Global leaderboard.
- `/reveal-confirmation`: The "reveal" process when a kill is reported.
- `/target-intel`: Information about the current target.

### Missing Screens
- **Admin Dashboard**: A central hub for admins to moderate pending kills, manage users, and perform global game actions (e.g., assigning a specific agent as a global target for all other players for 24h).
- **Landing Page**: Introduction to the game, "Enlistment" call-to-action, and basic rules.
- **Kill Report Form**: A dedicated, high-friction form to report a kill with evidence/details.

## 6. User Flow

1. **Enlistment**:
    - User lands on the Landing Page $\rightarrow$ registers as an Agent $\rightarrow$ sets up their profile in `/dossier`.
2. **The Hunt**:
    - Agent checks `/target-intel` to identify their target.
    - Agent checks `/operational-boundaries` for game constraints.
3. **The Kill**:
    - Agent performs the kill $\rightarrow$ Reports it via the Kill Report Form.
    - Victim receives notification $\rightarrow$ Navigates to `/reveal-confirmation` to confirm the kill.
4. **Verification**:
    - Admin sees the pending event in the Admin Dashboard $\rightarrow$ verifies evidence $\rightarrow$ Approves the kill.
5. **Progression**:
    - Points are added to the Killer's profile $\rightarrow$ Victim is eliminated from the game $\rightarrow$ Rankings are updated in `/rankings`.

## 7. Future Roadmap

### Task-Based Gameplay
- **Tinder-like Target Selection**: Implement a swipe-based interface for agents to choose their next target from a curated list of available targets.
- **Multiple Targets**: Modify the target assignment system to allow an agent to track and hunt multiple targets simultaneously.
- **Non-Kill Objectives**: Introduce various missions (e.g., "shadowing", "intelligence gathering") that reward points without needing a kill.

### Rewards & Progression
- **Treasure Chests**: Implement a "loot" system where agents can find chests in specific locations or after completing tasks.
- **Gold & Perks**: Use `JSONB` in Postgres to store agent inventory (gold) and active perks.
- **Perk Prototypes**:
    - **Manual Target Selection**: Choose the next target manually instead of receiving a random assignment.
    - **Score Multiplier (24h)**: Double MMR changes for 24h (one-sided: doesn't affect the counterparty's loss/gain).
    - **Invulnerability (24h)**: Cannot be revealed for 24h; the revealer is notified of the perk.
    - **Territory Expansion**: Remove location restrictions (e.g., only CU/dorms); the victim is notified.
    - **Target Extension (168h)**: Extend the current target's deadline by one week.
    - **Shadow Strike (24h)**: Ability to kill for 24h if $\le 2$ other people (excluding the agent) are nearby; the victim is notified.
    - **Kill Transfer / Aegis (24h)**: One-time shield for 24h; if killed, the points for the kill are assigned to a random player instead.
- **Geolocation Sharing**: Ability for agents to voluntarily share their real-time coordinates with all other players.
- **Location-Based Verification**: Ability for admins to request specific location snapshots from both killer and victim to verify proximity during a reported kill.

