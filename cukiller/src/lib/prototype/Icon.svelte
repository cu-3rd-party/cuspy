<script lang="ts">
	let { name, filled = false, class: className = '' } = $props<{
		name: string;
		filled?: boolean;
		class?: string;
	}>();
</script>

<span
	class={`material-symbols-outlined ${className}`}
	style={`font-variation-settings: 'FILL' ${filled ? 1 : 0};`}
>{name}</span>
