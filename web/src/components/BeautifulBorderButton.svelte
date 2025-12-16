<script lang="ts">
	export let onClick: () => void = () => {};
	export let variant: 'primary' | 'secondary' | 'link' = 'primary';
	export let className: string = '';
	$: finalClassName = `rounded-sm border-border ${
		variant === 'primary'
			? 'bg-primary text-secondary shadow-xl shadow-primary/50 '
			: variant === 'secondary'
				? 'bg-secondary text-primary'
				: 'bg-transparent text-foreground hover:underline hover:underline-offset-2'
	} px-4 py-1 text-sm text-pretty   ${className}`;
</script>

<button on:click={onClick} class={finalClassName} {...$$restProps}><slot /> </button>
