# Cukiller Frontend

SPA-игра с киберпанк-стилистикой. Агенты регистрируются, проходят модерацию, получают цели и отчитываются о ликвидациях.

---

## Быстрый старт

```bash
bun install
bun run dev      # dev-сервер на http://localhost:5173
bun run check    # typecheck (svelte-check + синхронизация)
bun run lint     # prettier + eslint
bun run format   # автоформатирование
```

**Тесты:**

```bash
bun run test:unit           # vitest (unit + browser component)
bun run test:e2e            # playwright
bun run test:e2e:selenium   # selenium (требуется Docker-бэкенд)
```

---

## Tech Stack

| Слой | Технология |
|------|-----------|
| Фреймворк | Svelte 5 (runes mode) + SvelteKit 2 |
| Язык | TypeScript 6 |
| Сборка | Vite 8 |
| CSS | Tailwind CSS v4 (CSS-based config) |
| i18n | Inlang Paraglide-JS (EN/RU) |
| Адаптер | `@sveltejs/adapter-node` (Docker) |
| Тесты | Vitest + Playwright + Selenium |
| Форматтер | Prettier (tabs, single quotes, trailing comma none) |

---

## Архитектура

### SPA с псевдо-роутингом

Приложение полностью клиентское (`export const ssr = false`).

**Как это работает:**

1. Все SPA-пути (`/agent-id`, `/dossier`, `/rankings` и т.д.) редиректятся на `/` через `reroute` в `src/hooks.ts:4-20`.
2. Единственная точка входа — `src/routes/+page.svelte`. Она содержит все страницы как импортированные компоненты и рендерит их по условию `{#if app.view === '...'}`.
3. `AppProvider.svelte` управляет состоянием `view` и синхронизирует URL через `replaceState('/')`, чтобы адресная строка всегда была `/`.
4. Переходы между страницами — через `app.navigate(viewName)` без перезагрузки.

### Структура проекта

```
src/
├── routes/             # SvelteKit-маршруты (только /)
│   ├── +layout.svelte  # AppProvider + favicon
│   ├── +layout.ts      # SSR off, начальная загрузка сессии
│   ├── +page.svelte    # SPA-роутер со всеми страницами
│   ├── layout.css      # Tailwind v4 + темы + анимации
│   └── admin/          # Компоненты админ-страниц
├── lib/
│   ├── shared/
│   │   ├── api/        # HTTP-клиент к бэкенду
│   │   ├── auth/       # localStorage/cookie для JWT
│   │   ├── config/     # Навигация (BottomNavItem, TopBarConfig)
│   │   ├── model/      # TypeScript-типы + Svelte store
│   │   ├── providers/  # AppContext + AppProvider
│   │   └── ui/         # Переиспользуемые компоненты
│   ├── pages/
│   │   └── profile-flow/  # Модель регистрации (dossier-draft, session flow)
│   └── paraglide/      # Сгенерировано Paraglide (gitignored)
└── app.html            # HTML-шаблон (%paraglide.lang%, %paraglide.dir%)
```

---

## Состояние и контекст (Svelte 5 runes)

### AppContext (`src/lib/shared/providers/app-context.ts`)

Единый контекст приложения, создаваемый в `AppProvider.svelte`. Содержит:

```typescript
type AppContext = {
  view: AppView;                             // текущая SPA-страница
  sessionFlow: SessionFlow | null;           // состояние сессии
  sessionUser: SessionUser | null;           // текущий пользователь
  rankings: RankingEntry[];                  // таблица лидеров
  killTargets: KillTarget[];                 // цели для отчёта
  adminProfileRequests: ProfileRequest[];    // заявки (админка)
  killReports: KillReport[];                 // отчёты о ликвидациях (админка)
  verification: LandingVerification;         // промис для лендинга
  navigate: (target: AppView) => void;
  refreshSession: () => Promise<void>;
  setSessionUser: (user: SessionUser | null) => void;
  loadRankings, loadKillTargets, loadAdminProfileRequests, loadKillReports: () => Promise<void>;
};
```

**Доступ в любом компоненте:** `const app = getAppContext();`

### Session Flow (конечный автомат)

Статусы и переходы (определены в `src/lib/pages/profile-flow/model/profile-flow.ts`):

```
guest → no_profile → pending → approved
                      ↓
                   rejected
```

- **guest** — неавторизован. Показывается лендинг.
- **no_profile** — зарегистрирован, но не создал профиль. Редирект на `/agent-id`.
- **pending** — профиль отправлен на модерацию. Страница `/waiting-clearance`.
- **approved** — как минимум одна заявка принята. Доступ к игре.
- **rejected** — последняя заявка отклонена. Редирект на `/agent-id?mode=edit`.

Определяющая функция: `buildSessionFlow(user, latestProfileRequest, allRequests)`.

### Guarded Views

`AppProvider` автоматически редиректит при переходе на запрещённую страницу:

- `protectedViews` (target-intel, report-kill, rankings...) — доступны только если `sessionFlow.canPlay === true`.
- `adminViews` (admin-moderation, admin-events) — только `sessionUser.is_admin === true`.
- `userViews` (dossier, waiting-clearance) — только если есть `sessionUser`.

### Polling

Каждые 5 секунд `AppProvider` вызывает `refreshSession()`, которая дёргает `getSessionFlow()` и при изменении состояния автоматически переводит пользователя на соответствующую страницу через `profileFlowTarget()`.

---

## API слой (`src/lib/shared/api/backend.ts`)

### `backendJson<T>(path, options, customFetch?)`

Базовый HTTP-клиент:

- Читает токен из `readAccessToken()` (localStorage + cookie).
- Автоматически добавляет `Authorization: Bearer <token>`.
- Автоматически ставит `Content-Type: application/json` (кроме FormData).
- Базовый URL: `env.PUBLIC_BACKEND_URL || http://127.0.0.1:3000`.
- На не-OK ответ кидает `Error` с сообщением из бэкенда.
- 204 возвращает `undefined`.

### Эндпоинты

| Функция | Метод | Path |
|---------|-------|------|
| `registerUser()` | POST | `/auth/register` |
| `loginUser()` | POST | `/auth/login` |
| `getCurrentUser()` | GET | `/auth/me` |
| `createAgentData()` | POST | `/agent-data` |
| `createProfileRequest()` | POST | `/profile-requests` |
| `listProfileRequests()` | GET | `/profile-requests` |
| `deleteProfileRequest()` | DELETE | `/profile-requests/:id` |
| `listAdminProfileRequests()` | GET | `/admin/profile-requests/` |
| `moderateProfileRequest()` | PATCH | `/admin/profile-requests/:id` |
| `listRankings()` | GET | `/stats/rankings` |
| `listKillTargets()` | GET | (из rankings) |
| `reportKill()` | POST | `/kill/` |
| `listKillReports()` | GET | `/kill/` |
| `moderateKillReport()` | POST | `/kill/:id/moderate` |

### Нормализация данных

Backend-типы (`BackendUser`, `BackendAgentData`, `BackendProfileRequest`) маппятся во frontend-типы (`SessionUser`, `AgentProfileData`, `ProfileRequest`) с camelCase + преобразованием enum-ов. См. функции `normalizeUser`, `normalizeAgentData`, `normalizeProfileRequest`.

---

## Авторизация (`src/lib/shared/auth/session.ts`)

- JWT-токен хранится в `localStorage` + cookie (дублируется для SSR-совместимости).
- Auth-payload (email/password/telegram_id/agent_name) сохраняется в `localStorage` для авто-релогина при протухании токена.
- `getSessionFlow()` при невалидном токене пытается перелогиниться через `loginUser()`. Если не получается — чистит токен и возвращает guest-сессию.

---

## Регистрация (3 шага)

1. **Agent ID** (`/agent-id`) — позывной, академ. информация, фото.
2. **Operational Boundaries** (`/operational-boundaries`) — разрешение на физический контакт и объятия.
3. **Dossier Verification** (`/dossier-verification`) — просмотр + отправка на модерацию.

Черновик хранится в localStorage под ключом `dossier-draft`. Каждый шаг открывается только после прохождения предыдущего (`unlockedStep`). Функции-валидаторы: `isAgentIdComplete()`, `isBoundariesComplete()`, `canAccessStep()`.

---

## Интернационализация (Paraglide)

- Конфиг: `project.inlang/settings.json` (base: `en`, locales: `en`, `ru`).
- Сообщения: `messages/{en,ru}.json` (по 190 ключей).
- Использование: `import { m } from '$lib/paraglide/messages.js'`.
- Параметры: `m.home_buffer_ready({ percent: String(value) })`.
- Смена языка: `setLocale('ru')` из `$lib/paraglide/runtime.js`.
- Локализация определяется в server hook через `paraglideMiddleware`.

---

## UI-компоненты (`src/lib/shared/ui/`)

| Компонент | Назначение |
|-----------|-----------|
| `TerminalShell.svelte` | Основной лэйаут (TopBar + контент + BottomNav) |
| `TopBar.svelte` | Верхняя панель: логотип, статус, выбор языка |
| `BottomNav.svelte` | Нижняя навигация (мобильная) |
| `Sidebar.svelte` | Боковая навигация (десктоп) |
| `Icon.svelte` | Обёртка Material Symbols Outlined |
| `AgentPersonalInfo.svelte` | Ввод позывного + загрузка фото |
| `ProfileStatePanel.svelte` | Информационная панель с заголовком, телом, CTA |
| `ProgressBar.svelte` | Сегментированный прогресс-бар |
| `Countdown.svelte` | Таймер MM:SS |
| `NodeConnectivity.svelte` | Индикатор статуса |

---

## Навигация (`src/lib/shared/config/navigation.ts`)

Два набора навигационных элементов:

- **`enlistNav`** — шаги регистрации (agent-id → boundaries → dossier-verification).
- **`gameplayNav`** — основная навигация (dossier, target-intel, rankings, surveillance, missions, loot, perks, rules).

Группировка (BottomNav): `enlist` / `dossier` / `gameplay` отображаются в зависимости от статуса сессии.

---

## Стилизация (Tailwind CSS v4)

- `src/routes/layout.css` — единственный CSS-файл с:
  - `@import 'tailwindcss'` — импорт Tailwind v4.
  - `@theme` — кастомная палитра (киберпанк: зелёный `#76dd71`, пурпурный `#fea9ff`).
  - Шрифты: **Inter** (body), **Space Grotesk** (headlines/labels).
  - Иконки: Material Symbols Outlined.
  - Анимации: scan-sweep, signal-pulse, glitch-burst, live-bar.
- Модификаторы: `.tactical-button`, `.scan-sweep`, `.signal-dot`, `.scanline`, `.segment-bar`.
- Нет tailwind.config — v4 использует CSS-based конфигурацию.

---

## Тестирование

```bash
bun run test              # unit + e2e
bun run test:unit --run   # только unit
bun run test:e2e          # только e2e
```

- **Unit (Vitest)**: `src/**/*.{test,spec}.ts`, `src/**/*.svelte.{test,spec}.{js,ts}`.
- **Browser component (Vitest + Playwright)**: тесты компонентов в браузере.
- **E2E (Playwright)**: `tests/**/*.e2e.ts` с mock-бэкендом `tests/mock-backend.mjs`.
- **Selenium**: `tests/selenium-dossier-approval.e2e.mjs` (требуется Docker-бэкенд).

---

## Production (Docker)

```dockerfile
FROM oven/bun:1 AS build
COPY . .
RUN bun install && bun run build

FROM oven/bun:1
COPY --from=build /app/build /build
EXPOSE 4173
CMD ["bun", "/build/index.js"]
```

Адаптер Node (`@sveltejs/adapter-node`). Сборка в `/build`. Переменная окружения: `PUBLIC_BACKEND_URL`.

---

## Docker-бэкенд для разработки

```bash
docker run -d \
  --name cukiller-backend \
  -p 3000:3000 \
  -e DATABASE_URL=sqlite:///app/cukiller.db \
  ghcr.io/anomalyco/cukiller-backend:latest
```

Стандартный `PUBLIC_BACKEND_URL=http://127.0.0.1:3000` указан по умолчанию.
