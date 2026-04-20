# Design System: The Ghost Signal

## 1. Overview & Creative North Star
**Creative North Star: "The Classified Protocol"**

This design system is not a standard mobile interface; it is a high-stakes intelligence terminal. It rejects the soft, approachable "friendly" UI of modern consumer apps in favor of **Minimalist Noir Brutalism**. By combining the clinical precision of a terminal with the atmospheric tension of Cyberpunk, we create an experience that feels like handling redacted data.

The system breaks the "template" look through **Intentional Asymmetry**—headers may be offset, and data points are often grouped in non-traditional "dossier" clusters. We move away from the grid to create a sense of urgency and secrecy. Elements should feel like they were "slotted" into a high-tech briefcase rather than rendered on a screen.

---

## 2. Colors & Surface Logic

### The Palette
The core of the system is absolute depth. We use **Deep Black (#000000)** as the void, with **Emerald Green (#007A1B)** and **Rich Purple (#81008F)** serving as high-frequency signal tracers.

- **Primary (Emerald):** Used for "Active Intelligence"—successful hacks, confirmed targets, and safe paths.
- **Secondary (Purple):** Used for "Encryption & Mystery"—locked dossiers, clandestine objectives, and high-level clearance.

### The "No-Line" Rule
**Explicit Instruction:** 1px solid borders are strictly prohibited for sectioning. We do not "box" our data. Boundaries must be defined solely through background color shifts.
*   *Correct:* A `surface-container-low` section sitting directly on a `surface` background.
*   *Incorrect:* A black box with a grey border.

### Surface Hierarchy & Nesting
Treat the UI as physical layers of hardware. 
1.  **Base Layer:** `surface-dim` (#131313).
2.  **Navigation/Backdrops:** `surface-container-lowest` (#0e0e0e).
3.  **Active Gadgets/Cards:** `surface-container` (#1f1f1f) or `surface-container-high` (#2a2a2a).
Nesting a higher-tier container inside a lower-tier one creates a "milled" effect, as if the component was carved out of the device's chassis.

### Glass & Gradient Rule
To prevent a "flat" terminal look, use **Glassmorphism** for floating overlays (e.g., incoming mission alerts). Apply `surface` colors at 60-80% opacity with a `20px` backdrop blur. Use subtle gradients for CTAs, transitioning from `primary` (#76dd71) to `primary-container` (#007a1b) at a 45-degree angle to give the neon a sense of "glow" and energy.

---

## 3. Typography
The typography is the voice of the system. It alternates between "The Machine" and "The Analyst."

*   **Display & Headlines (Space Grotesk):** This is our "Technical Authority" font. Use `display-lg` and `headline-md` for mission titles and status codes. The wide, geometric stance of Space Grotesk should feel like a redacted header on a physical folder.
*   **Body & Titles (Inter):** This is the "Field Report" font. Inter provides the legibility required for dense spy briefings. It is clinical, modern, and sharp.
*   **Labels (Space Grotesk):** Use `label-sm` in all caps for metadata—timestamps, coordinates, and "Classified" stamps.

---

## 4. Elevation & Depth

### The Layering Principle
Depth is achieved through **Tonal Layering**. To "lift" a component, move it up the surface scale. A card should never use a shadow to separate itself from a background of the same color; it must change color to `surface-container-highest`.

### Ambient Shadows
When a "floating gadget" effect is required (e.g., a modal window), use an **Extra-Diffused Tinted Shadow**:
*   **Blur:** 40px - 60px
*   **Opacity:** 8%
*   **Color:** `primary` (#76dd71) or `secondary` (#fea9ff) depending on the mission context. This mimics the ambient glow of a neon screen reflecting off a dark surface.

### The "Ghost Border" Fallback
If contrast is insufficient for accessibility, use a **Ghost Border**: The `outline-variant` token (#3f4a3c) at **15% opacity**. It should be felt, not seen.

---

## 5. Components

### The "Dossier" Card
*   **Shape:** 0px roundedness (Sharp corners only).
*   **Structure:** No dividers. Use `surface-container-low` for the header area and `surface-container` for the body.
*   **Content:** Group data in asymmetrical clusters. Metadata (labels) should be positioned in corners to look like technical specs.

### Buttons (Tactical Triggers)
*   **Primary:** Solid `primary-container` (#007a1b). Text is `on-primary-container`. No rounded corners.
*   **Secondary:** Ghost Border with `secondary` (#fea9ff) text.
*   **State:** On press, the button should "glitch"—briefly invert colors or shift 2px to the left.

### Technical Inputs
*   **Style:** Minimalist underline using the `outline` token (#899483). 
*   **Focus State:** The underline turns `primary` (#76dd71) with a subtle neon glow (`shadow-blur: 4px`).
*   **Error State:** Use `error` (#ffb4ab) with the label changing to "SYSTEM BREACH / UNAUTHORIZED."

### Progress Signalers (Health/Hack Bars)
Instead of a smooth bar, use segmented blocks. Each block is a `surface-container-highest` rectangle. Active segments glow `primary`.

---

## 6. Do’s and Don’ts

### Do:
*   **Do** use extreme vertical white space to separate "intel" blocks.
*   **Do** use "Scanning" animations—a subtle horizontal line moving down the screen over `surface-variant` elements.
*   **Do** embrace the sharp edges; 0px border-radius is the law of this system.
*   **Do** use `secondary` (Purple) for high-clearance or "encrypted" content to differentiate from standard gameplay.

### Don’t:
*   **Don't** use icons with rounded caps. Icons must be sharp, linear, and "tech" oriented.
*   **Don't** use standard "Material" shadows. If it doesn't look like it's glowing or physically layered, it's wrong.
*   **Don't** use centered layouts for everything. Align mission data to the left and metadata to the right to create "Dossier tension."
*   **Don't** use pure white (#FFFFFF). All "white" text should be `on-surface` (#e2e2e2) to maintain the noir atmosphere.