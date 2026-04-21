<script lang="ts">
	import { resolve } from '$app/paths';
	import { m } from '$lib/paraglide/messages.js';
</script>

<a href={resolve('/demo/paraglide')}>{m.demo_link_paraglide()}</a>
<a href={resolve('/demo/playwright')}>{m.demo_link_playwright()}</a>
